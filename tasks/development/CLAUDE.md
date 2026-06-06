# CLAUDE.md — word-count (claude executor)

> Loaded by spec-executor 2.0. Copied from `tasks/development/CLAUDE.md`
> to the worktree root during StartingExecutor. PROMPT.md is sent
> as the first user message via `send_input`.

## Mission

Create a Rust binary crate from scratch. The repository has no Rust
code as baseline — you must produce `Cargo.toml` and `src/main.rs`
that reads command-line arguments via `std::env::args()`, splits each
argument by whitespace, and outputs:

- `word: <token>` for every token
- `Total: N` on the last line, where N is the total token count

Example — `cargo run -- "hello world"` must produce:
```
word: hello
word: world
Total: 2
```

Empty input (`cargo run` with no arguments) must produce `Total: 0`
only (no `word:` lines).

## Working Agreement

- **Standard library only.** No external dependencies, no `clap`.
- **Do not modify** `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`, or any file not related to the crate
  you are creating.
- **No `rust-toolchain.toml`.** Build with the resolved stable
  toolchain.
- **Output format must match exactly.** The verifier runs
  `cargo run -- "hello world hello" | grep -Fxq "Total: 3"`.

## Self-Verification

Before declaring done, run from the worktree root:
```
cargo build
cargo run -- "hello world hello"
```

Confirm the last output line is `Total: 3`.

## Definition of Done

1. `Cargo.toml` exists and defines a binary crate.
2. `src/main.rs` exists; `cargo build` exits 0.
3. `cargo run -- "hello world hello"` prints `Total: 3` on its
   last line.
4. `tasks/development/` is byte-identical to the baseline
   (verified by the spec).

## Out of Scope

- External dependencies.
- Integration tests, benchmarks, or examples.
- Adding a `rust-toolchain.toml`.
- Modifying any file outside the crate you create.
- `Cargo.lock`.
