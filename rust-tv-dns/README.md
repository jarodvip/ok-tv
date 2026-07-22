# tv-dns

Rust DNS/DoH 辅助模块，用于 ok-tv 网络层渐进式重构。

## 当前范围
- hosts 规则匹配
- 命中结果缓存
- DoH A 记录查询

## 下一步
- 接入 OkDns
- 增加更多 DoH 后端与超时控制
- 增加 IPv6 / 多地址回退策略
