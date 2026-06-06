# Delivery Standard — word-count

## Expected Work

Create a Rust binary crate from scratch that reads CLI arguments and
outputs each word plus a total count.

## Deliverable Files

| File | Condition | Verification |
|---|---|---|
| `Cargo.toml` | must exist | existence check |
| `src/main.rs` | must exist; `cargo run -- "hello world hello"` prints `Total: 3` | `cargo build && cargo run -- "hello world hello" \| grep -Fxq "Total: 3"` |

## Expected Output Format

```
$ cargo run -- "hello world hello"
word: hello
word: world
word: hello
Total: 3
```

## Task Package Integrity

`tasks/development/` and `docs/` must remain byte-identical to the
baseline.
