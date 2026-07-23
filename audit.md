# Audit

This file records findings from the full recursive scan that were not fully auto-fixed, or that need human confirmation before changing.

## 2026-07-23

- Resolved: `rust-tv-server` now compiles for Android NDK targets and produces `.so` files copied into `app/src/main/jniLibs/`.
- Resolved: missing Android SDK setup by installing commandline tools, accepting licenses, installing `platform-tools`, `platforms;android-37.0`, and `build-tools;37.0.0`, then creating `local.properties`.
- Resolved: `catvod` module compile errors by fixing ambiguous `Proxy` imports, missing `TextUtils` import, `JSONObject.keySet()` compatibility, and missing local copies of `RustNet`/`RustDns` wrappers.
- Resolved: added missing AndroidX Media3 core dependencies (`media3-common`, `media3-session`, `media3-ui`, `media3-exoplayer`, `media3-extractor`, `media3-datasource`, `media3-datasource-okhttp`, `media3-database`) to `app/build.gradle` and `gradle/libs.versions.toml`.

- Still needs human confirmation / external sources:
  - `androidx.media3.ui.PlayerSeekView`, `androidx.media3.ui.danmaku.DanmakuConfig`, `androidx.media3.mpvplayer.MpvPlayer`/`MpvPlayerConfig`, and `androidx.media3.exoplayer.source.preload.DiskPreloadManager` are not present in the configured `androidx.media3:1.3.1` artifacts.
  - 已检查官方上游 `androidx/media` 仓库；它是 AndroidX Media/Media3 的官方源码库，但其中不包含项目当前缺失的自定义扩展类：
    - 已确认官方 `libraries/ui` 下有 `PlayerView`，但未见 `PlayerSeekView`。
    - 其余缺失类 `DanmakuConfig`、`MpvPlayer`/`MpvPlayerConfig`、`DiskPreloadManager` 也不是当前已配置公开 Media3 1.3.1 产物中的类。
  - 因此当前阻塞仍不是“没装 SDK/NDK”，而是缺少第三方扩展库来源：
    - `iawsecondary/media3-extensions` 未找到可访问公开源。
    - `media3-mpv` 只看到应用项目引用，未见可用库源。
    - `shenbengit/Media3Extensions` 可访问，但其公开说明只提供 Media3 FFmpeg 扩展，不是本项目需要的 `PlayerSeekView / DanmakuConfig / MpvPlayer / DiskPreloadManager` 那套类。
  - Action needed: 提供 `media3-extensions` 和 `media3-mpv` 的仓库地址 / JitPack 坐标 / 本地 AAR 路径 / 内部 Maven 仓库；或授权走最小兼容改法，先让 `app` 构建通过。

- Build status:
  - `rust-tv-dns`: builds successfully.
  - `rust-tv-net`: builds successfully.
  - `rust-tv-server`: builds successfully.
  - `catvod`: compiles successfully after fixes.
  - `app`: still blocked on unresolved custom Media3 extension classes in `main`, `leanback`, and `mobile` variants.
