# Testing Strategy

Validation scope:

- `cargo fmt --check` was attempted and failed on existing Rust formatting
  drift unrelated to this README/session-doc change.
- `cargo test --workspace` was attempted and blocked by crates.io DNS/network
  resolution while fetching `anyhow`.
- `cargo clippy --workspace -- -D warnings` was attempted and blocked by the
  same crates.io DNS/network resolution.
- Confirm `git status` is clean after commit in the isolated worktree.
- Confirm the final commit message includes the required Codex co-author
  trailer.
