# tv-net

Rust 网络规则引擎，通过 JNI 为 Java 层提供代理、Hosts、广告拦截和请求头注入能力。

## 已完成
- 代理规则匹配（HTTP/HTTPS/SOCKS4/SOCKS5）
- Hosts / 通配符匹配
- 广告拦截（`ads` 黑名单）
- 请求头注入（CORS 等）
- 通过 JNI 接入 `RustNet`，配置热更新时重置

## 构建
```bash
bash build.sh
```

产物为 `libtv_net.so`，自动复制到 `app/src/main/jniLibs/`。
