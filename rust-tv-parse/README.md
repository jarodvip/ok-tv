# tv-parse

Rust URL 解析引擎，處理影片 URL 的規則匹配與重寫。

## 已完成
- URL 解析規則匹配
- 透過 JNI 接入 `RustParse`

## 構建
```bash
bash build.sh
```

產物為 `libtv_parse.so`，自動複製到 `app/src/main/jniLibs/`。
