# tv-util

Rust 工具函式模組，提供編解碼、雜湊等通用工具，替代 Java 中的純工具函式。

## 已完成
- AES/CBC 解密
- MD5 / SHA256
- Hex 轉換
- Base64 編解碼
- ClearKey 密鑰處理

## 構建
```bash
bash build.sh
```

產物為 `libtv_util.so`，自動複製到 `app/src/main/jniLibs/`。
