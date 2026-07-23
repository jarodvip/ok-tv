#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [ -n "${ANDROID_NDK_HOME:-}" ]; then
  export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
fi
if [ -n "${ANDROID_NDK_ROOT:-}" ] && [ -d "$ANDROID_NDK_ROOT" ]; then
  export PATH="$ANDROID_NDK_ROOT:$PATH"
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

cd "$SCRIPT_DIR"
cargo ndk --manifest-path Cargo.toml \
  --target arm64-v8a \
  --target armeabi-v7a \
  --platform 24 \
  build
