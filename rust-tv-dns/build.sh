#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [ -n "${ANDROID_NDK_HOME:-}" ]; then
  export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
fi
if [ -n "${ANDROID_NDK_ROOT:-}" ] && [ -d "$ANDROID_NDK_ROOT" ]; then
  export PATH="$ANDROID_NDK_ROOT:$PATH"
fi
JNI_DIR="$ROOT_DIR/../app/src/main/jniLibs"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found, skip rust build"
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

mkdir -p "$JNI_DIR"

cd "$ROOT_DIR"

cargo ndk --manifest-path Cargo.toml \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64-linux-android \
  -t i686-linux-android \
  -P 24 \
  -o "$JNI_DIR" \
  build

echo "tv dns .so updated"
