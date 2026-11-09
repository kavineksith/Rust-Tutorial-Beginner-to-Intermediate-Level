# Command-Line Apps

Rust is a great language for writing CLI tools — fast startup, single
static binary, cross-compilation. This chapter covers reading args,
parsing flags, exit codes, env vars, and spawning child processes.

## Reading CLI Arguments with `std::env::args`

```rust
use std::env;

let args: Vec<String> = env::args().collect();
// args[0] is the program name; args[1..] are user-provided args.

for (i, arg) in args.iter().enumerate() {
    println!("[{}] {}", i, arg);
}
```

Run with: `./my_app foo bar --flag baz`

## A Simple Manual Parser

For small tools, `std::env::args` is enough:

```rust
let args: Vec<String> = env::args().collect();
let mut name = String::new();
let mut uppercase = false;
let mut count: usize = 1;

let mut i = 1;
while i < args.len() {
    match args[i].as_str() {
        "--uppercase" | "-u" => uppercase = true,
        "--count" | "-c" => {
            i += 1;
            count = args[i].parse().map_err(|_| "count must be a number")?;
        }
        arg if arg.starts_with("--") => return Err(format!("unknown flag: {}", arg)),
        _ => name = args[i].clone(),
    }
    i += 1;
}
```

## Use `clap` for Real Projects

`clap` is the standard CLI parser. It handles `--help`, `--version`, type
checking, subcommands, env vars, and shell completions.

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// Greet a person by name.
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Print in uppercase
    #[arg(short, long)]
    uppercase: bool,
}

fn main() {
    let args = Args::parse();
    let greeting = if args.uppercase {
        format!("HELLO, {}!", args.name.to_uppercase())
    } else {
        format!("Hello, {}!", args.name)
    };
    for _ in 0..args.count {
        println!("{}", greeting);
    }
}
```

### Run It

```bash
$ greet Alice
Hello, Alice!

$ greet Bob --count 3
Hello, Bob!
Hello, Bob!
Hello, Bob!

$ greet Carol -u -c 2
HELLO, CAROL!
HELLO, CAROL!

$ greet --help
Greet a person by name.

Usage: greet [OPTIONS] <NAME>

Arguments:
  <NAME>  Name of the person to greet

Options:
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -u, --uppercase      Print in uppercase
  -h, --help           Print help
  -V, --version        Print version
```

### Clap Features

| Feature | How |
|---------|-----|
| Positional args | `name: String` |
| Optional args | `name: Option<String>` |
| Boolean flags | `verbose: bool` |
| Default values | `#[arg(default_value_t = 1)]` |
| Short flag | `#[arg(short = 'v')]` (auto from field name) |
| Long flag | `#[arg(long = "verbose")]` (auto from field name) |
| Required vs optional | `Option<T>` = optional |
| Count flag | `#[arg(action = clap::ArgAction::Count)] verbose: u8` |
| Env var fallback | `#[arg(env = "VERBOSE")]` |
| Subcommands | `#[command(subcommand)] cmd: Commands` |
| Version | `#[command(version)]` on the struct |

## Subcommands (git-style)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a file
    Add { file: String },
    /// List files
    List,
    /// Remove a file
    Remove { file: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { file } => println!("adding {}", file),
        Commands::List => println!("listing"),
        Commands::Remove { file } => println!("removing {}", file),
    }
}
```

```bash
$ myapp add foo.txt
$ myapp list
$ myapp remove foo.txt
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | success |
| 1 | general failure |
| 2 | usage error (wrong args) |
| 3+ | application-specific |

```rust
use std::process;

process::exit(0);   // success
process::exit(1);   // failure
process::exit(2);   // usage error
```

### Returning `Result` from `main`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    do_work()?;
    Ok(())    // exit code 0
}
```

Returning `Err` from `main` prints the error (via `Debug`) and exits
with code 1. For finer control, call `process::exit` explicitly.

## Environment Variables

```rust
use std::env;

// Read an env var:
let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));

// Set an env var (for child processes):
env::set_var("MY_VAR", "value");

// Iterate all env vars:
for (key, value) in env::vars() {
    println!("{}={}", key, value);
}
```

### With clap

```rust
#[derive(Parser)]
struct Args {
    /// API key (or set MYAPP_API_KEY)
    #[arg(env = "MYAPP_API_KEY")]
    api_key: String,
}
```

If the user doesn't pass `--api-key`, clap falls back to the env var.

## Standard Input / Output / Error

```rust
use std::io::{self, Read, Write};

