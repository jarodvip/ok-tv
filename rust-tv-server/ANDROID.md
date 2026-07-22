# Android 集成说明

## 当前状态
- Rust 服务骨架已加入项目：`rust-tv-server/`
- JNI 包装类已加入：`app/src/main/java/com/fongmi/android/tv/server/RustServer.java`
- Java 回调入口已加入：`app/src/main/java/com/fongmi/android/tv/server/RustServerCallback.java`
- 本地服务启动已支持优先使用 Rust：`app/src/main/java/com/fongmi/android/tv/server/Server.java`

## 构建 .so
```bash
./build-tv-server.sh
```

产物会尝试复制到：
- `app/src/main/jniLibs/arm64-v8a/libtv_server.so`
- `app/src/main/jniLibs/armeabi-v7a/libtv_server.so`

## 验证
1. 先确认 `app/src/main/jniLibs/**/libtv_server.so` 是真实编译产物
2. 运行 App 后查看日志：
   - `LocalServer`
   - `RustServer`
   - `DeviceHandler`
   - `TvbusHandler`
   - `ActionHandler`
3. 访问本地服务：
   - `http://127.0.0.1:{port}/device`
   - `http://127.0.0.1:{port}/action?do=control&type=play`

## 注意
- 当前仍保留 NanoHTTPD fallback
- Rust 失败时会自动回退到旧实现
- Release 包已加入 JNI 相关 ProGuard 规则
