#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"

if [ -n "${ANDROID_NDK_HOME:-}" ]; then
  export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
fi
if [ -n "${ANDROID_NDK_ROOT:-}" ] && [ -d "$ANDROID_NDK_ROOT" ]; then
  export PATH="$ANDROID_NDK_ROOT:$PATH"
fi
JNI_ARM64="$ROOT_DIR/../app/src/main/jniLibs/arm64-v8a"
JNI_ARMEABI="$ROOT_DIR/../app/src/main/jniLibs/armeabi-v7a"
JNI_X86_64="$ROOT_DIR/../app/src/main/jniLibs/x86_64"
JNI_X86="$ROOT_DIR/../app/src/main/jniLibs/x86"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found, skip rust build"
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  cargo install cargo-ndk
fi

mkdir -p "$JNI_ARM64" "$JNI_ARMEABI" "$JNI_X86_64" "$JNI_X86"

cd "$ROOT_DIR"

cargo ndk --manifest-path Cargo.toml \
  --target arm64-v8a \
  --target armeabi-v7a \
  --target x86_64-linux-android \
  --target i686-linux-android \
  --platform 24 \
  build

find target -type f -name 'libtv_net.so' | while read -r src; do
  case "$src" in
    *aarch64*)
      dest="$JNI_ARM64/libtv_net.so"
      ;;
    *armv7*)
      dest="$JNI_ARMEABI/libtv_net.so"
      ;;
    *x86_64*)
      dest="$JNI_X86_64/libtv_net.so"
      ;;
    *i686*)
      dest="$JNI_X86/libtv_net.so"
      ;;
    *)
      continue
      ;;
  esac
  echo "copy $src -> $dest"
  cp -f "$src" "$dest"
done

echo "tv net .so updated"
