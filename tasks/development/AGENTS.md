# AGENTS.md — word-count (codex executor)

> Loaded by spec-executor 2.0 when `executor: codex`. Copied to
> the worktree root during StartingExecutor. PROMPT.md is sent as
> the first user message via `send_input`.

## Task

Create a Rust binary crate from scratch. The repository has no Rust
code as baseline — produce `Cargo.toml` and `src/main.rs` that reads
command-line arguments and outputs:

```
word: <token>
word: <token>
Total: N
```

## Constraints

- Create `Cargo.toml` and `src/main.rs` at the repo root.
- Standard library only. No external dependencies such as `clap`.
- Do not modify `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`.
- No `rust-toolchain.toml`.
- Output format must match exactly. The verifier runs
  `cargo run -- "hello world hello" | grep -Fxq "Total: 3"`.

## Self-Verification (mandatory)

```
cargo build
cargo run -- "hello world hello"
```

## Definition of Done

1. `Cargo.toml` exists.
2. `src/main.rs` exists; `cargo build` exits 0.
3. `cargo run -- "hello world hello"` prints `Total: 3` on its
   last line.
4. `tasks/development/` is byte-identical to the baseline.

## Out of Scope

- Refactoring, comments, tests, benchmarks, examples.
- Adding CI, README, or `LICENSE` files.
- External dependencies.
