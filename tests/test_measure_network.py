"""Direct load-client lifecycle regressions, using only Python/Node stdlib."""
import contextlib
import csv
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CLIENT = ROOT / "scripts/measure-network.mjs"
SERVER = r"""
const http2 = require('node:http2');
const fs = require('node:fs');
const [recordFile, goawayArg, resetArg] = process.argv.slice(1);
const goaway = Number(goawayArg);
let sessions = 0, sent = false;
const server = http2.createServer();
server.on('session', session => {
  session.fixtureNumber = ++sessions;
  session.on('error', () => {});
});
server.on('stream', stream => {
  const session = stream.session;
  const chunks = [];
  stream.on('error', () => {});
  stream.on('data', chunk => chunks.push(chunk));
  stream.on('end', () => {
    const wire = Buffer.concat(chunks);
    if (wire.length < 13) throw new Error('missing test payload');
    if (wire[0] !== 0 || wire.readUInt32BE(1) + 5 !== wire.length || wire.subarray(13).some(byte => byte !== 0xa5)) throw new Error('invalid test payload framing/pattern');
    const id = wire.readBigUInt64LE(5);
    fs.appendFileSync(recordFile, `${id}\t${session.fixtureNumber}\n`);
    if (resetArg === 'reset' && id === 0n) { stream.close(http2.constants.NGHTTP2_CANCEL); return; }
    stream.respond({ ':status': 200, 'content-type': 'application/grpc+deleuze',
      'deleuze-capture-outcome': 'retained', 'deleuze-seq': String(id + 1n),
      'deleuze-property-hash': '00'.repeat(32) }, { waitForTrailers: true });
    stream.on('wantTrailers', () => stream.sendTrailers({ 'grpc-status': '0' }));
    if (goaway >= 0 && !sent) {
      sent = true;
      stream.once('close', () => {
        if (!session.destroyed) { session.goaway(goaway); session.close(); }
      });
    }
    stream.end(Buffer.alloc(5));
  });
});
server.listen(0, '127.0.0.1', () => console.log(server.address().port));
"""


class MeasureNetworkTests(unittest.TestCase):
    @contextlib.contextmanager
    def server(self, directory, goaway=-1, reset=False):
        records = directory / "observed.tsv"
        with subprocess.Popen(
            ["node", "-e", SERVER, str(records), str(goaway), "reset" if reset else ""],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        ) as process:
            try:
                port = process.stdout.readline().strip()
                self.assertTrue(port.isdecimal(), "HTTP/2 fixture did not bind")
                yield f"http://127.0.0.1:{port}", records
            finally:
                process.terminate()
                process.communicate(timeout=5)

    def load(self, directory, address, preload=None):
        samples = directory / "samples.tsv"
        command = ["node"]
        if preload:
            command += ["-r", str(preload)]
        result = subprocess.run(
            command + [str(CLIENT), "grpc", address, "4", "16", "4", "1", str(samples)],
            capture_output=True, text=True, timeout=12,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        summary = dict(line.split("\t", 1) for line in result.stdout.splitlines() if "\t" in line)
        with samples.open() as stream:
            rows = list(csv.DictReader(stream, delimiter="\t"))
        self.assertEqual([int(row["id"]) for row in rows], list(range(4)))
        self.assertEqual(int(summary["offered"]), int(summary["issued"]) + int(summary["not_sent"]))
        self.assertEqual(int(summary["completed"]), int(summary["issued"]))
        self.assertEqual(int(summary["pending"]), 0)
        self.assertEqual(int(summary["client_retained"]) + int(summary["client_failed"]) + int(summary["client_unknown"]), int(summary["issued"]))
        self.assertEqual(int(summary["h2_sessions_created"]), int(summary["h2_sessions_closed"]), "all created sessions must be cleaned up")
        for row in rows:
            self.assertLessEqual(int(row["due_ns"]), int(row["start_ns"]))
            self.assertLessEqual(int(row["start_ns"]), int(row["end_ns"]))
        return summary, rows

    def test_graceful_goaway_replaces_session_for_future_offers_without_retries(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            with self.server(directory, goaway=0) as (address, records):
                summary, rows = self.load(directory, address)
            observed = [tuple(map(int, line.split("\t"))) for line in records.read_text().splitlines()]
            self.assertEqual([row[0] for row in observed], list(range(4)), "later offers disappeared after GOAWAY")
            self.assertGreater(len({row[1] for row in observed}), 1)
            self.assertEqual([row["outcome"] for row in rows], ["retained"] * 4)
            self.assertEqual(int(summary["issued"]), 4)
            self.assertGreaterEqual(int(summary["h2_sessions_created"]), 2)
            self.assertGreaterEqual(int(summary["h2_goaway_code_0"]), 1)

    def test_pre_stream_invalid_session_is_not_sent_not_unknown(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            preload = directory / "prestream.cjs"
            preload.write_text(r"""
const http2 = require('node:http2');
const connect = http2.connect;
let injected = false;
http2.connect = function (...args) {
  const session = connect.apply(this, args);
  const request = session.request;
  session.request = function (...args) {
    if (!injected) {
      injected = true;
      const error = new Error('test rejection before stream handoff');
      error.code = 'ERR_HTTP2_INVALID_SESSION';
      throw error;
    }
    return request.apply(this, args);
  };
  return session;
};
""")
            with self.server(directory) as (address, records):
                summary, rows = self.load(directory, address, preload)
            self.assertEqual(rows[0]["outcome"], "not_sent")
            self.assertEqual([int(line.split("\t")[0]) for line in records.read_text().splitlines()], [1, 2, 3], "rejected workload ID must not be replayed")
            self.assertEqual(int(summary["issued"]), 3)
            self.assertEqual(int(summary["not_sent"]), 1)
            self.assertEqual(int(summary["client_unknown"]), 0)
            self.assertEqual(int(summary["not_sent_ERR_HTTP2_INVALID_SESSION"]), 1)

    def test_nonzero_goaway_reports_runtime_code_and_future_offers_continue(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            with self.server(directory, goaway=11) as (address, records):
                summary, _ = self.load(directory, address)
            self.assertEqual([int(line.split("\t")[0]) for line in records.read_text().splitlines()], list(range(4)))
            self.assertGreaterEqual(int(summary["h2_goaway_code_11"]), 1)
            self.assertGreaterEqual(int(summary["h2_sessions_created"]), 2)
            self.assertEqual(int(summary["issued"]), 4)
            self.assertEqual(int(summary["not_sent"]), 0)
            self.assertGreaterEqual(int(summary["client_retained"]), 3)

    def test_reset_after_stream_handoff_is_unknown_and_never_replayed(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            with self.server(directory, reset=True) as (address, records):
                summary, rows = self.load(directory, address)
            self.assertEqual([int(line.split("\t")[0]) for line in records.read_text().splitlines()], list(range(4)))
            self.assertEqual(rows[0]["outcome"], "unknown")
            self.assertEqual(int(summary["issued"]), 4)
            self.assertEqual(int(summary["not_sent"]), 0)
            self.assertEqual(int(summary["client_unknown"]), 1)
            self.assertGreaterEqual(int(summary["h2_stream_reset_code_8"]), 1)
            self.assertEqual(int(summary["grpc_status_0"]), 3)


if __name__ == "__main__":
    unittest.main()
