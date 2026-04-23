//! rck-core — terminal capability detection + rich rendering primitives.
//!
//! Emits kitty-graphics-protocol sequences on capable terminals (Ghostty, kitty,
//! WezTerm), and falls back to plain ASCII otherwise.
//!
//! Reference:
//! - <https://sw.kovidgoyal.net/kitty/graphics-protocol/>
//! - <https://ghostty.org/docs/features>

pub mod capabilities;
pub mod encoder;
pub mod image_data;
pub mod panel;
pub mod progress;

pub use capabilities::{detect, Capabilities};
pub use image_data::ImageData;
pub use panel::{emit_panel, BorderStyle};
pub use progress::{emit_progress, ProgressStyle};

use std::io::Write;

/// High-level convenience: write an image to `out` using the best mode for `caps`.
pub fn emit_image<W: Write>(out: &mut W, caps: &Capabilities, img: &ImageData) -> anyhow::Result<()> {
    if caps.graphics {
        encoder::write_kitty_png(out, &img.png_bytes)
    } else {
        // Fallback: short textual summary
        writeln!(
            out,
            "[image: {}x{} {} (graphics protocol unavailable)]",
            img.width,
            img.height,
            img.alt_text.as_deref().unwrap_or("untitled")
        )?;
        Ok(())
    }
}
