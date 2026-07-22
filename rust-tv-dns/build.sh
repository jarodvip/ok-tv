#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
JNI_DIR="$ROOT_DIR/../app/src/main/jniLibs"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found, skip rust build"
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

mkdir -p "$JNI_DIR"

pushd "$ROOT_DIR" >/dev/null

cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -P 24 \
  -o "$JNI_DIR" \
  build

popd >/dev/null

echo "tv dns .so updated"
