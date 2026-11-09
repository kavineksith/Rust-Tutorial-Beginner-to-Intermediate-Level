# Idiomatic Rust & Best Practices

The capstone chapter: how to write Rust like a Rustacean. Each pattern
shows the non-idiomatic way followed by the idiomatic way.

## 1. Use Iterators Instead of Loops

```rust
// ❌ Non-idiomatic:
let mut sum = 0;
for n in &nums {
    if n % 2 == 0 { sum += n * n; }
}

// ✅ Idiomatic:
let sum: i32 = nums.iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * n)
    .sum();
```

Iterators are zero-cost — they compile down to the same machine code as
the loop. Use them.

## 2. Use `?` Instead of Match-on-Result

```rust
// ❌ Non-idiomatic:
fn parse(s: &str) -> Result<i32, String> {
    let n = match s.parse() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };
    Ok(n * 2)
}

// ✅ Idiomatic:
fn parse(s: &str) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|e| e.to_string())?;
    Ok(n * 2)
}
```

## 3. Use `if let` for Single-Arm Matches

```rust
// ❌ Non-idiomatic:
match maybe {
    Some(n) => println!("{}", n),
    _ => {}
}

// ✅ Idiomatic:
if let Some(n) = maybe {
    println!("{}", n);
}
```

## 4. Use the Builder Pattern

For types with many optional fields:

```rust
let request = RequestBuilder::new()
    .url("https://example.com")
    .method("GET")
    .header("Accept", "application/json")
    .timeout_ms(5000)
    .build();
```

The builder has chainable methods returning `Self`, and a final `build()`
that returns the constructed object.

## 5. Use the Newtype Pattern

Wrap a primitive to give it a distinct type:

```rust
struct Meters(f64);
struct Seconds(f64);

let d = Meters(5.0);
let t = Seconds(2.0);
let speed = d.0 / t.0;
// let bad = d + t;   // ERROR: can't add Meters and Seconds
```

Prevents a whole class of bugs where two values with the same primitive
type are accidentally mixed up.

## 6. Use `&str` Instead of `&String` in Parameters

```rust
// ❌ Non-idiomatic:
fn greet(name: &String) { /* ... */ }

// ✅ Idiomatic:
fn greet(name: &str) { /* ... */ }
```

`&str` accepts `&str`, `&String`, `&&str`, and more. More flexible.

Same for slices: prefer `&[T]` over `&Vec<T>`.

## 7. Don't `unwrap` in Production Code

```rust
// ❌ Panics on Err:
let n: i32 = "abc".parse().unwrap();

// ✅ Handle the error:
let n: i32 = "42".parse().unwrap_or(0);

// ✅ Or propagate with ?:
let n: i32 = s.parse()?;
```

`unwrap` / `expect` are fine in tests and in places where failure is
truly impossible (proved by invariants). Use them sparingly.

## 8. Derive Boilerplate Traits

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User { /* ... */ }
```

Never hand-write `Debug`, `Clone`, `PartialEq` unless you have a special
reason.

## 9. Return `impl Trait` for Opaque Types

```rust
fn evens(up_to: u32) -> impl Iterator<Item = u32> {
    (0..up_to).filter(|x| x % 2 == 0)
}
```

Hides the concrete iterator type from the caller.

## 10. Use `clippy` — the Rust Linter

```bash
cargo clippy
cargo clippy --fix
```

Common catches:

| Non-idiomatic | Idiomatic |
|---------------|-----------|
| `if x == true` | `if x` |
| `if x == false` | `if !x` |
| `for i in 0..v.len() { v[i] }` | `for x in &v` |
| `String::from("x").as_str()` | `"x"` |
| `.map(\|x\| x + 1).collect::<Vec<_>>().len()` | `.map(\|x\| x + 1).count()` |
| `let _ = x;` (when you mean to drop) | `drop(x);` |

## 11. Module Organization — One Concept Per File

```
src/
├── main.rs             # binary entry point
├── lib.rs              # library entry point (if any)
├── config.rs           # config loading
├── network/
│   ├── mod.rs          # `pub mod client; pub mod server;`
│   ├── client.rs
│   └── server.rs
└── storage/
    ├── mod.rs
    └── sqlite.rs
```

Keep files focused. A 2000-line file is a smell.

## 12. Library vs Application Error Handling

```rust
// ✅ Library: return Result, don't log
pub fn fetch(url: &str) -> Result<String, MyError> { /* ... */ }

// ✅ Application: log errors at the top, then exit
fn main() -> anyhow::Result<()> {
    let body = fetch("...").context("failed to fetch")?;
    println!("{}", body);
    Ok(())
}
```

Libraries should NEVER call `env_logger::init()` or force a logging
backend on the user.

## 13. Test in the Same File as the Code

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Unit tests live next to the code. Integration tests live in `tests/`.

## 14. Use `cargo fmt`

Don't argue about style. Run `cargo fmt` before every commit.

Configure in `rustfmt.toml` if you want specific options.

## 15. Document Public APIs

```rust
/// Adds two integers.
///
/// # Examples
///
/// ```
/// let n = my_crate::add(2, 3);
/// assert_eq!(n, 5);
/// ```
///
/// # Panics
///
/// This function never panics.
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

The doc-comment becomes HTML documentation via `cargo doc`. Code blocks
become doctests run by `cargo test`.

## 16. Avoid `clone` When You Can Borrow

```rust
// ❌ Non-idiomatic:
let names: Vec<String> = strings.iter().cloned().collect();

// ✅ Idiomatic (when you only need to read):
let names: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
```

Borrowing is free; cloning allocates.

## 17. Prefer Generics Over `Box<dyn Trait>`

