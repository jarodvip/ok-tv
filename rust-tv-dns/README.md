# tv-dns

Rust DNS/DoH 模块，通过 JNI 为 Java 层提供 DNS over HTTPS 解析能力。

## 已完成
- Hosts 规则匹配（含通配符）
- 命中结果缓存
- DoH A 记录查询（Bootstrap IP 支持）
- 通过 JNI 接入 Java 网络层

## 构建
```bash
bash build.sh
```

产物为 `libtv_dns.so`，自动复制到 `app/src/main/jniLibs/`。
