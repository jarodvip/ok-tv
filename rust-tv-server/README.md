# tv-server

Rust 版本地 HTTP 服务原型，用于 ok-tv P0 迁移。

## 当前范围
- 监听 127.0.0.1:9978..9998
- 提供 `/device` `/tvbus` `/action` 最小路由
- 通过 JNI 暴露 `nativeStart` / `nativeStop`

## 构建
```bash
bash build-tv-server.sh
```

推荐使用仓库根脚本 `build-tv-server.sh` 构建，产物会自动复制到 `app/src/main/jniLibs/...`。

## 当前进展
- 已接入 Rust 本地 HTTP 路由 `/proxy`、`/file`、`/upload`、`/newFolder`、`/delFile`、`/delFolder`
- Rust 服务优先启动，失败时仍回退到 NanoHTTPD

## 下一步
- 完成 `/file` 子路径与 Range/ETag 的 Rust 原生实现
- 完成 `/upload` 文件写入和 zip 解压迁移
- 统一 Rust 与 Java 回调的 content-type/error 约定
