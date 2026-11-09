# Rust Intro

## What is Rust?

Rust is a **systems programming language** that runs blazingly fast, prevents
segfaults, and guarantees thread safety. It was originally sponsored by
Mozilla and first released in 2010. The first stable release (1.0) shipped
in May 2015.

Rust combines the **performance** of low-level languages like C and C++ with
the **safety** of high-level languages like Python. It achieves this with a
unique system called **ownership** that the compiler checks at compile time —
so there is **no runtime cost** for the safety guarantees.

## Why Use Rust?

* **Performance** — Rust is as fast as C and C++, with no garbage collector.
* **Memory safety** — the compiler prevents dangling pointers, double frees,
  and data races.
* **Concurrency** — fearless multi-threading: the compiler proves your
  concurrent code is data-race free.
* **Zero-cost abstractions** — high-level features (iterators, generics,
  traits) compile down to the same machine code as hand-written low-level code.
* **Great tooling** — `cargo` (build tool + package manager), `rustfmt`
  (formatter), `clippy` (linter), and excellent compiler error messages.

## What is Rust Used For?

| Domain | Example |
|--------|---------|
| Systems programming | Operating systems, device drivers |
| Web browsers | Parts of Firefox (Servo engine) |
| Command-line tools | `ripgrep`, `fd`, `bat`, `exa` |
| Web servers | Actix, Axum, Rocket frameworks |
| WebAssembly | High-performance code running in the browser |
| Embedded | Microcontrollers and IoT devices |
| Game engines | Bevy, Fyrox |

## Rust Hello World

The smallest useful Rust program:

```rust
fn main() {
    println!("Hello, World!");
}
```

* `fn` declares a function.
* `main` is the special function that always runs first.
* `println!` is a **macro** (note the `!`) that prints text to the screen.
* Strings inside `"..."` are string literals.
* Lines end with a semicolon `;`.

## Fun Facts

* Rust has been voted the **"most loved programming language"** in the
  Stack Overflow Developer Survey for many consecutive years.
* The Rust mascot is **Ferris the crab** 🦀.
* Rust's compile-time checks catch whole categories of bugs before your
  program ever runs.

**Next:** [Get Started](../01_getting_started) — install Rust and write your
first program.
