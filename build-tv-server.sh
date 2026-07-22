#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUST_DIR="$ROOT_DIR/rust-tv-server"
JNI_ARM64="$ROOT_DIR/app/src/main/jniLibs/arm64-v8a"
JNI_ARMEABI="$ROOT_DIR/app/src/main/jniLibs/armeabi-v7a"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found, skip rust build"
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

mkdir -p "$JNI_ARM64" "$JNI_ARMEABI"

pushd "$RUST_DIR" >/dev/null

cargo ndk -p . \
  --target aarch64-linux-android \
  --target armv7-linux-androideabi \
  --android-platform 24 \
  --build

find target -type f -name 'libtv_server.so' | while read -r src; do
  case "$src" in
    *aarch64*)
      dest="$JNI_ARM64/libtv_server.so"
      ;;
    *armv7*)
      dest="$JNI_ARMEABI/libtv_server.so"
      ;;
    *)
      continue
      ;;
  esac
  echo "copy $src -> $dest"
  cp -f "$src" "$dest"
done

popd >/dev/null

echo "tv server .so updated"
