# rich-cli-kit — AGENTS.md

## Project Overview

Rust CLI toolkit — rich console output utilities for the command line.

## Stack

- Language: Rust (per GitHub language detection)
- Build system: Cargo (verify `Cargo.toml`)
- Package manager: Cargo

## Key Commands

```bash
# Verify project structure
ls -la Cargo.toml Cargo.lock 2>/dev/null

# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy
```

## Notes

- **Active** — verify language and build system locally before running commands
- CLI tool — likely has binary with `[[bin]]` entries