// Read a line from stdin:
let mut input = String::new();
io::stdin().read_line(&mut input)?;
println!("you typed: {}", input.trim());

// Read all of stdin:
let mut buf = Vec::new();
io::stdin().read_to_end(&mut buf)?;

// Write to stdout:
print!("hello");
io::stdout().flush()?;   // ensure it's flushed

// Write to stderr:
eprintln!("warning: ...");
```

## Spawning Child Processes

```rust
use std::process::Command;
use std::io::Write;

// Run and capture output:
let output = Command::new("echo")
    .arg("hello")
    .arg("world")
    .output()?;

println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
println!("exit code: {:?}", output.status.code());

// Stream output (don't capture):
let status = Command::new("ls")
    .arg("-l")
    .status()?;

// Pipe stdin:
use std::process::Stdio;
let mut child = Command::new("grep")
    .arg("rust")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

if let Some(mut stdin) = child.stdin.take() {
    stdin.write_all(b"rust is great\npython is also fine\n")?;
}
let output = child.wait_with_output()?;
println!("matches: {}", String::from_utf8_lossy(&output.stdout));
```

## Signals (Ctrl-C, SIGTERM)

The `ctrlc` crate handles Ctrl-C:

```toml
[dependencies]
ctrlc = "3"
```

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        // do work
    }
    println!("shutting down gracefully");
}
```

For async, use `tokio::signal::ctrl_c()`.

## Colored Output

```toml
[dependencies]
colored = "2"
```

```rust
use colored::*;

println!("{}", "hello".green().bold());
println!("{}", "error".red());
println!("{}", "warning".yellow());
```

Detect if output is a TTY before coloring (use the `atty` or `is-terminal`
crate) to avoid escape codes in pipes.

## Progress Bars

```toml
[dependencies]
indicatif = "0.17"
```

```rust
use indicatif::ProgressBar;

let pb = ProgressBar::new(100);
for _ in 0..100 {
    pb.inc(1);
    std::thread::sleep(std::time::Duration::from_millis(10));
}
pb.finish_with_message("done");
```

## Logging

See chapter 48. The `log` + `env_logger` combo is standard:

```rust
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("something suspicious");
}
```

Run with `RUST_LOG=info` to see the output.

## Project Layout for a CLI

```
my_cli/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs        # entry point — parse args, dispatch
│   ├── cli.rs         # clap definitions
│   ├── commands/      # one module per subcommand
│   │   ├── mod.rs
│   │   ├── add.rs
│   │   └── list.rs
│   └── config.rs      # config loading
└── tests/
    └── integration.rs
```

## Distributing a CLI

| Method | How |
|--------|-----|
| `cargo install` | `cargo install my_cli` (from crates.io) |
| Pre-built binaries | GitHub Releases + CI |
| `cargo-binstall` | download pre-built binaries via cargo |
| Homebrew | `brew install my_cli` |
| apt / dnf | Linux distro packages |
| Docker | `docker run my_cli` |

For CI builds, use `cross` for cross-compilation:

```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
```

## Useful CLI Crates

| Crate | Purpose |
|-------|---------|
| `clap` | argument parsing |
| `structopt` | older clap derive (now part of clap) |
| `colored` | colored output |
| `is-terminal` | check if stdout is a TTY |
| `indicatif` | progress bars |
| `dialoguer` | interactive prompts |
| `console` | terminal utilities |
| `ctrlc` | handle Ctrl-C |
| `notify` | file watching |
| `dirs` | standard config/cache dirs |
| `anyhow` | error handling for apps |
| `env_logger` / `tracing` | logging |

## CLI Design Best Practices

* Always support `--help` and `--version`.
* Use clear, consistent flag names (`--verbose`, not `--v`).
* Provide sensible defaults.
* Print errors to **stderr**, normal output to **stdout**.
* Return non-zero exit codes on failure.
* Make output machine-parseable when piped (consider `--json` flag).
* Don't print colors when stdout isn't a TTY.
* Respect `NO_COLOR` env var.
* Provide a `--quiet` flag for scripts.

**Next:** [Serde & Serialization](../45_serde) — JSON, YAML, custom serializers.
