# Rust Modules & Crates

As programs grow, you need to organize code into **modules** and **crates**.
Rust's module system is explicit and powerful.

## The Hierarchy

```
Crate (compilation unit)
└── Module
    └── Submodule
        └── Item (function, struct, enum, ...)
```

* A **crate** is what `cargo build` produces — a binary or a library.
* A **module** groups related items inside a crate.
* An **item** is a function, struct, enum, constant, etc.

## Declaring a Module (Inline)

```rust
mod geometry {
    pub const PI: f64 = 3.141592653589793;

    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Circle { radius }
        }
        pub fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }
}
```

Items inside a module are **private by default**. Use `pub` to expose them.

## Visibility Keywords

| Keyword | Meaning |
|---------|---------|
| (nothing) | private — only visible in current module |
| `pub` | visible everywhere |
| `pub(crate)` | visible anywhere in this crate |
| `pub(super)` | visible in the parent module |
| `pub(in path)` | visible in a specific ancestor module |

## Modules in Separate Files

If you write `mod network;` (no body), Rust looks for the content in:

* `src/network.rs`, or
* `src/network/mod.rs`

Example layout:

```
src/
├── main.rs              // `mod network;` and `mod utils;`
├── network.rs           // `pub fn connect() { }`
├── utils/
│   ├── mod.rs           // `pub mod format;`
│   └── format.rs        // `pub fn pretty_print() { }`
```

In `main.rs`:

```rust
mod network;
mod utils;

fn main() {
    network::connect();
    utils::format::pretty_print();
}
```

## Paths

```rust
geometry::Circle::new(2.0);                 // absolute from crate root
crate::geometry::Circle::new(2.0);          // explicit crate-relative
self::geometry::Circle::new(2.0);           // relative to current module
super::some_function();                     // relative to parent module
```

| Keyword | Reference frame |
|---------|------------------|
| `crate::` | crate root |
| `self::` | current module |
| `super::` | parent module |

## `use` — Bring Items Into Scope

Without `use`, you must write the full path every time:

```rust
let c = geometry::Circle::new(2.0);
```

With `use`:

```rust
use geometry::Circle;
let c = Circle::new(2.0);
```

You can `use` modules, functions, structs, constants — anything with a path.

## Nested `use`

```rust
use math::{arithmetic::add, trigonometry::sin_deg};
```

## Renaming with `as`

```rust
use std::io::Result as IoResult;
```

## Glob Imports

```rust
use std::io::*;
```

Use sparingly — it can shadow names and make code harder to read. Common
acceptable use: `use std::prelude::*;` (which Rust does implicitly for some
modules).

## `pub use` — Re-exports

A library can re-export items to provide a clean public API:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn mul(a: i32, b: i32) -> i32 { a * b }

    pub use self::ops::power;
    mod ops {
        pub fn power(base: i32, exp: u32) -> i32 {
            (1..=exp).fold(1, |acc, _| acc * base)
        }
    }
}

// Users can call calculator::power even though it's nested deep inside.
```

## Crates

A **crate** is the compilation unit — what `cargo build` produces.

| Crate type | Has | Entry point |
|------------|-----|-------------|
| Binary | `src/main.rs` | `fn main()` |
| Library | `src/lib.rs` | (none — exposes items) |
| Workspace | `Cargo.toml` (root) | multiple sub-crates |

A single package can have **both** a binary and a library:

```
src/
├── main.rs    // binary entry point — can use `my_crate::function()`
└── lib.rs     // library code
```

## `Cargo.toml` — The Crate Manifest

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"
authors = ["You <you@example.com>"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
proptest = "1.0"
```

* `[dependencies]` — libraries your crate needs at runtime.
* `[dev-dependencies]` — only needed for tests/benchmarks.
* `features` — enable optional functionality in a dependency.

## Adding a Dependency

```bash
cargo add serde              # adds the latest version
cargo add tokio --features full
cargo add serde --version 1.0
```

## Using a Dependency

```rust
// In your code:
use serde::Serialize;

#[derive(Serialize)]
struct User { name: String, age: u32 }
```

## Workspaces — Multiple Crates Sharing Dependencies

For large projects:

```
my_workspace/
├── Cargo.toml          // [workspace] members = ["cli", "lib", "common"]
├── cli/
│   └── Cargo.toml      // depends on `lib` and `common`
├── lib/
│   └── Cargo.toml
└── common/
    └── Cargo.toml
```

All crates share a single `target/` directory and lockfile.

## The Standard Library (`std`)

The standard library is automatically available — you don't need to declare
it in `Cargo.toml`. Use `std::` paths directly:

```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
```

Some items are in the **prelude** — automatically imported into every
program (e.g. `Vec`, `String`, `Option`, `Result`, `Box`).

## Common Patterns

### Module Per Concept

```
src/
├── main.rs
├── config.rs        // config loading
├── network/
│   ├── mod.rs
│   ├── client.rs
│   └── server.rs
└── storage/
    ├── mod.rs
    └── sqlite.rs
```

### Flat Public API

Use `pub use` in `lib.rs` to flatten nested modules for users:

```rust
// lib.rs
mod network;
mod storage;

pub use network::Client;       // users write `my_crate::Client`
pub use storage::Database;     // users write `my_crate::Database`
```

## Editions

Rust ships in "editions" — `2015`, `2018`, `2021`, `2024`. The edition is
declared in `Cargo.toml`. Editions enable new syntax and stricter checks
without breaking old code. Always use the latest (`2021` as of Rust 1.99).

## Summary

| Concept | Purpose |
|---------|---------|
| Crate | compilation unit (binary or library) |
| `mod` | declare a module |
| `pub` | make an item visible outside its module |
| `use` | bring an item into scope |
| `pub use` | re-export an item |
| `crate::`, `self::`, `super::` | path keywords |
| `Cargo.toml` | crate manifest |
| `cargo add` | add a dependency |

**Next:** [Lifetimes](../21_lifetimes) — the borrow checker's most advanced feature.
