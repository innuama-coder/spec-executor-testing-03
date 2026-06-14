# CLAUDE.md - word-count

## 工作协议

你正在执行 `spec-executor-testing-03` 的 Rust CLI 开发任务。请先阅读 `docs/PRD.md`、`docs/HLD.md` 和 `docs/LLD.md`。

## 任务目标

创建 Rust binary crate，读取命令行参数并输出每个词元与总数。

## 交付要求

- 创建 `Cargo.toml`。
- 创建 `src/main.rs`。
- 使用 `std::env::args().skip(1)` 读取参数。
- 使用空白字符拆分词元。
- 输出格式必须与 PRD 一致。
- 不修改任务包和工作文档。

## 验证

运行 `cargo build` 和 `cargo run -q -- "hello world hello"`，并在最终回复中报告结果。
