# Contributing to the Rust Tutorial

First off: **thank you for considering a contribution!** 🦀

This tutorial exists to help people learn Rust. Every contribution — typo fixes,
new examples, clearer explanations, translations — makes the Rust community
a little more welcoming.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Getting Started](#getting-started)
- [Contribution Workflow](#contribution-workflow)
- [Style Guidelines](#style-guidelines)
- [Practice with Real Open-Source Projects](#practice-with-real-open-source-projects)
- [Using This Tutorial to Teach](#using-this-tutorial-to-teach)
- [Questions?](#questions)

---

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
Please be kind, patient, and welcoming. We are all here to learn.

---

## Ways to Contribute

### 🐛 Report Bugs or Broken Examples

If a code example doesn't compile or a `notes.md` has incorrect information:

1. Check [existing issues](../../issues) to avoid duplicates.
2. Open a new issue with:
   - Chapter number and folder (e.g. `29_traits`)
   - What you expected vs. what happened
   - The exact error message (if applicable)
   - Your Rust version (`rustc --version`)

### 💡 Suggest New Chapters or Improvements

Have an idea for a new chapter, a better explanation, or an additional
example? Open an issue with the `enhancement` label and describe:

- What topic you want to add or improve
- Why it's valuable
- A rough outline (if you have one)

### ✏️ Fix Typos and Clarify Explanations

Spotted a typo? An unclear sentence? A confusing example? PRs are very
welcome — small fixes are just as valuable as big ones.

### 🌍 Translate Notes to Other Languages

If you'd like to translate the `notes.md` files into another language
(中文, Español, Français, 日本語, ...), open an issue first to coordinate
so we don't duplicate work. Suggested structure:

```
01_intro/
├── notes.md              # English (default)
├── notes.zh.md           # Chinese
├── notes.es.md           # Spanish
└── main.rs               # Code stays the same
```

### 📝 Add Practice Problems

Chapter 28 has 12 worked practice problems. We'd love more! Open an issue
to propose a new problem before writing it up.

---

## Getting Started

### Prerequisites

- Rust 1.85+ (install via [rustup](https://www.rust-lang.org/tools/install)) — required for edition 2024
- Git
- A text editor with Rust support ([rust-analyzer](https://rust-analyzer.github.io/) recommended)

### Setup

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR-USERNAME/rust-tutorial.git
cd rust-tutorial

# Add the upstream remote:
git remote add upstream https://github.com/ORIGINAL-OWNER/rust-tutorial.git

# Verify everything compiles:
for d in */; do
    d="${d%/}"
    if [ -f "$d/main.rs" ]; then
        rustc --edition 2024 "$d/main.rs" -o /tmp/test_"$d" 2>&1 | head -3
        echo "Checked $d"
    fi
done
```

### File Structure

Each chapter is a folder with exactly two files:

```
29_traits/
├── notes.md      # Markdown notes in beginner-friendly style
└── main.rs       # Self-contained Rust code with inline comments
```

**Both files matter.** The `notes.md` is the concept explanation; the
`main.rs` is the worked example. They should reference each other.

---

## Contribution Workflow

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally.
3. **Create a branch** for your change:
   ```bash
   git checkout -b fix/typo-in-chapter-29
   ```
4. **Make your changes**. Follow the [style guidelines](#style-guidelines).
5. **Verify** that your changes compile:
   ```bash
   cd 29_traits && rustc --edition 2024 main.rs -o /tmp/out && /tmp/out
   ```
6. **Commit** with a clear message:
   ```bash
   git commit -m "Fix typo in chapter 29 traits notes"
   ```
7. **Push** to your fork:
   ```bash
   git push origin fix/typo-in-chapter-29
   ```
8. **Open a Pull Request** on GitHub. Reference any related issues.

### Commit Message Conventions

Use clear, descriptive commit messages:

- `Fix typo in chapter 29 traits notes`
- `Add example demonstrating iterator flatten in chapter 35`
- `Clarify ownership explanation in chapter 19`
- `Translate chapter 01 notes to Chinese`

---

## Style Guidelines

### For `notes.md`

- Use **Markdown** — GitHub-flavored.
- Follow the **beginner-friendly style**: short paragraphs, lots of small code
  blocks, comparison tables, and "Try it Yourself" prompts.
- Use `##` and `###` for sections.
- Code blocks should have language hints: ` ```rust `.
- Tables for comparisons (e.g. "When to use X vs Y").
- End each chapter with `**Next:** [link to next chapter](../XX_name)`.

### For `main.rs`

- **Must compile** with `rustc --edition 2024 main.rs` — no external crates
  for the core chapters. (Advanced chapters may include std-only demos
  with notes on the crate-based production setup.)
- **Must run** without panicking (except in cases where a panic is the
  point — clearly commented).
- Start with a banner comment:
  ```rust
  // ============================================================================
  //  CHAPTER NN — CHAPTER TITLE
  //  One-sentence description.
  // ============================================================================
  ```
- **Rich inline comments** explaining every concept. Use `// ----` dividers
  to break the file into sections.
- Prefer **clear, runnable examples** over abstract ones.
- Test that output is correct: `rustc main.rs -o ./out && ./out`.

### For `README.md`

- Keep the table of contents up to date when adding chapters.
- Use badges sparingly — they should add information, not decoration.

---

## Practice with Real Open-Source Projects

The best way to solidify your Rust skills is to **contribute to real
projects**. The Rust ecosystem is famously welcoming to newcomers —
many crates label their starter-friendly issues with `good-first-issue`
or `help wanted`.

### How to Find Good First Issues

1. **Browse GitHub directly**:
   - [github.com/issues?q=is:open+label:"good+first-issue"+language:rust](https://github.com/issues?q=is%3Aopen+label%3A%22good+first+issue%22+language%3Arust)
   - Filter by Rust language and the `good-first-issue` label.

2. **Use the `good-first-issue` CLI tool**:
   ```bash
   cargo install good-first-issue
   good-first-issue rust
   ```

3. **Check specific beginner-friendly projects** (see list below).

4. **Visit [hello-rust.show](https://hello-rust.show/contributing/)** — a
   curated list of Rust projects welcoming contributors.

### Beginner-Friendly Rust Projects

These projects are known for being welcoming to first-time contributors:

| Project | What it does | Issue tracker |
|---------|--------------|---------------|
| [ripgrep](https://github.com/BurntSushi/ripgrep) | Fast search tool (like grep) | [good-first-issues](https://github.com/BurntSushi/ripgrep/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [rustup](https://github.com/rust-lang/rustup) | The Rust installer | [E-mentor issues](https://github.com/rust-lang/rustup/issues?q=is%3Aopen+label%3AE-mentor) |
| [cargo](https://github.com/rust-lang/cargo) | The build tool | [S-accepted issues](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+is%3Aissue+label%3AS-accepted) |
| [rustfmt](https://github.com/rust-lang/rustfmt) | Code formatter | [good-first-issues](https://github.com/rust-lang/rustfmt/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [clippy](https://github.com/rust-lang/rust-clippy) | The linter | [good-first-issues](https://github.com/rust-lang/rust-clippy/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [rust-analyzer](https://github.com/rust-lang/rust-analyzer) | IDE support | [good-first-issues](https://github.com/rust-lang/rust-analyzer/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [tokio](https://github.com/tokio-rs/tokio) | Async runtime | [good-first-issues](https://github.com/tokio-rs/tokio/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [serde](https://github.com/serde-rs/serde) | Serialization framework | [issues](https://github.com/serde-rs/serde/issues) |
| [axum](https://github.com/tokio-rs/axum) | Web framework | [good-first-issues](https://github.com/tokio-rs/axum/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [Bevy](https://github.com/bevyengine/bevy) | Game engine | [good-first-issues](https://github.com/bevyengine/bevy/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [Servo](https://github.com/servo/servo) | Web browser engine | [good-first-issues](https://github.com/servo/servo/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [TiKV](https://github.com/tikv/tikv) | Distributed KV database | [good-first-issues](https://github.com/tikv/tikv/issues?q=is%3Aopen+is%3Aissue+label%3A%22difficulty%3Aeasy%22) |
| [Nushell](https://github.com/nushell/nushell) | Modern shell | [good-first-issues](https://github.com/nushell/nushell/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [Zola](https://github.com/getzola/zola) | Static site generator | [good-first-issues](https://github.com/getzola/zola/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |
| [uv](https://github.com/astral-sh/uv) | Fast Python package installer | [good-first-issues](https://github.com/astral-sh/uv/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) |

### Tips for Your First Contribution

1. **Read the project's `CONTRIBUTING.md`** before anything else.
2. **Comment on the issue** you want to work on — say "I'd like to take
   this" so maintainers can assign it to you.
3. **Set up the project locally** following its build instructions.
4. **Run the existing tests** to make sure your environment works.
5. **Start small** — a one-line fix or a doc improvement is a great
   first PR.
6. **Write tests** for your change if applicable.
7. **Run `cargo fmt` and `cargo clippy`** before pushing.
8. **Write a clear PR description** explaining what changed and why.
9. **Be patient** — maintainers are often volunteers. A polite follow-up
   after a week is fine.
10. **Be kind** — even if your PR is rejected, thank the maintainer for
    their time. Feedback is a gift.

### What to Do If You Get Stuck

1. Search the project's issues and discussions.
2. Ask in the project's chat (Discord / Zulip / Matrix — most Rust projects
   are on the [Rust Discord](https://discord.gg/rust-lang)).
3. Ask on [users.rust-lang.org](https://users.rust-lang.org).
4. Open a draft PR explaining where you're stuck — maintainers often help.

---

## Using This Tutorial to Teach

You're welcome to use this material in classrooms, workshops, meetups,
and bootcamps under the MIT license. Some suggestions:

- **Print or share `notes.md` files** as handouts.
- **Project `main.rs` on a screen** and walk through the inline comments.
- **Have students modify the examples** — break things, fix them.
- **Use the practice problems** (chapter 28) as homework.
- **Pair students up** — one reads `notes.md`, one runs `main.rs`, they
  teach each other.

If you use this material in a course, we'd love to hear about it! Open an
issue with the `teaching` label and let us know.

---

## Questions?

- **Issues**: [open an issue](../../issues) for bugs, suggestions, or questions
- **Discussions**: [start a discussion](../../discussions) for general questions
- **Rust community**: [users.rust-lang.org](https://users.rust-lang.org) or the [Rust Discord](https://discord.gg/rust-lang)

---

<p align="center">
  <sub>Thank you for helping make Rust more accessible to everyone. 🦀</sub>
</p>
