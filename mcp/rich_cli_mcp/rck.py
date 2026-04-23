"""Thin subprocess wrapper around the `rck` binary.

Agents call this via FastMCP tools; we shell out to the Rust CLI so that the
actual kitty-graphics escape sequences are emitted by a single source of truth.
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
from dataclasses import dataclass
from typing import Optional


class RckUnavailable(RuntimeError):
    """Raised when the `rck` binary cannot be located."""


@dataclass
class Capabilities:
    graphics: bool
    sixel: bool
    truecolor: bool
    unicode_width: int
    terminal: str
    is_tty: bool

    @classmethod
    def plain(cls) -> "Capabilities":
        return cls(False, False, False, 1, "unknown", False)


def find_rck() -> str:
    """Locate the `rck` binary.

    Resolution order: `RCK_BIN` env → `rck` on PATH → debug build in repo.
    """
    env = os.environ.get("RCK_BIN")
    if env and os.path.isfile(env) and os.access(env, os.X_OK):
        return env
    found = shutil.which("rck")
    if found:
        return found
    # Dev fallback — repo-local debug build.
    here = os.path.dirname(os.path.abspath(__file__))
    candidate = os.path.normpath(os.path.join(here, "..", "..", "target", "debug", "rck"))
    if os.path.isfile(candidate) and os.access(candidate, os.X_OK):
        return candidate
    raise RckUnavailable(
        "`rck` binary not found. Install with `cargo build --release` in rich-cli-kit/ "
        "or set RCK_BIN."
    )


def detect() -> Capabilities:
    try:
        bin_ = find_rck()
    except RckUnavailable:
        return Capabilities.plain()
    proc = subprocess.run([bin_, "detect"], capture_output=True, text=True, timeout=5)
    if proc.returncode != 0:
        return Capabilities.plain()
    data = json.loads(proc.stdout)
    return Capabilities(
        graphics=bool(data.get("graphics")),
        sixel=bool(data.get("sixel")),
        truecolor=bool(data.get("truecolor")),
        unicode_width=int(data.get("unicode_width", 1)),
        terminal=str(data.get("terminal", "unknown")),
        is_tty=bool(data.get("is_tty")),
    )


def run_image(path: str, alt: Optional[str] = None) -> bytes:
    bin_ = find_rck()
    args = [bin_, "image", path]
    if alt:
        args += ["--alt", alt]
    proc = subprocess.run(args, capture_output=True, timeout=30)
    proc.check_returncode()
    return proc.stdout


def run_progress(ratio: float, label: Optional[str] = None, ascii_: bool = False) -> bytes:
    bin_ = find_rck()
    args = [bin_, "progress", f"{ratio}"]
    if label:
        args += ["--label", label]
    if ascii_:
        args += ["--ascii"]
    proc = subprocess.run(args, capture_output=True, timeout=5)
    proc.check_returncode()
    return proc.stdout


def run_panel(title: str, body: str, border: str = "rounded") -> bytes:
    bin_ = find_rck()
    args = [bin_, "panel", "--title", title, "--file", "-", "--border", border]
    proc = subprocess.run(args, input=body.encode("utf-8"), capture_output=True, timeout=5)
    proc.check_returncode()
    return proc.stdout
