# Cargo & Build System (in depth)

**Cargo** is Rust's official build system and package manager. It handles
dependencies, compilation, testing, documentation, and publishing — all
from one `Cargo.toml` file.

## Quick Reference

```bash
cargo new my_app              # create a new binary project
cd my_app
cargo build                   # compile (debug mode)
cargo run                     # compile + run
cargo test                    # run all tests
cargo doc --open              # build & view docs
cargo fmt                     # auto-format code
cargo clippy                  # lint
cargo add serde               # add a dependency
cargo publish                 # publish to crates.io
```

## Project Layout

```
my_app/
├── Cargo.toml        # manifest: name, version, dependencies, features
├── Cargo.lock        # pinned dependency versions (auto-generated)
├── src/
│   ├── main.rs       # binary entry point (crate root for binaries)
│   ├── lib.rs        # library crate root (for libraries)
│   └── bin/
│       └── tool.rs   # additional binaries
├── tests/
│   └── integration.rs   # integration tests
├── benches/
│   └── my_bench.rs      # benchmarks (criterion)
├── examples/
│   └── demo.rs          # examples (run with `cargo run --example demo`)
├── build.rs             # build script (optional)
└── target/              # build output (auto-generated, gitignored)
```

## `Cargo.toml` Anatomy

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"
authors = ["You <you@example.com>"]
description = "A short description"
license = "MIT OR Apache-2.0"
repository = "https://github.com/you/my_app"
readme = "README.md"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
my_local = { path = "../my_local" }
my_git = { git = "https://github.com/foo/bar", branch = "dev" }

[dev-dependencies]
proptest = "1.0"

[build-dependencies]
cc = "1.0"

[features]
default = ["json"]
json = ["dep:serde_json"]
yaml = ["dep:serde_yaml"]

[[bin]]
name = "my_app"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## Dependency Version Syntax

| Spec | Meaning |
|------|---------|
| `"1.0"` / `"^1.0"` | ≥1.0.0, <2.0.0 (caret — default) |
| `"~1.0"` | ≥1.0.0, <1.1.0 (tilde) |
| `"=1.0.0"` | exactly 1.0.0 |
| `"*"` | any version (dangerous!) |
| `">=1.0, <2.0"` | range |

## Adding Dependencies

```toml
[dependencies]
# Three equivalent ways:
serde = "1.0"
serde = { version = "1.0" }
serde = { version = "1.0", features = ["derive"] }

# Local path (good for monorepos):
common = { path = "../common" }

# Git:
my_crate = { git = "https://github.com/foo/bar" }
my_crate = { git = "...", branch = "dev" }
my_crate = { git = "...", tag = "v1.0.0" }
my_crate = { git = "...", rev = "abc123" }
```

## Features

Features let users opt into optional behavior:

```toml
[features]
default = ["json"]
json = ["dep:serde_json"]
yaml = ["dep:serde_yaml"]
```

In code:

```rust
#[cfg(feature = "json")]
fn parse_json(s: &str) { /* ... */ }
```

Users enable features when depending on your crate:

```toml
[dependencies]
my_app = { version = "1.0", features = ["yaml"] }
my_app = { version = "1.0", default-features = false, features = ["json"] }
```

## Profiles — Build Configuration

```toml
[profile.release]
opt-level = 3           # 0-3, "s" (size), "z" (smaller)
lto = true              # link-time optimization
codegen-units = 1       # slower build, better optimization
panic = "abort"         # smaller binary, no unwinding
strip = true            # remove debug symbols
debug = false           # no debug info

[profile.dev]
opt-level = 0
debug = true

[profile.dev.package."*"]
opt-level = 3           # optimize dependencies even in dev
```

## Workspaces

For multi-crate projects:

```toml
# Top-level Cargo.toml
[workspace]
members = ["cli", "lib", "common"]
resolver = "2"
```

All crates share one `target/` directory and one `Cargo.lock`. Run commands
for a specific member:

```bash
cargo build -p lib
cargo test -p cli
```

## Build Scripts (`build.rs`)

Run code **before** compilation. Use for:

* Generating Rust code from a schema (protobuf, SQL).
* Compiling C/C++ code (via the `cc` crate).
* Setting custom linker flags.
* Reading environment variables.

```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=schema.sql");
    println!("cargo:rustc-link-lib=z");
}
```

