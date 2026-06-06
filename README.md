# spec-executor-testing-03 — word-count

spec-executor 2.0 测试用例：从零构建 CLI 单词计数器，读取 `std::env::args()` 参数，
输出 `word: <token>` 和 `Total: N`。

## 目录

- `tasks/development/spec.yaml` — spec-executor 2.0 入口
- `tasks/development/CLAUDE.md` / `AGENTS.md` / `PROMPT.md` — 任务包
- `docs/DELIVERY.md` — 验收标准说明

## 运行

```
spec-executor run --spec tasks/development/spec.yaml --workspace ./workspace
```
