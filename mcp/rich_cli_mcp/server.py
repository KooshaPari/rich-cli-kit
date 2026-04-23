"""FastMCP server exposing rich-cli-kit tools.

Tools:
    rich.emit_image
    rich.emit_progress
    rich.emit_panel

Each tool detects terminal capabilities first. If the attached terminal cannot
render the requested output, the tool returns a structured result with
``rendered=False`` and a ``fallback_text`` payload; the caller (Claude Code /
Codex / forge) then decides how to present it to the user.
"""
from __future__ import annotations

import os
from typing import Literal

try:
    from fastmcp import FastMCP
except ImportError:  # pragma: no cover — informative error at runtime
    FastMCP = None  # type: ignore[assignment]

from . import rck


def build_server():
    if FastMCP is None:
        raise RuntimeError("fastmcp not installed; `pip install 'fastmcp>=2.0'`")
    mcp = FastMCP("rich-cli-kit")

    @mcp.tool
    def emit_image(path: str, alt_text: str = "") -> dict:
        """Render a PNG/JPG inline via kitty-graphics protocol if supported."""
        caps = rck.detect()
        if not caps.graphics:
            return {
                "rendered": False,
                "reason": f"terminal '{caps.terminal}' does not support kitty-graphics",
                "fallback_text": f"[image: {path} — {alt_text or 'no description'}]",
                "capabilities": caps.__dict__,
            }
        try:
            bytes_ = rck.run_image(path, alt=alt_text or None)
        except Exception as e:
            return {"rendered": False, "reason": str(e), "fallback_text": f"[image: {path}]"}
        return {
            "rendered": True,
            "bytes_b64": _b64(bytes_),
            "capabilities": caps.__dict__,
        }

    @mcp.tool
    def emit_progress(ratio: float, label: str = "") -> dict:
        """Render a single progress bar. Always renders (ASCII fallback)."""
        caps = rck.detect()
        try:
            bytes_ = rck.run_progress(ratio, label=label or None, ascii_=(caps.unicode_width < 2))
        except Exception as e:
            pct = max(0, min(100, int(ratio * 100)))
            return {"rendered": False, "reason": str(e), "fallback_text": f"[{pct}%] {label}"}
        return {
            "rendered": True,
            "text": bytes_.decode("utf-8", errors="replace"),
            "capabilities": caps.__dict__,
        }

    @mcp.tool
    def emit_panel(
        title: str,
        body: str,
        kind: Literal["info", "warn", "error", "success"] = "info",
    ) -> dict:
        """Render a titled status panel. Always renders (ASCII fallback)."""
        caps = rck.detect()
        border = "rounded" if caps.unicode_width >= 2 else "ascii"
        decorated_title = f"[{kind}] {title}"
        try:
            bytes_ = rck.run_panel(decorated_title, body, border=border)
        except Exception as e:
            return {
                "rendered": False,
                "reason": str(e),
                "fallback_text": f"=== {decorated_title} ===\n{body}\n",
            }
        return {
            "rendered": True,
            "text": bytes_.decode("utf-8", errors="replace"),
            "capabilities": caps.__dict__,
        }

    return mcp


def _b64(b: bytes) -> str:
    import base64
    return base64.b64encode(b).decode("ascii")


def main() -> None:
    mcp = build_server()
    transport = os.environ.get("RICH_CLI_MCP_TRANSPORT", "stdio")
    mcp.run(transport=transport)


if __name__ == "__main__":
    main()
