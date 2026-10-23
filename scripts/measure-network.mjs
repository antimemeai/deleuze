#!/usr/bin/env node
// Fixed absolute due times; no retries or response-dependent scheduling.
import http from 'node:http';
import http2 from 'node:http2';
import fs from 'node:fs';
import { once } from 'node:events';

const [mode, address, countArg, sizeArg, rateArg, capArg, output] = process.argv.slice(2);
const count = Number(countArg), size = Number(sizeArg), rate = Number(rateArg), cap = Number(capArg);
if (!['http', 'grpc'].includes(mode) || !address || !output ||
    ![count, size, rate, cap].every(Number.isSafeInteger) ||
    count < 1 || count > 1_000_000 || size < 8 || size > 1_048_571 ||
    rate < 1 || rate > 10_000_000 || cap < 1 || cap > 4096 ||
    count * (size + 108) > 1024 ** 3) {
  throw new Error('usage: measure-network.mjs http|grpc URL COUNT BYTES RATE OUTSTANDING NEW_SAMPLES_TSV');
}
const samples = new Array(count);
const origin = process.hrtime.bigint();
const now = () => process.hrtime.bigint() - origin;
const agent = mode === 'http' ? new http.Agent({ keepAlive: true, maxSockets: cap }) : undefined;
const sessions = new Set();
let currentSession, sessionsCreated = 0;
let next = 0, active = 0, issued = 0, notSent = 0, protocolErrors = 0;
let scheduleEnd = 0n;
let resolveDone;
const done = new Promise(resolve => { resolveDone = resolve; });
const counts = { retained: 0, failed: 0, unknown: 0 };
const responseCodes = {}, diagnostics = {};
const bump = key => { diagnostics[key] = (diagnostics[key] ?? 0) + 1; };
const errorCode = (error, fallback = 'UNSPECIFIED') => {
  const code = typeof error === 'string' ? error : error?.code;
  return typeof code === 'string' && /^[A-Z0-9_]+$/.test(code) ? code : fallback;
};

function retire(state) {
  state.accepting = false;
  // Graceful close allows existing owned streams to finish; no stream is replayed.
  if (!state.session.closed && !state.session.destroyed) {
    try { state.session.close(); } catch (error) { bump(`h2_close_error_${errorCode(error)}`); }
  }
}
function sessionForOffer() {
  if (currentSession?.accepting && !currentSession.session.closed && !currentSession.session.destroyed) return currentSession;
  // Only a future offered ID creates a replacement. Events never trigger retries.
  const session = http2.connect(address);
  const state = { session, accepting: true, closed: false, active: 0 };
  sessions.add(state);
  if (sessionsCreated++) bump('h2_sessions_replaced');
  bump('h2_sessions_created');
  session.on('goaway', code => { bump('h2_goaway'); bump(`h2_goaway_code_${code}`); retire(state); });
  session.on('error', error => { bump('h2_session_errors'); bump(`h2_session_error_${errorCode(error)}`); retire(state); });
  session.on('frameError', (_type, code) => { bump(`h2_frame_error_code_${code}`); retire(state); });
  session.on('close', () => { state.closed = true; state.accepting = false; sessions.delete(state); bump('h2_sessions_closed'); });
  currentSession = state;
  return state;
}

function metadata(headers) {
  let outcome = headers['deleuze-capture-outcome'];
  if (!Object.hasOwn(counts, outcome)) outcome = 'unknown';
  const seq = String(headers['deleuze-seq'] ?? '');
  const hash = String(headers['deleuze-property-hash'] ?? '');
  if ((seq && !/^[1-9][0-9]*$/.test(seq)) || (hash && !/^[0-9a-f]{64}$/.test(hash)) ||
      (outcome === 'retained' && (!seq || !hash))) {
    protocolErrors++;
    return { outcome: 'unknown', seq: '', hash: '' };
  }
  return { outcome, seq, hash };
}

