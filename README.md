# spec-executor-testing-03 - word-count

Rust CLI 开发型独立测试仓库。该仓库用于验证 spec-executor 是否能够驱动 Claude/Codex 创建命令行程序，并按验收命令生成稳定文本输出。

## 目录

- `docs/PRD.md`：产品需求。
- `docs/HLD.md`：高层设计。
- `docs/LLD.md`：详细设计。
- `docs/DELIVERY.md`：交付说明模板。
- `tasks/development/`：spec-executor task package。

## 运行

```bash
spec-executor run --spec tasks/development/spec.yaml --workspace ./workspace
```
