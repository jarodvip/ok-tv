# Audit

This file records issues found during the full recursive scan that could not be safely auto-fixed or require human confirmation before changing.

## 2026-07-23

- Android build environment is not fully configured in this workspace.
  - `app/build.gradle` currently loads `local.properties` without guarding against a missing file.
  - After adding a guard, `./gradlew tasks --all` still fails because no Android SDK path is available.
  - `/Users/jarod/Dev/ok-tv/local.properties` does not exist.
  - No standard Android SDK install was detected under common paths, and `ANDROID_HOME` / `ANDROID_SDK_ROOT` are not set.
  - Action needed: create or configure `local.properties` with a valid `sdk.dir` or set the Android SDK environment variables, then re-run Gradle.
