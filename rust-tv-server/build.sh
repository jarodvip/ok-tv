#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

cargo ndk -p . \
  --target aarch64-linux-android \
  --target armv7-linux-androideabi \
  --android-platform 24 \
  --build