## Conditional Compilation (`cfg`)

```rust
// Attribute form:
#[cfg(target_os = "linux")]
fn linux_only() { /* ... */ }

// Macro form (returns bool):
if cfg!(debug_assertions) {
    println!("debug build");
}
```

### Common `cfg` Keys

| Key | Values |
|-----|--------|
| `target_os` | `linux`, `windows`, `macos`, `android`, `ios`, ... |
| `target_arch` | `x86_64`, `aarch64`, `wasm32`, ... |
| `target_family` | `unix`, `windows` |
| `target_pointer_width` | `32`, `64` |
| `feature` | your feature names |
| `debug_assertions` | (true in debug) |
| `test` | (true when running `cargo test`) |

## Editions

| Edition | Highlights |
|---------|------------|
| 2015 | original Rust |
| 2018 | module system overhaul, `dyn Trait`, async/await |
| 2021 | array `IntoIterator`, disjoint closure captures |
| 2024 | unsafe attributes, more edition cleanups |

Set in `Cargo.toml`: `edition = "2024"`. Always use the latest.

## Useful Cargo Commands

| Command | Purpose |
|---------|---------|
| `cargo new <name>` | create a new project |
| `cargo build` | compile |
| `cargo build --release` | optimized compile |
| `cargo run` | build + run |
| `cargo check` | type-check only (fast) |
| `cargo test` | run all tests |
| `cargo bench` | run benchmarks |
| `cargo doc` | generate documentation |
| `cargo fmt` | auto-format code |
| `cargo clippy` | lint |
| `cargo add <dep>` | add a dependency |
| `cargo update` | update `Cargo.lock` |
| `cargo tree` | show dependency tree |
| `cargo tree --duplicates` | show duplicate dependency versions |
| `cargo clean` | remove `target/` |
| `cargo publish` | publish to crates.io |
| `cargo login <token>` | save crates.io token |
| `cargo install <crate>` | install a binary globally |
| `cargo uninstall <crate>` | remove an installed binary |

## Semantic Versioning

Cargo follows **SemVer**: `MAJOR.MINOR.PATCH`

* **PATCH** (1.0.0 → 1.0.1): bug fixes only
* **MINOR** (1.0.0 → 1.1.0): new features, backward compatible
* **MAJOR** (1.0.0 → 2.0.0): breaking changes

> ⚠️ In `0.x.y` versions, MINOR bumps (`0.1.0` → `0.2.0`) are considered
> breaking. This is a Cargo convention.

## Publishing to crates.io

1. `cargo login <token>` — save your API token (one-time).
2. `cargo publish --dry-run` — verify everything.
3. `cargo publish` — upload.

Required fields: `name`, `version`, `description`, `license`.

Once published, **you cannot reuse a version number**. Bump the version
for every new release.

## Cargo.lock

* **Binaries**: commit `Cargo.lock` to git (reproducible builds).
* **Libraries**: don't commit it (let downstream apps resolve).

## Common Patterns

### Workspace with Shared Dependencies

```toml
# workspace Cargo.toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[workspace]
members = ["cli", "lib"]
```

In a member's `Cargo.toml`:

```toml
[dependencies]
serde = { workspace = true }
```

### Custom Build with `cargo-features`

```toml
cargo-features = ["named-features"]

[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[features]
my_feature = []
```

### Multi-Binary Project

```toml
[[bin]]
name = "server"
path = "src/server/main.rs"

[[bin]]
name = "client"
path = "src/client/main.rs"
```

Run with: `cargo run --bin server`.

## Useful Cargo Subcommands (install with `cargo install`)

| Tool | Purpose |
|------|---------|
| `cargo-edit` | `cargo add` / `cargo rm` (now built-in) |
| `cargo-watch` | auto-rebuild on file changes |
| `cargo-expand` | show macro-expanded code |
| `cargo-outdated` | show outdated dependencies |
| `cargo-audit` | scan for known vulnerabilities |
| `cargo-bloat` | find what's bloating your binary |
| `cargo-udeps` | find unused dependencies |
| `cargo-nextest` | faster test runner |
| `cargo-deny` | lint licenses / vulnerabilities |
| `cargo-flamegraph` | profile with flamegraphs |

**Next:** [Collections (in depth)](../48_collections) — `BTreeMap`, `HashSet`, `VecDeque`, etc.
