# LLD - word-count

## 文件设计

| 文件 | 设计要求 |
| --- | --- |
| `Cargo.toml` | 包名建议为 `word-count`，版本为 `0.1.0`，edition 使用稳定 Rust edition。 |
| `src/main.rs` | 实现参数读取、词元拆分和输出。 |

## 算法设计

1. 调用 `std::env::args().skip(1)` 忽略程序名。
2. 对每个参数执行 `split_whitespace()`。
3. 将词元按原始顺序输出为 `word: <token>`。
4. 维护计数器并在末尾输出 `Total: N`。

## 验证设计

| 命令 | 预期 |
| --- | --- |
| `cargo build` | 编译成功。 |
| `cargo run -q -- "hello world hello"` | 输出三行词元和 `Total: 3`。 |