function dispatch(id, due) {
  const start = now();
  active++; // Reserve the outstanding slot, even during local construction.
  let finished = false, handedOff = false, state;
  function accept() { handedOff = true; issued++; if (state) state.active++; }
  function finish(result, code, reason = 'RESPONSE_OR_RECEIPT') {
    if (finished) return;
    finished = true;
    if (result.outcome === 'not_sent') { notSent++; bump(`not_sent_${reason}`); }
    else {
      counts[result.outcome]++;
      if (result.outcome === 'unknown') bump(`unknown_${reason}`);
    }
    if (code !== undefined) {
      const key = `${mode}_status_${code}`;
      responseCodes[key] = (responseCodes[key] ?? 0) + 1;
    }
    const end = now();
    samples[id] = { id, due, start, end, ...result };
    active--;
    if (state && handedOff) state.active--;
    if (next === count && active === 0) resolveDone();
  }
  const unknown = error => finish({ outcome: 'unknown', seq: '', hash: '' }, undefined, errorCode(error, 'MISSING_REPLY'));
  try {
    const payload = Buffer.alloc(size, 0xa5);
    payload.writeBigUInt64LE(BigInt(id));
    if (mode === 'http') {
      const request = http.request(new URL('/v1/capture', address), {
        method: 'POST', agent,
        headers: { 'content-type': 'application/octet-stream', 'content-length': size },
      }, response => {
        response.on('error', unknown);
        response.on('aborted', () => unknown('RESPONSE_ABORTED'));
        response.resume();
        response.on('end', () => {
          const result = metadata(response.headers);
          if (result.outcome === 'retained' && response.statusCode !== 201) protocolErrors++;
          finish(result, response.statusCode);
        });
      });
      accept(); // ClientRequest ownership exists; subsequent errors are unknown.
      const timer = setTimeout(() => { unknown('CLIENT_TIMEOUT'); request.destroy(); }, 30_000);
      request.once('close', () => { clearTimeout(timer); if (!finished) unknown('REQUEST_CLOSED'); });
      request.on('error', unknown);
      request.end(payload);
    } else {
      state = sessionForOffer();
      const request = state.session.request({
        ':method': 'POST', ':path': '/deleuze.v1.Capture/Append',
        'content-type': 'application/grpc+deleuze', 'te': 'trailers',
      });
      // Even a stream awaiting connection/ID now belongs to Node. Failure after
      // this point does not establish whether the server received its content.
      accept();
      let headers = {}, status;
      const chunks = [];
      request.on('response', values => { headers = values; });
      request.on('trailers', values => { headers = { ...headers, ...values }; status = values['grpc-status']; });
      request.on('data', bytes => chunks.push(bytes));
      request.on('error', error => { bump(`h2_stream_error_${errorCode(error)}`); unknown(error); });
      const timer = setTimeout(() => { unknown('CLIENT_TIMEOUT'); request.close(); }, 30_000);
      request.once('close', () => {
        clearTimeout(timer);
        if (request.rstCode) bump(`h2_stream_reset_code_${request.rstCode}`);
        if (!finished) unknown('STREAM_CLOSED');
      });
      request.on('end', () => {
        const result = metadata(headers);
        if (result.outcome === 'retained' &&
            (headers[':status'] !== 200 || status !== '0' || !Buffer.concat(chunks).equals(Buffer.alloc(5)))) {
          protocolErrors++;
        }
        finish(result, status ?? 'missing');
      });
      const prefix = Buffer.alloc(5);
      prefix.writeUInt32BE(size, 1);
      request.write(prefix);
      request.end(payload);
    }
  } catch (error) {
    if (!handedOff) {
      if (state) retire(state);
      finish({ outcome: 'not_sent', seq: '', hash: '' }, undefined, errorCode(error));
    } else {
      if (state && (state.session.closed || state.session.destroyed)) retire(state);
      unknown(error);
    }
  }
}

function pump() {
  let batch = 0;
  while (next < count && batch++ < 1024) {
    const due = BigInt(next) * 1_000_000_000n / BigInt(rate);
    const actual = now();
    if (actual < due) break;
    const id = next++;
    if (active === cap) {
      notSent++;
      bump('not_sent_outstanding_cap');
      samples[id] = { id, due, start: actual, end: actual, outcome: 'not_sent', seq: '', hash: '' };
    } else dispatch(id, due);
  }
  if (next === count) {
    scheduleEnd = now();
    if (active === 0) resolveDone();
  } else {
    const delayNs = BigInt(next) * 1_000_000_000n / BigInt(rate) - now();
    // Timers may batch due events; measured generator lag exposes this cost.
    if (delayNs > 1_000_000n) setTimeout(pump, Number(delayNs / 1_000_000n));
    else setImmediate(pump);
  }
}

pump();
await done;
const end = now();
// Parent closes/drains the server now; artifact export stays outside its timer.
console.log('WORKLOAD_COMPLETE');
agent?.destroy();
await Promise.all([...sessions].map(state => new Promise(resolve => {
  if (state.closed) return resolve();
  state.session.once('close', resolve);
  state.session.destroy();
})));
if (issued + notSent !== count || counts.retained + counts.failed + counts.unknown !== issued || active !== 0) {
  throw new Error('workload population mismatch');
}
const stream = fs.createWriteStream(output, { flags: 'wx' });
stream.on('error', error => { console.error(error); process.exitCode = 1; });
if (!stream.write('id\tdue_ns\tstart_ns\tend_ns\toutcome\tseq\thash\n')) await once(stream, 'drain');
for (const sample of samples) {
  if (!stream.write(`${sample.id}\t${sample.due}\t${sample.start}\t${sample.end}\t${sample.outcome}\t${sample.seq}\t${sample.hash}\n`)) {
    await once(stream, 'drain');
  }
}
stream.end();
await once(stream, 'finish');
for (const [key, value] of Object.entries({
  offered: count, issued, not_sent: notSent, completed: issued, pending: active,
  client_retained: counts.retained, client_failed: counts.failed, client_unknown: counts.unknown,
  protocol_errors: protocolErrors, schedule_ns: scheduleEnd.toString(), client_ns: end.toString(),
  ...responseCodes, ...diagnostics,
})) console.log(`${key}\t${value}`);
if (protocolErrors) process.exitCode = 1;
