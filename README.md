# 🦀 Rust Tutorial — Complete 51-Chapter Course

> A complete, hands-on Rust course — from Hello World to async, FFI, and idiomatic Rust best practices. An original curriculum covering the full ecosystem you need to write real, production-quality Rust applications.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/)
[![Chapters](https://img.shields.io/badge/chapters-51-brightgreen.svg)](#table-of-contents)
[![Status](https://img.shields.io/badge/status-complete-success.svg)](#)

---

## 📖 Table of Contents

- [About](#about)
- [Features](#features)
- [Quick Start](#quick-start)
- [Table of Contents](#table-of-contents-1)
- [How to Use This Tutorial](#how-to-use-this-tutorial)
- [Learning Path](#learning-path)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Resources & Further Reading](#resources--further-reading)
- [License](#license)

---

## About

This repository is a **complete, self-paced Rust course** in 51 chapters. It is an original curriculum that takes you from your first `Hello, World!` through ownership, async/await, smart pointers, FFI, and idiomatic Rust patterns.

The chapters are organized into **thematic modules** that build on each other: start with the basics, move through Rust's unique ownership model, explore data structures and abstraction, then tackle concurrency, the ecosystem, and best practices.

Every chapter contains:

1. **`notes.md`** — concise, beginner-friendly notes explaining the concept with runnable examples and comparison tables.
2. **`main.rs`** — a single, self-contained, runnable Rust source file with rich inline comments walking you through every line.

All 51 `main.rs` files **compile cleanly** with `rustc --edition 2024` — no `cargo`, no external crates required for the core chapters. Advanced chapters (Serde, Logging, FFI) include self-contained std-only demos plus extensive notes showing the idiomatic crate-based setup for real projects.

---

## Features

✅ **51 chapters** covering the entire Rust language and ecosystem

✅ **Every code example compiles and runs** — verified with Rust 1.85+ (edition 2024)

✅ **No external dependencies** for the core chapters — just `rustc`

✅ **Rich inline comments** explaining every line of every program

✅ **Markdown notes** with tables, examples, and "Try it Yourself" prompts

✅ **Progressive difficulty** — from Hello World to async, FFI, and idiomatic patterns

✅ **Self-contained advanced chapters** — async, serde, logging, FFI all work with plain `rustc`

✅ **Practice problems** with worked solutions (chapter 51)

✅ **MIT licensed** — use it, fork it, teach with it

✅ **Edition 2024** — uses the latest Rust edition

---

## Quick Start

### Prerequisites

Install Rust via [rustup](https://www.rust-lang.org/tools/install):

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows: download and run rustup-init.exe from the URL above
```

Verify the installation:

```bash
rustc --version    # needs to be 1.85 or later for edition 2024
cargo --version
```

### Clone and Run

```bash
git clone https://github.com/your-username/rust-tutorial.git
cd rust-tutorial

# Run any chapter:
cd 01_getting_started
rustc --edition 2024 main.rs
./main            # on Linux / macOS
.\main.exe        # on Windows
```

Or, even simpler, compile and run in one command:

```bash
cd 04_variables && rustc --edition 2024 main.rs -o ./out && ./out
```

> **Tip**: For real projects, use `cargo` — see [chapter 01 (Getting Started)](01_getting_started) and [chapter 42 (Cargo & Build System)](42_cargo_build) for the full setup.

---

## Table of Contents

The curriculum is organized into **thematic modules**. Each module builds on the previous one.

### Module 1: Getting Started

| # | Chapter | Folder |
|---|---------|--------|
| 01 | Getting Started | [01_getting_started](01_getting_started) |
| 02 | Intro | [02_intro](02_intro) |
| 03 | Syntax | [03_syntax](03_syntax) |

### Module 2: Variables & Types

| # | Chapter | Folder |
|---|---------|--------|
| 04 | Variables | [04_variables](04_variables) |
| 05 | Data Types | [05_data_types](05_data_types) |
| 06 | Operators | [06_operators](06_operators) |
| 07 | Booleans | [07_booleans](07_booleans) |
| 08 | Constants | [08_constants](08_constants) |

### Module 3: Output & Documentation

| # | Chapter | Folder |
|---|---------|--------|
| 09 | Output | [09_output](09_output) |
| 10 | Comments | [10_comments](10_comments) |

### Module 4: Functions & Scope

| # | Chapter | Folder |
|---|---------|--------|
| 11 | Functions | [11_functions](11_functions) |
| 12 | Scope | [12_scope](12_scope) |

### Module 5: Control Flow

| # | Chapter | Folder |
|---|---------|--------|
| 13 | If..Else | [13_if_else](13_if_else) |
| 14 | Loops | [14_loops](14_loops) |
| 15 | While Loops | [15_while_loops](15_while_loops) |
| 16 | For Loops | [16_for_loops](16_for_loops) |
| 17 | Match | [17_match](17_match) |

### Module 6: Strings & Ownership

| # | Chapter | Folder |
|---|---------|--------|
| 18 | Strings | [18_strings](18_strings) |
| 19 | Ownership | [19_ownership](19_ownership) |
| 20 | Borrowing | [20_borrowing](20_borrowing) |
| 21 | Lifetimes (in depth) | [21_lifetimes](21_lifetimes) |

### Module 7: Data Structures

| # | Chapter | Folder |
|---|---------|--------|
| 22 | Data Structures (Overview) | [22_data_structures](22_data_structures) |
| 23 | Arrays | [23_arrays](23_arrays) |
| 24 | Vectors | [24_vectors](24_vectors) |
| 25 | Tuples | [25_tuples](25_tuples) |
| 26 | HashMap | [26_hashmap](26_hashmap) |
| 27 | Structs | [27_structs](27_structs) |
| 28 | Enums | [28_enums](28_enums) |

### Module 8: Abstraction & Types

| # | Chapter | Folder |
|---|---------|--------|
| 29 | Traits | [29_traits](29_traits) |
| 30 | Generics | [30_generics](30_generics) |
| 31 | Type Conversions | [31_type_conversions](31_type_conversions) |
| 32 | Common Traits (Copy/Clone/Drop/Default) | [32_common_traits](32_common_traits) |
| 33 | Pattern Matching (in depth) | [33_pattern_matching](33_pattern_matching) |
| 34 | Closures & Iterators | [34_closures_iterators](34_closures_iterators) |

### Module 9: Memory & Smart Pointers

| # | Chapter | Folder |
|---|---------|--------|
| 35 | Smart Pointers | [35_smart_pointers](35_smart_pointers) |
| 36 | Macros | [36_macros](36_macros) |

### Module 10: Error Handling & Testing

| # | Chapter | Folder |
|---|---------|--------|
| 37 | Error Handling | [37_error_handling](37_error_handling) |
| 38 | Testing | [38_testing](38_testing) |

### Module 11: Concurrency & Async

| # | Chapter | Folder |
|---|---------|--------|
| 39 | Concurrency | [39_concurrency](39_concurrency) |
| 40 | Async / Await | [40_async](40_async) |

### Module 12: Project Organization

| # | Chapter | Folder |
|---|---------|--------|
| 41 | Modules & Crates | [41_modules_crates](41_modules_crates) |
| 42 | Cargo & Build System | [42_cargo_build](42_cargo_build) |

### Module 13: I/O & Applications

| # | Chapter | Folder |
|---|---------|--------|
| 43 | File I/O & std::fs | [43_file_io](43_file_io) |
| 44 | Command-Line Apps | [44_cli_apps](44_cli_apps) |
| 45 | Serde & Serialization | [45_serde](45_serde) |
| 46 | Logging & Tracing | [46_logging](46_logging) |

### Module 14: Advanced & Unsafe

| # | Chapter | Folder |
|---|---------|--------|
| 47 | unsafe Rust & FFI | [47_unsafe_ffi](47_unsafe_ffi) |
| 48 | Collections (in depth) | [48_collections](48_collections) |
| 49 | Standard Library Tour | [49_std_tour](49_std_tour) |

### Module 15: Mastery & Practice

| # | Chapter | Folder |
|---|---------|--------|
| 50 | Idiomatic Rust & Best Practices | [50_idiomatic_rust](50_idiomatic_rust) |
| 51 | Practice Problems | [51_practice](51_practice) |

---

## How to Use This Tutorial

Each chapter is a self-contained folder with two files:

```
04_variables/
├── notes.md      # Read this first — concept explanation + examples
└── main.rs       # Then run this — full code with inline comments
```

### Recommended workflow

1. **Read `notes.md` first** to understand the concept.
2. **Open `main.rs`** and read the inline comments — they walk through every line.
3. **Compile and run**:
   ```bash
   cd 04_variables
   rustc --edition 2024 main.rs -o ./out && ./out
   ```
4. **Modify the code** — break things, fix them, experiment. The inline comments tell you what to try.
5. **Move to the next chapter** when you can explain the output without looking.

### A note on advanced chapters

Some advanced chapters (Serde, Logging, FFI, Async) include **self-contained demos that compile with plain `rustc`** — no external crates needed. The `notes.md` for those chapters also shows the **idiomatic crate-based setup** (using `serde`, `tokio`, `clap`, etc.) that you'd use in real production projects.

---

## Learning Path

### 🚀 Beginner (Modules 1–7, Chapters 1–28)

Start with **01_getting_started** to set up your environment. Work through Modules 1–7 in order — each builds on the previous. You'll learn the basics, then Rust's unique ownership model, then data structures.

### 🔥 Intermediate (Modules 8–11, Chapters 29–40)

Move on to **abstraction** (traits, generics, pattern matching), **memory** (smart pointers, macros), **error handling & testing**, and **concurrency & async** — the concepts you need to write real Rust applications.

### 🏆 Advanced & Mastery (Modules 12–15, Chapters 41–51)

Finish with **project organization** (cargo, modules), **I/O & applications** (file I/O, CLI, serde, logging), **advanced topics** (unsafe, FFI, collections, std tour), and finally **idiomatic Rust best practices** and **practice problems**.

### Estimated time

- **Beginner**: ~20 hours (1 hour per chapter)
- **Intermediate**: ~15 hours (1.5 hours per chapter)
- **Advanced & Mastery**: ~15 hours (1.5 hours per chapter)
- **Total**: ~50 hours of focused study

---

## Project Structure

```
rust-tutorial/
├── README.md                  # This file
├── LICENSE                    # MIT license
├── CONTRIBUTING.md            # How to contribute
├── RESOURCES.md               # Books, links, and further reading
├── .gitignore                 # Rust gitignore
│
├── 01_getting_started/
│   ├── notes.md
│   └── main.rs
├── 02_intro/
│   ├── notes.md
│   └── main.rs
├── ...
├── 50_idiomatic_rust/
│   ├── notes.md
│   └── main.rs
└── 51_practice/
    ├── notes.md
    └── main.rs
```

Each chapter folder is independent — you can study them in any order after the beginner modules.

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- 🐛 Reporting bugs or broken examples
- 💡 Suggesting new chapters or improvements
- ✏️ Fixing typos and clarifying explanations
- 🌍 Translating notes to other languages
- 🎓 Using this material to teach

We also maintain a list of **Rust open-source projects with `good-first-issue` labels** so you can practice contributing to real Rust codebases — see [CONTRIBUTING.md](CONTRIBUTING.md#practice-with-real-open-source-projects).

---

## Resources & Further Reading

This tutorial is a starting point. For deeper mastery, see [RESOURCES.md](RESOURCES.md) for:

- 📕 **The Rust Programming Language** (the official free book)
- 📗 Rust By Example
- 📘 Rustlings (interactive exercises)
- 🌐 The async book
- 🎥 Recommended talks and conference videos
- 📦 Curated crate recommendations
- 🏛️ The Rust reference and standard library docs

**The single most important next step** after finishing this tutorial is to read **[The Rust Programming Language](https://doc.rust-lang.org/book/)** — the official, free, comprehensive book. It covers everything here in more depth, plus advanced topics like advanced trait features, advanced lifetimes, and the type system in detail.

---

## License

This project is licensed under the **MIT License** — see [LICENSE](LICENSE) for the full text.

You are free to:

- ✅ Use this material for personal learning
- ✅ Use it in classrooms and workshops
- ✅ Fork it and adapt it
- ✅ Distribute it
- ✅ Commercial use

Just keep the copyright notice and license text.

---

## Acknowledgements

- Built on the excellent [Rust documentation](https://doc.rust-lang.org/) and [The Rust Programming Language](https://doc.rust-lang.org/book/) book
- Thanks to the entire Rust community for making Rust so welcoming to newcomers 🦀

---

<p align="center">
  <strong>Happy Rusting! 🦀</strong>
</p>

<p align="center">
  <sub>Built with ❤️ for the Rust community. If this tutorial helped you, consider ⭐ starring the repo.</sub>
</p>
