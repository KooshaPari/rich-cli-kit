//! rck — Rich-CLI Kit CLI.
//!
//! One-shot inline renderer: `rck detect | image | progress | panel | demo`.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rck_core::{
    detect, emit_image, emit_panel, emit_progress, BorderStyle, Capabilities, ImageData,
    ProgressStyle,
};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rck", version, about = "Rich-CLI Kit — inline images, progress bars, status panels")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print detected terminal capabilities as JSON.
    Detect,
    /// Render an image inline (PNG/JPG via kitty graphics; text fallback otherwise).
    Image {
        path: PathBuf,
        /// Alt text printed in fallback mode.
        #[arg(long)]
        alt: Option<String>,
    },
    /// Render a one-shot progress bar.
    Progress {
        /// Progress ratio 0.0–1.0.
        ratio: f32,
        #[arg(long)]
        label: Option<String>,
        /// Force ASCII glyphs.
        #[arg(long)]
        ascii: bool,
    },
    /// Render a titled status panel from a file or stdin.
    Panel {
        #[arg(long)]
        title: String,
        /// Read body lines from this file (use "-" for stdin).
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value = "rounded")]
        border: String,
    },
    /// Show all four renderers as a self-test.
    Demo,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let caps = detect();
    let mut out = io::stdout().lock();

    match cli.cmd {
        Cmd::Detect => {
            let json = serde_json::to_string_pretty(&caps)?;
            writeln!(out, "{json}")?;
        }
        Cmd::Image { path, alt } => {
            let mut img = ImageData::from_path(&path)
                .with_context(|| format!("loading image {}", path.display()))?;
            if let Some(a) = alt { img.alt_text = Some(a); }
            emit_image(&mut out, &caps, &img)?;
        }
        Cmd::Progress { ratio, label, ascii } => {
            let style = if ascii { ProgressStyle::Ascii } else { ProgressStyle::Blocks };
            emit_progress(&mut out, &caps, ratio, style, label.as_deref())?;
        }
        Cmd::Panel { title, file, border } => {
            let body = read_body(file.as_deref())?;
            let lines: Vec<&str> = body.lines().collect();
            let style = match border.as_str() {
                "square" => BorderStyle::Square,
                "ascii" => BorderStyle::Ascii,
                _ => BorderStyle::Rounded,
            };
            emit_panel(&mut out, &caps, &title, &lines, style)?;
        }
        Cmd::Demo => demo(&mut out, &caps)?,
    }
    Ok(())
}

fn read_body(file: Option<&std::path::Path>) -> Result<String> {
    match file {
        None => Ok(String::new()),
        Some(p) if p == std::path::Path::new("-") => {
            let mut s = String::new();
            use std::io::Read;
            io::stdin().read_to_string(&mut s)?;
            Ok(s)
        }
        Some(p) => Ok(fs::read_to_string(p).with_context(|| format!("reading {}", p.display()))?),
    }
}

fn demo<W: Write>(out: &mut W, caps: &Capabilities) -> Result<()> {
    writeln!(out, "rck demo\n")?;
    writeln!(out, "1) capabilities:")?;
    writeln!(out, "{}\n", serde_json::to_string_pretty(caps)?)?;

    writeln!(out, "2) progress bars:")?;
    for r in [0.15f32, 0.5, 0.85, 1.0] {
        emit_progress(out, caps, r, ProgressStyle::Blocks, Some("task"))?;
    }
    writeln!(out)?;

    writeln!(out, "3) status panel:")?;
    emit_panel(
        out,
        caps,
        "status",
        &[
            "build: ok",
            "tests: 12 passed",
            "graphics: detected",
        ],
        BorderStyle::Rounded,
    )?;
    writeln!(out)?;

    writeln!(out, "4) image: (skipped — pass `rck image <path>` to render)")?;
    Ok(())
}
