#!/usr/bin/env python3
"""Send stdin as content: capture.py http|grpc http://127.0.0.1:3000.

HTTP uses Python's standard library. gRPC uses grpcio with byte serializers;
there is no producer event schema or generated protobuf wrapper.
"""
import sys
import urllib.request
from urllib.parse import urlsplit


def receipt(metadata):
    values = {key.lower(): value for key, value in metadata}
    if values.get("deleuze-capture-outcome") != "retained":
        raise RuntimeError(f"capture outcome: {values.get('deleuze-capture-outcome')}")
    seq = int(values["deleuze-seq"])
    digest = values["deleuze-property-hash"]
    if seq < 1 or len(bytes.fromhex(digest)) != 32:
        raise RuntimeError("invalid receipt")
    print(f"seq={seq} property_hash={digest}")


def main():
    if len(sys.argv) != 3 or sys.argv[1] not in ("http", "grpc"):
        raise SystemExit("usage: capture.py http|grpc http://host:port < content")
    mode, base = sys.argv[1:]
    payload = sys.stdin.buffer.read()
    if mode == "http":
        request = urllib.request.Request(
            base.rstrip("/") + "/v1/capture", data=payload, method="POST",
            headers={"Content-Type": "application/octet-stream"},
        )
        with urllib.request.urlopen(request, timeout=10) as response:
            if response.status != 201:
                raise RuntimeError(f"HTTP status {response.status}")
            response.read()
            receipt(response.headers.items())
    else:
        import grpc
        address = urlsplit(base)
        if address.scheme != "http" or not address.netloc:
            raise RuntimeError("this local example requires http://host:port")
        with grpc.insecure_channel(address.netloc) as channel:
            append = channel.unary_unary(
                "/deleuze.v1.Capture/Append",
                request_serializer=lambda content: content,
                response_deserializer=lambda content: content,
            )
            result, call = append.with_call(payload, timeout=10)
            if result != b"":
                raise RuntimeError("expected one empty acknowledgement message")
            receipt(list(call.initial_metadata()) + list(call.trailing_metadata()))


if __name__ == "__main__":
    main()
