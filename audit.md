# Audit

This file records findings from the full recursive scan that were not fully auto-fixed, or that need human confirmation before changing.

## 2026-07-23

- Android Gradle build environment is incomplete in this workspace.
  - `app/build.gradle` no longer hard-fails when `local.properties` is missing, but Gradle still needs a valid Android SDK path.
  - `/Users/jarod/Dev/ok-tv/local.properties` does not exist.
  - No standard Android SDK install was found under common paths, and `ANDROID_HOME` / `ANDROID_SDK_ROOT` are not set.
  - Action needed: create `local.properties` with a valid `sdk.dir`, or set the Android SDK environment variables, then rerun Gradle.

- `rust-tv-server` cannot be built in this environment from a clean source state.
  - `rust-tv-server/Cargo.toml` depends on `axum-multipart = "0.7"`.
  - The local cargo cache does not contain this crate source, and crates.io index access does not provide a downloadable crate source in this environment.
  - This blocks `bash build-tv-server.sh` and any `cargo ndk` build for `rust-tv-server`.
  - Action needed: ensure `axum-multipart` source is available in the local cargo cache, vendored dependencies, or an internal registry mirror.

- `cargo-ndk v4.1.2` has inconsistent package-selection behavior across modules.
  - `rust-tv-dns` and `rust-tv-server` work with manifest-path-based invocation.
  - `rust-tv-net` failed with `-p .` but worked after switching to `--manifest-path Cargo.toml`.
  - Workaround applied in build scripts, but the underlying tool behavior should be reviewed.
