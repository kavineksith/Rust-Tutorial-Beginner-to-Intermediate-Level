# Resources & Further Reading

This tutorial is a starting point. Rust is a deep language with a rich
ecosystem — here's where to go next to take your skills further.

## 📖 Table of Contents

- [The Single Most Important Next Step](#the-single-most-important-next-step)
- [Official Books](#official-books)
- [Interactive Learning](#interactive-learning)
- [Reference Documentation](#reference-documentation)
- [Async Rust](#async-rust)
- [Video & Talks](#video--talks)
- [Blogs & Newsletters](#blogs--newsletters)
- [Community](#community)
- [Curated Crate Recommendations](#curated-crate-recommendations)
- [Advanced Topics to Study Next](#advanced-topics-to-study-next)

---

## The Single Most Important Next Step

### 📕 [The Rust Programming Language](https://doc.rust-lang.org/book/)

**Affectionately known as "the Book."** This is the official, free,
comprehensive Rust textbook, written by the Rust team. It is the canonical
reference for the language.

> **Read this after finishing this tutorial.** It covers everything here
> in more depth, plus advanced topics like advanced trait features,
> advanced lifetimes, the type system in detail, and real-world project
> patterns.

- **Free online**: <https://doc.rust-lang.org/book/>
- **Print edition**: "The Rust Programming Language" by Steve Klabnik and
  Carol Nichols (No Starch Press)
- **Translations**: [available in many languages](https://github.com/rust-lang/book#translations)

#### Most Important Chapters After This Tutorial

| Book chapter | Why it matters |
|--------------|----------------|
| [Ch. 10 — Generics, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) | Deepens chapters 29, 30, 33 |
| [Ch. 15 — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | Deepens chapter 36 — covers `Rc`, `RefCell`, `Drop` in depth |
| [Ch. 16 — Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | Deepens chapter 37 |
| [Ch. 17 — Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html) | Deepens chapter 34 |
| [Ch. 19 — Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html) | Advanced traits, macros, `unsafe`, and more |
| [Ch. 20 — A Final Project (Multithreaded Web Server)](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) | Bring it all together |

---

## Official Books

| Book | Link | Description |
|------|------|-------------|
| 📕 **The Rust Programming Language** | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) | The canonical book — read this |
| 📗 **Rust By Example** | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) | Code-first learning — every concept shown as a runnable example |
| 📘 **The Rust Reference** | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) | The language specification — for when you need precise semantics |
| 📙 **The Rustonomicon** | [doc.rust-lang.org/nomicon](https://doc.rust-lang.org/nomicon/) | The dark arts of `unsafe` Rust — read before writing unsafe code |
| 📒 **The Unstable Book** | [doc.rust-lang.org/nightly/unstable-book](https://doc.rust-lang.org/nightly/unstable-book/) | Nightly-only features |
| 📓 **The Edition Guide** | [doc.rust-lang.org/edition-guide](https://doc.rust-lang.org/edition-guide/) | Differences between Rust 2015 / 2018 / 2021 / 2024 |
| 📕 **The Cargo Book** | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) | Everything about Cargo — deepens chapter 40 |
| 📘 **The rustdoc Book** | [doc.rust-lang.org/rustdoc](https://doc.rust-lang.org/rustdoc/) | How to write and generate documentation |
| 📗 **The Clippy Book** | [doc.rust-lang.org/clippy](https://doc.rust-lang.org/clippy/) | The linter — what it catches and how to configure it |
| 📙 **The rustc Book** | [doc.rust-lang.org/rustc](https://doc.rust-lang.org/rustc/) | The compiler directly — flags, codegen options |

---

## Interactive Learning

| Resource | Link | Description |
|----------|------|-------------|
| **Rustlings** | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) | Small exercises to fix broken Rust code — great for muscle memory |
| **Rust By Practice** | [practice.rs](https://practice.rs/) | Interactive exercises with instant feedback |
| **Exercism Rust Track** | [exercism.org/tracks/rust](https://exercism.org/tracks/rust) | Mentor-reviewed exercises |
| **CodeCrafters Rust** | [codecrafters.io](https://codecrafters.io/) | Build Redis, Git, SQLite from scratch in Rust |
| **Advent of Code** | [adventofcode.com](https://adventofcode.com/) | Annual coding puzzles — many people solve them in Rust |

### Rustlings Quick Start

```bash
cargo install rustlings
rustlings
# Follow the on-screen prompts
```

---

## Reference Documentation

| Resource | Link |
|----------|------|
| Standard library docs | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) |
| Cargo docs | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) |
| Crates.io (package registry) | [crates.io](https://crates.io) |
| docs.rs (auto-generated crate docs) | [docs.rs](https://docs.rs) |
| lib.rs (curated crate index) | [lib.rs](https://lib.rs) |

### Pro Tip

Whenever you're reading a crate's documentation, type `?` on docs.rs to
bring up a search box. You can search any item in any crate instantly.

---

## Async Rust

Async is one of the most powerful and complex parts of Rust. These
resources go deep:

| Resource | Link | Description |
|----------|------|-------------|
| **Asynchronous Programming in Rust** | [rust-lang.github.io/async-book](https://rust-lang.github.io/async-book/) | The official async book |
| **Tokio Tutorial** | [tokio.rs/tokio/tutorial](https://tokio.rs/tokio/tutorial) | Building a Redis clone with tokio |
| **Async Rust cheat-sheet** | [github.com/tokio-rs/tokio/blob/master/tokio-docs/...](https://github.com/tokio-rs/tokio/) | Quick reference |

---

## Video & Talks

### YouTube Channels

- **[Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty)** — tutorials
  and news
- **[Jon Gjengset](https://www.youtube.com/c/JonGjengset)** — deep dives
  for intermediate/advanced Rust
- **[No Boilerplate](https://www.youtube.com/c/NoBoilerplate)** — short,
  punchy videos on Rust concepts
- **[Faster Than Lime](https://www.youtube.com/@fasterthanlime)** —
  in-depth explorations

### Conference Talks

- **[RustConf](https://www.youtube.com/c/RustFoundation)** — the main Rust
  conference
- **[Rust Belt Rust](https://www.youtube.com/playlist?list=PLy60M3Z0GNL8sUdDOLgJOKO2rh0RmhZTO)** —
  East-coast US Rust conference
- **[EuroRust](https://www.youtube.com/@eurorust)** — European Rust
  conference

### Specific Recommended Talks

- *"Rust: A Language for the Next 40 Years"* — Steve Klabnik
- *"Fearless Concurrency"* — Aaron Turon
- *"Scaling Rust"* — Tyler Neely
- *"Designing APIs in Rust"* — Pascal Hertlei
- *"Why Rust?"* — Multiple authors

---

## Blogs & Newsletters

| Blog / Newsletter | Link | Focus |
|--------------------|------|-------|
| **This Week in Rust** | [this-week-in-rust.org](https://this-week-in-rust.org/) | Weekly Rust news digest |
| **Rust Blog (official)** | [blog.rust-lang.org](https://blog.rust-lang.org/) | Official announcements |
| **Inside Rust Blog** | [blog.rust-lang.org/inside-rust](https://blog.rust-lang.org/inside-rust/) | Internals for contributors |
| **Faster Than Lime** | [fasterthanli.me](https://fasterthanli.me/) | Deep technical articles |
| **Loris's blog** | [loris.rocks](https://loris.rocks/) | Async Rust internals |
| **Jakob's blog** | [jakobmeier.ch/blogging](https://blog.jakob-meier.ch/) | Rust ecosystem analysis |
| **r/rust** | [reddit.com/r/rust](https://www.reddit.com/r/rust/) | Community discussion |

---

## Community

| Place | Link | What it's good for |
|-------|------|---------------------|
| **Users Forum** | [users.rust-lang.org](https://users.rust-lang.org) | Asking questions |
| **Internals Forum** | [internals.rust-lang.org](https://internals.rust-lang.org) | Discussing language design |
| **Rust Discord** | [discord.gg/rust-lang](https://discord.gg/rust-lang) | Real-time chat |
| **Rust Zulip** | [rust-lang.zulipchat.com](https://rust-lang.zulipchat.com) | Working group discussions |
| **Mastodon** | [@rustlang@hachyderm.io](https://hachyderm.io/@rustlang) | Official announcements |
| **r/rust** | [reddit.com/r/rust](https://www.reddit.com/r/rust/) | News and discussion |
| **Stack Overflow** | [stackoverflow.com/questions/tagged/rust](https://stackoverflow.com/questions/tagged/rust) | Q&A |
| **Crates.io** | [crates.io](https://crates.io) | Package registry |
| **Are We Learning Rust Yet?** | [github.com/pretzelhammer/rust-blog](https://github.com/pretzelhammer/rust-blog) | Community blogroll |

---

## Curated Crate Recommendations

After finishing this tutorial, these are the crates you'll likely reach for:

### Serialization

| Crate | Use |
|-------|-----|
| `serde` | The serialization framework (chapter 47) |
| `serde_json` | JSON |
| `serde_yaml` | YAML |
| `toml` | TOML |
| `bincode` | Compact binary |

### Async

| Crate | Use |
|-------|-----|
| `tokio` | The dominant async runtime (chapter 34) |
| `async-std` | Alternative runtime mirroring std |
| `smol` | Small, simple runtime |
| `futures` | Future / Stream combinators |
| `reqwest` | Async HTTP client |
| `hyper` | Low-level async HTTP |
| `axum` | Async web framework |
| `actix-web` | Async web framework (actor-based) |
| `sqlx` | Async SQL |
| `tokio-tungstenite` | Async WebSocket |

### CLI

| Crate | Use |
|-------|-----|
| `clap` | Argument parsing (chapter 46) |
| `colored` | Colored terminal output |
| `indicatif` | Progress bars |
| `dialoguer` | Interactive prompts |
| `console` | Terminal utilities |
| `ctrlc` | Handle Ctrl-C |
| `dirs` | Standard config/cache dirs |
| `anyhow` | Application error handling |

### Concurrency

| Crate | Use |
|-------|-----|
| `rayon` | Data parallelism (chapter 37) |
| `crossbeam` | Low-level concurrency primitives |
| `parking_lot` | Faster Mutex / RwLock |
| `dashmap` | Concurrent HashMap |

### Web

| Crate | Use |
|-------|-----|
| `axum` | Web framework (tokio-based) |
| `actix-web` | Web framework (actor-based) |
| `rocket` | Web framework (ergonomic) |
| `warp` | Web framework (composable filters) |
| `tower` | Service middleware |
| `tower-http` | HTTP middleware |

### Database

| Crate | Use |
|-------|-----|
| `sqlx` | Async SQL with compile-time checks |
| `diesel` | ORM (sync) |
| `sea-orm` | Async ORM |
| `rusqlite` | SQLite bindings |
| `redis` | Redis client |

### Error Handling

| Crate | Use |
|-------|-----|
| `anyhow` | Application errors (chapter 31) |
| `thiserror` | Library errors |
| `miette` | Fancy diagnostic reports |

### Logging / Tracing

| Crate | Use |
|-------|-----|
| `log` | Logging facade (chapter 48) |
| `env_logger` | Simple backend |
| `tracing` | Structured, async-aware logging |
| `tracing-subscriber` | Subscribers for tracing |
| `slog` | Structured, hierarchical (older) |

### Date / Time

| Crate | Use |
|-------|-----|
| `chrono` | Date and time (most popular) |
| `time` | Newer, simpler alternative |

### Randomness

| Crate | Use |
|-------|-----|
| `rand` | Random numbers |
| `fastrand` | Simple, fast PRNG |

### Testing

| Crate | Use |
|-------|-----|
| `proptest` | Property-based testing (chapter 38) |
| `quickcheck` | Property-based testing (simpler) |
| `insta` | Snapshot testing |
| `mockall` | Mocking |
| `criterion` | Benchmarks (stable) |
| `rstest` | Parameterized tests |
| `pretty_assertions` | Nicer assertion diffs |

### Filesystem

| Crate | Use |
|-------|-----|
| `tempfile` | Temp files and dirs (chapter 45) |
| `walkdir` | Recursive directory walking |
| `ignore` | Like walkdir, respects .gitignore |
| `notify` | Filesystem event watching |
| `memmap2` | Memory-mapped files |

### Macros

| Crate | Use |
|-------|-----|
| `syn` | Parse Rust syntax (chapter 39) |
| `quote` | Generate Rust code |
| `proc-macro2` | Work with tokens outside proc-macro context |

### Cryptography

| Crate | Use |
|-------|-----|
| `sha2` / `sha3` | Hash functions |
| `ring` | High-level crypto |
| `rustls` | TLS implementation |
| `argon2` / `bcrypt` | Password hashing |

### Data Structures

| Crate | Use |
|-------|-----|
| `indexmap` | HashMap that preserves insertion order |
| `petgraph` | Graph data structures |
| `slotmap` | Slot map (stable keys) |
| `im` | Immutable data structures |

### Functional

| Crate | Use |
|-------|-----|
| `itertools` | Extra iterator helpers |
| `pipe-trait` | Pipe-style method chaining |

---

## Advanced Topics to Study Next

After finishing this tutorial and the Book, these topics will deepen your
expertise:

### 1. Advanced Trait Features

- Associated types vs generics
- Supertraits (`trait B: A`)
- Fully-qualified syntax (`<Type as Trait>::method()`)
- The newtype pattern for trait coherence
- Default trait method overrides

📖 [The Book, Ch. 19.3 — Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

### 2. Advanced Lifetimes

- Lifetime subtyping (`'a: 'b`)
- Higher-ranked trait bounds (`for<'a>`)
- Lifetime bounds on generic types (`T: 'a`)
- GATs (Generic Associated Types)

📖 [The Book, Ch. 19.2 — Advanced Lifetimes](https://doc.rust-lang.org/book/ch19-02-advanced-lifetimes.html)
📖 [The Nomicon — Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)

### 3. Macros in Depth

- Declarative macro hygiene
- Procedural macros (derive, attribute, function-like)
- The `syn` / `quote` / `proc-macro2` ecosystem
- Building your own `derive` macro

📖 [The Book, Ch. 19.5 — Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
📖 [The Rust Reference — Macros](https://doc.rust-lang.org/reference/macros.html)

### 4. `unsafe` Rust In Depth

- The Rust memory model (Stacked Borrows / Tree Borrows)
- When `unsafe` is necessary and when it's not
- Writing safe wrappers around `unsafe` code
- Miri: the UB detector
- Formal verification tools (Prusti, Kani)

📖 [The Nomicon](https://doc.rust-lang.org/nomicon/)
📖 [Miri](https://github.com/rust-lang/miri)

### 5. Async Internals

- How `async fn` desugars to state machines
- `Pin` and self-referential structs
- Implementing your own executor
- The `Future` trait in depth
- Async drop

📖 [The Async Book](https://rust-lang.github.io/async-book/)
📖 [Jon Gjengset — "Demystifying Async Rust"](https://www.youtube.com/watch?v=J__1Uj6LoNM)

### 6. Embedded Rust

- `no_std` programming
- The embedded HAL
- Cross-compiling to microcontrollers
- Real-time operating systems (RTIC, Embassy)

📖 [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

### 7. WebAssembly

- Compiling Rust to `wasm32-unknown-unknown`
- `wasm-bindgen` for JS interop
- `wasm-pack` for packaging
- frameworks: Yew, Leptos, Dioxus

📖 [The wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

### 8. Compiler Internals

- The MIR (Mid-level IR)
- The borrow checker's algorithm
- Monomorphization
- LLVM codegen

📖 [The rustc dev guide](https://rustc-dev-guide.rust-lang.org/)

### 9. Rust for Linux Kernel

- The Rust-for-Linux project
- Kernel bindings
- Driver writing

📖 [rust-for-linux.com](https://rust-for-linux.com/)

### 10. Formal Methods

- Prusti (verification with Viper)
- Kani (model checker)
- Aeneas (verification with Lean)

📖 [verification-tools.github.io](https://verification-tools.github.io/)

---

## Final Words

Rust rewards deep learning. The compiler is your pair-programmer — every
error message is a chance to understand the language better. Don't rush;
take the time to understand *why* the borrow checker is complaining, and
you'll come out the other side a much stronger programmer.

> **"Rust makes you a better programmer, even if you never write Rust again."**
> — common saying in the community

**Welcome to the Rust community.** 🦀

---

[← Back to README](README.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE)
