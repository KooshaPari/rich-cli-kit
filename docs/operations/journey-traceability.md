# Journey Traceability — rich-cli-kit

> Evidence-based traceability for CLI toolkit user journeys. Each journey maps
> keyframes to recordings, screenshots, and terminal output captures.

## Journey Evidence Matrix

| Journey | Keyframes | Evidence |
|---------|-----------|----------|
| CLI Help | 5 | `docs/journeys/manifests/cli-help/` |
| Command Execution | 6 | `docs/journeys/manifests/command-execution/` |
| Output Formatting | 5 | `docs/journeys/manifests/output-formatting/` |

---

## CLI Help Journey

**Objective:** Users discover commands and subcommands using `--help`, `man` pages, and shell completions.

### Keyframe 1 — Top-Level Help

```bash
my-cli-tool --help
# → Usage: my-cli-tool <COMMAND>
#   Commands:
#     build     Build artifacts
#     deploy    Deploy to target
#     inspect   Inspect state
#
#   Run 'my-cli-tool <COMMAND> --help' for more information.
```

### Keyframe 2 — Subcommand Help

```bash
my-cli-tool build --help
# → my-cli-tool-build
#
# Build the project artifact.
#
# USAGE:
#   my-cli-tool build [OPTIONS] --output <PATH>
#
# OPTIONS:
#   -o, --output <PATH>    Output path [required]
#   -j, --jobs <N>         Parallel jobs [default: 4]
#   -v, --verbose          Verbose output
#   -h, --help             Print help
```

### Keyframe 3 — Generate Shell Completions

```bash
# Bash
my-cli-tool completion bash > /etc/bash_completion.d/my-cli-tool.bash

# Zsh
my-cli-tool completion zsh > "${fpath[1]}/_my-cli-tool"

# Fish
my-cli-tool completion fish > ~/.config/fish/completions/my-cli-tool.fish
```

### Keyframe 4 — Tab Completion Usage

With completions installed, users type partial commands:

```bash
$ my-cli-tool build --<TAB>
--help    --output  --jobs    --verbose
```

### Keyframe 5 — Error State Help

When a required argument is missing, show contextual help:

```
Error: missing required option '--output <PATH>'
Hint: run 'my-cli-tool build --help' for usage information
```

### Evidence Links

| Link | Type | Status |
|------|------|--------|
| `docs/journeys/manifests/cli-help/` | Manifest | placeholder |

---

## Command Execution Journey

**Objective:** Parse arguments, dispatch to handlers, handle errors gracefully, and exit with correct codes.

### Keyframe 1 — Argument Parsing

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, default_value_t = 4)]
        jobs: usize,
    },
    Deploy {
        #[arg(short, long)]
        target: String,
        #[arg(short, long, action = ArgAction::SetTrue)]
        dry_run: bool,
    },
}
```

### Keyframe 2 — Subcommand Dispatch

```rust
match cli.command {
    Commands::Build { output, jobs } => build::run(output, jobs),
    Commands::Deploy { target, dry_run } => deploy::run(&target, dry_run),
}
```

### Keyframe 3 — Business Logic Execution

```rust
pub fn run(output: PathBuf, jobs: usize) -> Result<(), Error> {
    let workspace = Workspace::discover()?;
    let artifact = build::compile(&workspace, jobs)?;
    artifact.write_to(&output)?;
    println!("Artifact written to {}", output.display());
    Ok(())
}
```

### Keyframe 4 — Error Handling

```rust
match result {
    Ok(()) => {}
    Err(Error::BuildFailed(msg)) => {
        eprintln!("Build failed: {msg}");
        std::process::exit(1);
    }
    Err(Error::OutputDirNotFound(path)) => {
        eprintln!("Error: output directory '{}' not found", path.display());
        std::process::exit(2);
    }
    Err(e) => {
        eprintln!("Unexpected error: {e}");
        std::process::exit(127);
    }
}
```

### Keyframe 5 — Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Build/general failure |
| 2 | Invalid argument / configuration |
| 3 | Authentication/permission error |
| 127 | Unexpected/internal error |

### Keyframe 6 — Progress Feedback

Show progress for long-running operations:

```rust
let pb = ProgressBar::new(100);
pb.set_style(ProgressStyle::with_template(
    "{spinner:.cyan} [{elapsed_precise}] {bar:40.cyan/dim} {pos}/{len} {msg}"
)?);
for item in items.iter() {
    process(item);
    pb.inc(1);
}
pb.finish_with_message("done");
```

### Evidence Links

| Link | Type | Status |
|------|------|--------|
| `docs/journeys/manifests/command-execution/` | Manifest | placeholder |

---

## Output Formatting Journey

**Objective:** Produce beautiful terminal output with rich tables, trees, syntax highlighting, and progress bars.

### Keyframe 1 — Rich Tables

```rust
use rich::table::{Table, Row, Column};
use rich::TableOptions;

let mut table = Table::new();
table.add_column("Name", &["alice", "bob", "carol"]);
table.add_column("Role", &["admin", "user", "user"]);
table.add_column("Status", &["active", "active", "inactive"]);
table.row_style = Some(TableOptions::color(Color::Green).dimmed());
println!("{table}");
```

### Keyframe 2 — Syntax Highlighted Output

```rust
use rich::syntax::Syntax;
use rich::theme::Theme;

let syntax = Syntax::from_str(code, "rust", Theme::nord);
let theme = Theme::nord_dark();
println!("{}", syntax);
```

### Keyframe 3 — Tree Rendering

```rust
use rich::tree::Tree;
use rich::styled::Styled;
use rich::text::Text;

let root = Tree::new("project/");
root.add("src/");
let lib = root.add("lib/");
lib.add("main.rs");
lib.add("lib.rs");
println!("{root}");
```

### Keyframe 4 — Panel and Box Layouts

```rust
use rich::panel::Panel;
use rich::layout::Layout;

let panel = Panel::new(
    "Deploy to production?\n\n[yellow]This will overwrite running instances.[/yellow]"
).border_style(Color::Yellow);
println!("{panel}");
```

### Keyframe 5 — Paginated Output

```rust
use rich::pager::Pager;
use rich::console::Console;

let pager = Pager::new();
pager.display_lines(&all_lines, &console)?;
```

### Evidence Links

| Link | Type | Status |
|------|------|--------|
| `docs/journeys/manifests/output-formatting/` | Manifest | placeholder |

---

## Keyframe Template

```markdown
### Keyframe N — Title

**What:** One sentence describing the action.

**Why:** One sentence on the value delivered.

**Evidence:** Link to recording, screenshot, or terminal output capture.

**Verification command:**
```bash
# optional shell command to validate
```
```
