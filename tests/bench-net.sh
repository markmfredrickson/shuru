#!/bin/bash
# Benchmark NAT vs Proxy networking throughput.
# Usage: ./tests/bench-net.sh [size]
#   size: small (default), large (10MB download)

set -e

SHURU="$(cd "$(dirname "$0")/.." && pwd)/target/release/shuru"

if [ ! -x "$SHURU" ]; then
  echo "Release binary not found. Run: cargo build --release" >&2
  exit 1
fi

if [ "${1:-small}" = "large" ]; then
  # ~10MB file
  URL="https://speed.cloudflare.com/__down?bytes=10000000"
  echo "Downloading ~10MB file"
else
  # ~1KB file
  URL="https://storage.googleapis.com/claude-code-dist-86c565f3-f756-42ad-8dfa-d59b1c096819/claude-code-releases/latest/manifest.json"
  echo "Downloading manifest.json"
fi

echo ""
echo "=== NAT ==="
time "$SHURU" run --net nat -- bash -c "curl -so /dev/null -w 'speed: %{speed_download} bytes/sec, size: %{size_download} bytes, time: %{time_total}s\n' '$URL'" 2>&1

echo ""
echo "=== PROXY ==="
time "$SHURU" run --net proxy -- bash -c "curl -so /dev/null -w 'speed: %{speed_download} bytes/sec, size: %{size_download} bytes, time: %{time_total}s\n' '$URL'" 2>&1
