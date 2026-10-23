#!/usr/bin/env node
// Send stdin unchanged with Node's built-in HTTP/HTTP2 clients; no npm packages.
import http from 'node:http';
import http2 from 'node:http2';

function receipt(headers) {
  if (headers['deleuze-capture-outcome'] !== 'retained') {
    throw new Error(`capture outcome: ${headers['deleuze-capture-outcome']}`);
  }
  const seq = headers['deleuze-seq'];
  const hash = headers['deleuze-property-hash'];
  if (!/^[1-9][0-9]*$/.test(seq ?? '') || !/^[0-9a-f]{64}$/.test(hash ?? '')) {
    throw new Error('invalid receipt');
  }
  console.log(`seq=${seq} property_hash=${hash}`);
}

async function sendHttp(base, payload) {
  await new Promise((resolve, reject) => {
    const request = http.request(new URL('/v1/capture', base), {
      method: 'POST',
      headers: { 'content-type': 'application/octet-stream', 'content-length': payload.length },
    }, response => {
      response.resume();
      response.on('error', reject);
      response.on('end', () => {
        try {
          if (response.statusCode !== 201) throw new Error(`HTTP status ${response.statusCode}`);
          receipt(response.headers);
          resolve();
        } catch (error) { reject(error); }
      });
    });
    request.on('error', reject);
    request.setTimeout(10_000, () => request.destroy(new Error('HTTP timeout')));
    request.end(payload);
  });
}

async function sendGrpc(base, payload) {
  const session = http2.connect(base);
  try {
    await new Promise((resolve, reject) => {
      session.once('error', reject);
      const request = session.request({
        ':method': 'POST', ':path': '/deleuze.v1.Capture/Append',
        'content-type': 'application/grpc+deleuze', 'te': 'trailers',
      });
      let headers = {};
      let status;
      const chunks = [];
      request.on('response', values => { headers = values; });
      request.on('trailers', values => { headers = { ...headers, ...values }; status = values['grpc-status']; });
      request.on('data', bytes => chunks.push(bytes));
      request.on('error', reject);
      request.setTimeout(10_000, () => request.destroy(new Error('gRPC timeout')));
      request.on('end', () => {
        try {
          if (headers[':status'] !== 200 || status !== '0') {
            throw new Error(`gRPC status ${status}, HTTP ${headers[':status']}`);
          }
          if (!Buffer.concat(chunks).equals(Buffer.alloc(5))) {
            throw new Error('expected one empty framed acknowledgement');
          }
          receipt(headers);
          resolve();
        } catch (error) { reject(error); }
      });
      const prefix = Buffer.alloc(5);
      prefix.writeUInt32BE(payload.length, 1);
      request.write(prefix);
      request.end(payload);
    });
  } finally {
    session.destroy();
  }
}

try {
  const [mode, base] = process.argv.slice(2);
  if (!['http', 'grpc'].includes(mode) || !base) {
    throw new Error('usage: capture.mjs http|grpc http://host:port < content');
  }
  const parts = [];
  for await (const bytes of process.stdin) parts.push(bytes);
  const payload = Buffer.concat(parts);
  if (mode === 'http') await sendHttp(base, payload);
  else await sendGrpc(base, payload);
} catch (error) {
  console.error(error.message);
  process.exitCode = 1;
}
