"""Direct regression fixtures for the example clients; Python stdlib + Node."""
import contextlib
import importlib.util
import io
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]


class ClientProtocolTests(unittest.TestCase):
    def test_python_accepts_case_insensitive_http_receipt_headers(self):
        spec = importlib.util.spec_from_file_location("capture_client", ROOT / "examples/clients/capture.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            module.receipt([
                ("Deleuze-Capture-Outcome", "retained"),
                ("Deleuze-Seq", "1"),
                ("Deleuze-Property-Hash", "00" * 32),
            ])
        self.assertEqual(output.getvalue().strip(), "seq=1 property_hash=" + "00" * 32)

    def test_node_rejects_success_status_without_required_trailers(self):
        fixture = r"""
const http2 = require('node:http2');
const server = http2.createServer();
server.on('stream', stream => {
  stream.on('data', () => {});
  stream.on('end', () => {
    stream.respond({':status': 200, 'content-type': 'application/grpc+deleuze',
      'grpc-status': '0', 'deleuze-capture-outcome': 'retained',
      'deleuze-seq': '1', 'deleuze-property-hash': '00'.repeat(32)});
    stream.end(Buffer.alloc(5));
  });
});
server.listen(0, '127.0.0.1', () => console.log(server.address().port));
"""
        with subprocess.Popen(["node", "-e", fixture], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True) as server:
            try:
                port = server.stdout.readline().strip()
                self.assertTrue(port.isdecimal(), "fixture did not bind")
                result = subprocess.run(
                    ["node", str(ROOT / "examples/clients/capture.mjs"), "grpc", f"http://127.0.0.1:{port}"],
                    input=b"content", capture_output=True, timeout=15,
                )
                self.assertNotEqual(result.returncode, 0, "client accepted a framed success without status trailers")
                self.assertIn(b"gRPC status", result.stderr)
            finally:
                server.terminate()
                server.communicate(timeout=5)


if __name__ == "__main__":
    unittest.main()
