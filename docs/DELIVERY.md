# DELIVERY - word-count

## 验收用途

本文档用于人工复核 `word-count` 任务是否完成。执行者不得修改本文档，最终回复必须提供与本文档一致的验收证据。

## 交付物

| 交付物 | 验收要点 |
| --- | --- |
| `Cargo.toml` | 位于仓库根目录，声明 Rust binary crate。 |
| `src/main.rs` | 使用标准库读取命令行参数、拆分词元并输出统计结果。 |

## 验收命令

```bash
cargo build
cargo run -q -- "hello world hello"
```

## 通过标准

示例命令必须输出：

```text
word: hello
word: world
word: hello
Total: 3
```

`tasks/development/`、`docs/`、`README.md`、`.gitignore` 和 `spec.yaml` 不得被修改。
