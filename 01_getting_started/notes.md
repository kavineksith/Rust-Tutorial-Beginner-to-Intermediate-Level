# Rust Get Started

## Install Rust

The official installer is **`rustup`**, which manages Rust versions and
associated tools.

### Linux / macOS

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your shell (or run `source $HOME/.cargo/env`).

### Windows

Download and run `rustup-init.exe` from
<https://www.rust-lang.org/tools/install>. You will also need the
"Build Tools for Visual Studio" (the C++ workload).

### Verify the Installation

```bash
rustc --version
cargo --version
```

You should see version numbers for both the compiler and the build tool.

## Two Tools You Will Use Daily

| Tool | Purpose |
|------|---------|
| `rustc` | The Rust compiler. Compiles a single `.rs` file. |
| `cargo` | The build system and package manager. Creates projects, fetches dependencies, runs tests. |

## Hello World — Three Ways

### Way 1: Plain `rustc` (good for tiny experiments)

Create a file `main.rs`:

```rust
fn main() {
    println!("Hello, World!");
}
```

Compile and run:

```bash
rustc main.rs
./main
```

### Way 2: Cargo project (recommended for real projects)

```bash
cargo new my_project
cd my_project
cargo run
```

Cargo generates this layout for you:

```
my_project/
├── Cargo.toml     # project manifest (name, version, dependencies)
├── src/
│   └── main.rs    # your code, with a "Hello, world!" already inside
└── target/        # build output (auto-created)
```

### Way 3: Run without saving a binary (Rust 1.77+)

```bash
rustc -o /tmp/hello main.rs && /tmp/hello
```

## Useful Cargo Commands

| Command | What it does |
|---------|--------------|
| `cargo new <name>` | Create a new binary project |
| `cargo new <name> --lib` | Create a library project |
| `cargo build` | Compile the project |
| `cargo run` | Compile and run |
| `cargo check` | Type-check without producing a binary (fast) |
| `cargo test` | Run all tests |
| `cargo fmt` | Auto-format your code |
| `cargo clippy` | Run the linter for extra suggestions |
| `cargo build --release` | Optimised build for production |

## Edit / IDE Support

* **rust-analyzer** is the recommended language server and works with
  VS Code, IntelliJ, Neovim, Emacs and more.
* Install it from your editor's extension marketplace.

**Next:** [Syntax](../03_syntax) — the structure of a Rust program.