```rust
// ❌ Dynamic dispatch (slight overhead):
fn process(items: Vec<Box<dyn Display>>) { /* ... */ }

// ✅ Static dispatch (zero overhead):
fn process<T: Display>(items: Vec<T>) { /* ... */ }
```

Use `Box<dyn Trait>` only when you need heterogeneous collections (many
different types in one container).

## 18. Use `anyhow` for Apps, `thiserror` for Libraries

```rust
// Application (binary):
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config")
        .context("failed to read config")?;
    Ok(())
}

// Library:
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
```

## 19. Concurrency — Use `Arc<Mutex<T>>` for Shared Mutable State

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            *c.lock().unwrap() += 1;
        }
    }));
}
for h in handles { h.join().unwrap(); }

println!("{}", *counter.lock().unwrap());   // 10000
```

For async code, use `tokio::sync::Mutex` if the lock is held across
`.await` points.

## 20. Async — Use `tokio` for I/O, Threads for CPU

```rust
// Many concurrent I/O tasks: async + tokio
#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for url in urls {
        handles.push(tokio::spawn(fetch(url)));
    }
    for h in handles {
        let body = h.await.unwrap();
    }
}

// CPU-bound parallelism: threads or rayon
use rayon::prelude::*;
let results: Vec<_> = items.par_iter().map(process).collect();
```

## Common Crates to Learn

| Crate | Purpose |
|-------|---------|
| `serde` | serialization (JSON, YAML, ...) |
| `tokio` | async runtime |
| `clap` | CLI argument parsing |
| `anyhow` | application error handling |
| `thiserror` | library error types |
| `log` + `env_logger` | logging |
| `tracing` | structured logging |
| `rand` | random numbers |
| `chrono` / `time` | date / time |
| `regex` | regular expressions |
| `reqwest` | HTTP client |
| `axum` / `actix-web` | web frameworks |
| `sqlx` | async SQL |
| `rayon` | data parallelism |
| `crossbeam` | low-level concurrency |
| `once_cell` / `lazy_static` | global singletons |
| `itertools` | extra iterator helpers |
| `proptest` / `quickcheck` | property-based testing |
| `criterion` | benchmarks |
| `insta` | snapshot testing |
| `walkdir` / `ignore` | directory walking |
| `tempfile` | temp files / dirs |

## Final Checklist

Run through this before shipping:

- [ ] `cargo fmt` clean.
- [ ] `cargo clippy` clean (no warnings).
- [ ] No `unwrap()` / `expect()` in production paths (except where failure is truly impossible).
- [ ] Use `?` for error propagation.
- [ ] Use iterators instead of explicit loops.
- [ ] Use `&str` / `&[T]` in function parameters (not `&String` / `&Vec<T>`).
- [ ] Derive `Debug` for all public types.
- [ ] Document public items with `///`.
- [ ] Write at least one unit test per public function.
- [ ] Write doctests for examples.
- [ ] Use `anyhow` for apps, `thiserror` for libraries.
- [ ] Prefer generics over `Box<dyn Trait>`.
- [ ] Don't clone when you can borrow.
- [ ] Keep `unsafe` blocks small and well-documented (with `// SAFETY:` comments).
- [ ] Use `#[cfg(test)] mod tests` for unit tests.
- [ ] Put integration tests in `tests/`.
- [ ] Keep public API small — most things should be private.
- [ ] Use the newtype pattern for distinct quantity types.
- [ ] No logging in libraries — return errors instead.
- [ ] Use `Arc<Mutex<T>>` for multi-thread shared mutable state.
- [ ] `tokio` for async I/O; threads / `rayon` for CPU-bound work.

## Anti-Patterns to Avoid

| Anti-pattern | Fix |
|--------------|-----|
| `unwrap()` everywhere | Use `?` or `.unwrap_or` |
| `Box<dyn Trait>` when generics work | Use generics |
| `.clone()` to satisfy the borrow checker | Refactor with lifetimes / borrows |
| `String` parameters when `&str` works | Use `&str` |
| Manually implementing `Debug` | `#[derive(Debug)]` |
| Logging inside a library | Return errors |
| `unsafe` without a `// SAFETY:` comment | Document the contract |
| Deeply nested `match` / `if` | Extract functions / use early returns |
| 1000-line functions | Split into smaller functions |
| Public fields when they should be private | Make fields private, expose methods |
| Comments that restate code | Comments should explain WHY, not WHAT |

## Where to Go from Here

You've finished the entire tutorial. Next steps:

1. **Build something real.** A CLI tool, a small web server, a parser —
   anything that exercises what you learned.
2. **Read other people's Rust.** Browse popular crates' source code on
   crates.io / GitHub.
3. **Contribute.** Find a `good-first-issue` on a project you use.
4. **Stay current.** Read "This Week in Rust", follow the Rust blog.
5. **Join the community.** [users.rust-lang.org](https://users.rust-lang.org),
   Discord, Reddit r/rust.

## The Rust Book

For deeper learning, the official free book is the canonical reference:

**"The Rust Programming Language"** — https://doc.rust-lang.org/book/

Topics worth studying next:
- Advanced traits (associated types, supertraits, fully-qualified syntax)
- Advanced lifetimes (lifetime subtyping, HRTB)
- Macros (procedural macros in depth)
- `Pin` and self-referential structs
- Unsafe in depth (Stacked Borrows / Tree Borrows model)
- Async internals (executors, wakers, pinning)
- Embedded Rust (`no_std`, hardware abstraction)

---

🎉 **Congratulations — you've completed the entire 51-chapter Rust tutorial!**

From Hello World to async, FFI, and idiomatic patterns — you now have the
foundation to build real, production-quality Rust applications. The Rust
compiler is your pair-programmer: it catches bugs before they ship, and it
will only get friendlier as you learn its language.

**Go forth and build fearlessly.** 🦀
