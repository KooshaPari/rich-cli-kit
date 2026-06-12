<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/rich-cli-kit/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/rich-cli-kit?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/rich-cli-kit?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
> **Work state:** ACTIVE · **Progress:** `████████░░ 80%`
> Terminal-UX toolkit (`rck` + `rck-core` lib + FastMCP server): inline images/progress/panels via kitty-graphics, ASCII fallback. Public + consumed by KLA & thegent-dispatch. Broader adoption ongoing. · updated 2026-06-02

> **Pinned references (Phenotype-org)**
> - MSRV: see rust-toolchain.toml
> - cargo-deny config: see deny.toml
> - cargo-audit: rustsec/audit-check@v2 weekly
> - Branch protection: 1 reviewer required, no force-push
> - Authority: phenotype-org-governance/SUPERSEDED.md

# rich-cli-kit

[![AI Slop Inside](https://sladge.net/badge.svg)](https://sladge.net)

Inline rich terminal output — images, progress bars, status panels — emitted via
the [kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
when the terminal supports it ([Ghostty](https://ghostty.org/docs/features),
kitty, WezTerm), and gracefully degraded to plain ASCII otherwise.

Built for rich-CLI-capable agents (Claude Code, Codex, forge) that need a
one-shot way to render pretty output without bringing in a full TUI stack.

## Installation

Install the `rck` binary straight from Git with Cargo (the binary lives in the
`rck-cli` workspace crate):

```bash
cargo install --git https://github.com/KooshaPari/rich-cli-kit rck-cli
```

Or, with [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall --git https://github.com/KooshaPari/rich-cli-kit rck-cli
```

For local development, see [Quickstart](#quickstart).

## Components

```
rich-cli-kit/
├── crates/
│   ├── rck-core/        # capability detection + kitty-graphics encoder
│   └── rck-cli/         # `rck` binary — detect | image | progress | panel | demo
└── mcp/
    └── rich_cli_mcp/    # FastMCP server exposing rich.* tools over stdio
```

## Quickstart

```bash
# Rust CLI
cd rich-cli-kit
cargo build --release
./target/release/rck detect
./target/release/rck progress 0.42 --label "building"
./target/release/rck panel --title status --file <(echo -e "build: ok\ntests: 10 passed")
./target/release/rck image path/to/screenshot.png

# v0.2 additions
./target/release/rck link https://ghostty.org "Ghostty"          # OSC 8 hyperlink
echo "result" | ./target/release/rck copy --stdin                # OSC 52 clipboard
./target/release/rck task-start --id build                       # OSC 133 A + C
./target/release/rck task-end --exit 0                           # OSC 133 D
./target/release/rck ask "Proceed with deploy?"                  # yes/no (exit 0/1/2)
./target/release/rck pick "Pick env" dev staging prod            # arrow-key select
./target/release/rck input "Ticket number"                       # single-line
./target/release/rck shader install focus-vignette               # Ghostty shader pack

./target/release/rck demo

# MCP server
cd mcp
pip install -e .
rich-cli-mcp                 # stdio transport (for Claude Code / Codex)
```

## tmux passthrough

All OSC / APC sequences (image, hyperlink, clipboard, task markers, panel
links, progress labels) are automatically DCS-wrapped when `$TMUX` is set, so
tmux forwards them to the outer terminal. This only works if you enable
passthrough in your tmux config:

```
# ~/.tmux.conf
set -g allow-passthrough on
```

Without that line, tmux drops every wrapped sequence silently.

## Terminal support

| Terminal | Kitty graphics | Sixel | Truecolor |
|----------|:--------------:|:-----:|:---------:|
| Ghostty  | ✓              |       | ✓         |
| kitty    | ✓              |       | ✓         |
| WezTerm  | ✓              | ✓     | ✓         |
| Konsole  |                | ✓     | ✓         |
| iTerm2   |                | ✓     | ✓         |

Sixel support is a stretch goal — `rck` detects it but the current encoder only
emits kitty graphics. On sixel-only terminals, the image command falls back to
a text summary.

## Detection logic

`rck detect` combines three signals:

1. **Env vars** — `TERM_PROGRAM=ghostty|Ghostty|WezTerm`, `TERM` (e.g.
   `xterm-ghostty`, `xterm-kitty`), `KITTY_WINDOW_ID`, `WEZTERM_EXECUTABLE`,
   `KONSOLE_VERSION`, `COLORTERM=truecolor`.
2. **Kitty-graphics query** — when stdout is a TTY and env signals were
   inconclusive, `rck` sends
   `ESC _ G i=31,s=1,v=1,a=q,t=d,f=24 ; AAAA ESC \ ESC [c`
   and polls stdin for up to 150 ms looking for `_Gi=31;OK`.
3. **TTY check** — if stdout is not a TTY, `graphics` is forced to `false` to
   avoid writing APC sequences into pipes or logs.

## Protocol notes

- PNG is transmitted with `a=T,f=100,q=2`. `q=2` silences the terminal's
  ack/error responses so one-shot CLI output stays clean.
- Base64 payloads over 4096 bytes are split into APC frames with `m=1` on each
  non-final frame and `m=0` on the last. Every chunk length is a multiple of 4
  so base64 alignment is preserved.
- The encoder re-encodes JPEG inputs to PNG so the `f=100` path always works,
  avoiding the per-format quirks of `f=24` / `f=32` (which require explicit
  `s`, `v`, and pixel stride).

## Tests

```bash
cargo test --workspace     # 44 Rust tests (encoder, wrap, osc8, osc52, osc133, width, spans, shader, interactive)
cd mcp && pytest            # 11 Python tests (subprocess wrapper + server build + new v0.2 tools)
```

`rck demo` must be run interactively in a kitty-graphics terminal to
visually verify the image path; CI can only assert the byte-level encoder
output.

## Phenotype scripting policy

Per `repos/docs/governance/scripting_policy.md`, this project is Rust-first.
The MCP server is Python only because FastMCP is the canonical Python runtime;
all business logic lives in the Rust CLI and Python shells out to it.
