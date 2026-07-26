# tv-server

Rust 本地 HTTP 服务，通过 JNI 替代 NanoHTTPD，提供远程控制、文件传输等能力。

## 已完成
- 监听 127.0.0.1:9978..9998（自动端口检测）
- 路由：`/device`、`/tvbus`、`/action`、`/proxy`、`/file`、`/upload`、`/newFolder`、`/delFile`、`/delFolder`
- 通过 JNI 暴露 `nativeStart` / `nativeStop`
- Rust 服务优先启动，失败时回退到 NanoHTTPD

## 构建
```bash
bash build.sh
```

产物为 `libtv_server.so`，自动复制到 `app/src/main/jniLibs/`。
