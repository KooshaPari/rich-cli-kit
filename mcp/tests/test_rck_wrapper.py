"""Tests for the `rck` subprocess wrapper.

These exercise the real debug-built binary (`../target/debug/rck`) so we
verify end-to-end: Python → CLI → kitty-graphics-or-fallback bytes.
"""
from __future__ import annotations

import os
import sys

import pytest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from rich_cli_mcp import rck  # noqa: E402


@pytest.fixture(scope="module")
def rck_bin() -> str:
    try:
        return rck.find_rck()
    except rck.RckUnavailable as e:
        pytest.skip(str(e))


def test_detect_returns_capabilities(rck_bin):
    caps = rck.detect()
    assert caps.terminal
    assert isinstance(caps.graphics, bool)
    assert isinstance(caps.unicode_width, int)


def test_progress_renders_percentage(rck_bin):
    out = rck.run_progress(0.5, label="halfway", ascii_=True).decode()
    assert "50%" in out
    assert "halfway" in out
    assert "[" in out and "]" in out


def test_panel_renders_title(rck_bin):
    out = rck.run_panel("smoke", "line a\nline b", border="ascii").decode()
    assert "smoke" in out
    assert "line a" in out
    assert "line b" in out
    assert "+" in out


def test_progress_clamps_over_one(rck_bin):
    out = rck.run_progress(2.0, ascii_=True).decode()
    assert "100%" in out


def test_server_builds():
    """Just import + build; real MCP transport isn't exercised here."""
    from rich_cli_mcp.server import build_server
    try:
        mcp = build_server()
    except RuntimeError as e:
        pytest.skip(str(e))
    assert mcp is not None
