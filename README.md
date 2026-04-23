# rich-cli-kit

Inline rich terminal output — images, progress bars, status panels — emitted via
the [kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
when the terminal supports it ([Ghostty](https://ghostty.org/docs/features),
kitty, WezTerm), and gracefully degraded to plain ASCII otherwise.

Built for rich-CLI-capable agents (Claude Code, Codex, forge) that need a
one-shot way to render pretty output without bringing in a full TUI stack.

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
./target/release/rck demo

# MCP server
cd mcp
pip install -e .
rich-cli-mcp                 # stdio transport (for Claude Code / Codex)
```

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
cargo test --workspace     # 10 Rust tests (encoder byte-level checks + detect smoke)
cd mcp && pytest            # 5 Python tests (subprocess wrapper + server build)
```

`rck demo` must be run interactively in a kitty-graphics terminal to
visually verify the image path; CI can only assert the byte-level encoder
output.

## Phenotype scripting policy

Per `repos/docs/governance/scripting_policy.md`, this project is Rust-first.
The MCP server is Python only because FastMCP is the canonical Python runtime;
all business logic lives in the Rust CLI and Python shells out to it.
