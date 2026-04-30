# Known Issues

- Canonical `rich-cli-kit` is dirty and also behind origin. Badge integration
  into canonical `main` is deferred until unrelated local changes are handled.
- The isolated badge branch is prepared but not pushed or merged.
- `cargo fmt --check` fails on existing Rust source formatting outside this
  README-only change.
- `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` cannot
  resolve crates.io from this sandbox, so dependency download blocks those
  checks before source compilation.
