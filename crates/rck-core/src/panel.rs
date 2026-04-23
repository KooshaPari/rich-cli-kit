//! Box-drawn status panels.

use crate::Capabilities;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BorderStyle {
    Rounded,
    Square,
    Ascii,
}

impl Default for BorderStyle {
    fn default() -> Self { BorderStyle::Rounded }
}

struct Glyphs {
    tl: &'static str, tr: &'static str, bl: &'static str, br: &'static str,
    h: &'static str, v: &'static str,
}

fn glyphs(caps: &Capabilities, style: BorderStyle) -> Glyphs {
    let unicode = caps.unicode_width >= 2 && !matches!(style, BorderStyle::Ascii);
    match (unicode, style) {
        (true, BorderStyle::Rounded) => Glyphs { tl: "╭", tr: "╮", bl: "╰", br: "╯", h: "─", v: "│" },
        (true, BorderStyle::Square)  => Glyphs { tl: "┌", tr: "┐", bl: "└", br: "┘", h: "─", v: "│" },
        _                            => Glyphs { tl: "+", tr: "+", bl: "+", br: "+", h: "-", v: "|" },
    }
}

/// Emit a status panel. `lines` are drawn one per row inside the frame.
/// Title is truncated/padded to fit.
pub fn emit_panel<W: Write>(
    out: &mut W,
    caps: &Capabilities,
    title: &str,
    lines: &[&str],
    border_style: BorderStyle,
) -> anyhow::Result<()> {
    let g = glyphs(caps, border_style);

    // Compute inner width: max(title, longest line) + 2 padding chars.
    let content_width = std::cmp::max(
        visible_width(title),
        lines.iter().map(|l| visible_width(l)).max().unwrap_or(0),
    );
    let inner = content_width.max(12);
    let total = inner + 2; // one pad space on each side

    // Top: ╭─ title ──╮
    write!(out, "{}{}", g.tl, g.h)?;
    let title_segment = format!(" {} ", title);
    let title_w = visible_width(&title_segment);
    let remaining = total.saturating_sub(1 + title_w);
    out.write_all(title_segment.as_bytes())?;
    for _ in 0..remaining { write!(out, "{}", g.h)?; }
    writeln!(out, "{}", g.tr)?;

    // Body rows.
    for line in lines {
        let w = visible_width(line);
        let pad = inner.saturating_sub(w);
        writeln!(out, "{} {}{} {}", g.v, line, " ".repeat(pad), g.v)?;
    }

    // Bottom.
    write!(out, "{}", g.bl)?;
    for _ in 0..total { write!(out, "{}", g.h)?; }
    writeln!(out, "{}", g.br)?;
    Ok(())
}

/// Rough visible-width heuristic: counts codepoints, ignoring SGR escape sequences.
/// Adequate for the strings we render; not a full wcwidth implementation.
fn visible_width(s: &str) -> usize {
    let mut in_esc = false;
    let mut n = 0usize;
    for ch in s.chars() {
        if in_esc {
            if ch.is_ascii_alphabetic() { in_esc = false; }
            continue;
        }
        if ch == '\x1b' { in_esc = true; continue; }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::Capabilities;

    fn caps(unicode_width: u16) -> Capabilities {
        Capabilities {
            graphics: false, sixel: false, truecolor: false,
            unicode_width, terminal: "test".into(), is_tty: true,
        }
    }

    #[test]
    fn unicode_uses_rounded_corners() {
        let c = caps(2);
        let mut buf = Vec::new();
        emit_panel(&mut buf, &c, "hello", &["line one", "line two"], BorderStyle::Rounded).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("╭"));
        assert!(s.contains("╯"));
        assert!(s.contains("hello"));
        assert!(s.contains("line two"));
    }

    #[test]
    fn ascii_fallback_uses_plus_dash() {
        let c = caps(1);
        let mut buf = Vec::new();
        emit_panel(&mut buf, &c, "t", &["row"], BorderStyle::Rounded).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("+"));
        assert!(s.contains("-"));
        assert!(!s.contains("╭"));
    }
}
