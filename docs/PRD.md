# PRD - word-count

## 背景

spec-executor 需要一个小型 Rust CLI 开发任务，用于验证执行器是否能够处理命令行参数、文本输出格式和交付文档保持不变的约束。

## 目标

创建 Rust binary crate，读取命令行参数，将输入文本拆分为词元，并输出每个词元及总数。

## 功能需求

| ID | 需求 |
| --- | --- |
| FR-001 | 使用 `std::env::args().skip(1)` 读取命令行参数。 |
| FR-002 | 对每个参数按空白字符拆分词元。 |
| FR-003 | 对每个词元输出一行 `word: <token>`。 |
| FR-004 | 最后一行输出 `Total: N`，其中 `N` 为词元总数。 |
| FR-005 | 仅使用 Rust 标准库，不引入 `clap` 等外部依赖。 |

## 约束

不得修改 `tasks/development/`、`docs/`、`README.md`、`.gitignore` 或 `spec.yaml`。

## 验收

`cargo build` 成功，且 `cargo run -q -- "hello world hello"` 输出：

```text
word: hello
word: world
word: hello
Total: 3
```
