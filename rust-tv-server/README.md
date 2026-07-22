# tv-server

Rust 版本地 HTTP 服务原型，用于 ok-tv P0 迁移。

## 当前范围
- 监听 127.0.0.1:9978..9998
- 提供 `/device` `/tvbus` `/action` 最小路由
- 通过 JNI 暴露 `nativeStart` / `nativeStop`

## 构建
```bash
./build.sh
```

产物会输出到 `rust-tv-server/target/.../libtv_server.so`，可按模块路径放到 `app/src/main/jniLibs/...`。

## 下一步
- 把现有 `Process` 逻辑逐一切入
- 增加 multipart/form-data 解析
- 增加 Android asset/文件回调
