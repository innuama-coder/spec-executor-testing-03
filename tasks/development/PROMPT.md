# PROMPT.md — word-count (universal first instruction)

> Sent as the first user message to whichever executor is launched,
> via `send_input`.

---

Begin the **word-count** task.

This repository has no Rust code. Create a binary crate from scratch
that reads command-line arguments and outputs each token on its own
`word: <token>` line followed by `Total: N`.

Read your working agreement: `CLAUDE.md` (claude) or `AGENTS.md`
(codex) at the worktree root.

Steps:
1. Create `Cargo.toml` and `src/main.rs`.
2. Implement argument parsing via `std::env::args().skip(1)` and
   whitespace splitting.
3. Output `word: <token>` for each token, then `Total: N`.
4. Run `cargo build` and test with `cargo run -- "hello world hello"`.
5. Confirm `Total: 3` on the last line, then stop.

Constraints (full list in your agreement file):
- Standard library only. No external dependencies such as `clap`.
- Do not touch `tasks/development/`, `docs/`, `spec.yaml`,
  `README.md`, `.gitignore`.
- Do not add a `rust-toolchain.toml`.
