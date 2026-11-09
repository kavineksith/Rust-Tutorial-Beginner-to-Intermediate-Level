// ============================================================================
//  CHAPTER 40 — CARGO & BUILD SYSTEM (in depth)
//  Cargo is Rust's official build system and package manager. This file is
//  a SELF-CONTAINED demo (no external crates) plus extensive comments
//  showing the idiomatic cargo layout for real projects.
// ============================================================================

// ---- Conditional compilation with cfg! and #[cfg] ----------------------
// cfg!() is a macro that returns a bool at compile time:
fn _build_info() {
    if cfg!(target_os = "linux") {
        println!("Built on Linux");
    } else if cfg!(target_os = "windows") {
        println!("Built on Windows");
    } else if cfg!(target_os = "macos") {
        println!("Built on macOS");
    }

    // Common cfg keys:
    //   target_os       = "linux" | "windows" | "macos" | "android" | ...
    //   target_arch     = "x86_64" | "aarch64" | "wasm32" | ...
    //   target_family   = "unix" | "windows"
    //   feature         = "your_feature_name"
    //   debug_assertions (true in debug builds)
    //   test            (true when running cargo test)
}

// ---- #[cfg] attribute: include code only on certain conditions --------
#[cfg(target_os = "linux")]
fn platform_specific() -> &'static str { "Linux-specific code" }

#[cfg(not(target_os = "linux"))]
fn platform_specific() -> &'static str { "Non-Linux fallback" }

// ---- Feature flags -----------------------------------------------------
// Define features in Cargo.toml: [features] my_feature = []
// Then gate code with:
//
//   #[cfg(feature = "my_feature")]
//   fn only_with_feature() { /* ... */ }
//
// Features let users of your crate opt into optional behavior.
#[cfg(feature = "nonexistent_feature")]
fn _only_compiled_when_feature_on() { /* ... */ }

// ---- The `env!` and `option_env!` macros for compile-time env vars -----
// option_env! returns Option<&'static str> at compile time. In a const
// context we can't use .unwrap_or (not yet stable as const fn), so we use
// match instead:
const COMPILE_TIME_MESSAGE: &str = match option_env!("RUST_TUTORIAL_MSG") {
    Some(s) => s,
    None => "(RUST_TUTORIAL_MSG env var not set at compile time)",
};

fn main() {
    println!("=== Cargo & Build System ===\n");

    _build_info();
    println!("platform_specific(): {}", platform_specific());
    println!("\ncompile-time env: {}", COMPILE_TIME_MESSAGE);

    // ========================================================================
    //  THE cargo COMMAND CHEAT SHEET
    // ========================================================================
    //
    //  Project setup:
    //    cargo new my_app                  # new binary project
    //    cargo new my_lib --lib            # new library project
    //    cargo new my_app --vcs git        # also init git
    //
    //  Building & running:
    //    cargo build                       # debug build
    //    cargo build --release             # optimized build
    //    cargo run                         # build + run
    //    cargo run -- arg1 arg2            # pass args to the program
    //    cargo check                       # type-check only (fast)
    //    cargo check --release
    //
    //  Dependencies:
    //    cargo add serde                   # add latest version
    //    cargo add tokio --features full   # with features
    //    cargo add serde --version 1.0
    //    cargo remove serde                # remove a dependency
    //    cargo update                      # update Cargo.lock
    //    cargo update -p serde             # update only serde
    //    cargo tree                        # show dependency tree
    //    cargo tree --duplicates           # show duplicate versions
    //
    //  Testing:
    //    cargo test                        # run all tests
    //    cargo test test_name              # run tests matching name
    //    cargo test -- --nocapture         # show println! output
    //    cargo test --doc                  # only doctests
    //    cargo bench                       # run benchmarks (nightly)
    //
    //  Documentation:
    //    cargo doc                         # build docs for all crates
    //    cargo doc --open                  # build + open in browser
    //    cargo doc --no-deps               # only your crate, not deps
    //
    //  Publishing:
    //    cargo login <token>               # one-time setup
    //    cargo publish --dry-run           # check before publishing
    //    cargo publish                     # upload to crates.io
    //
    //  Formatting & linting:
    //    cargo fmt                         # auto-format
    //    cargo fmt -- --check              # check without writing
    //    cargo clippy                      # run the linter
    //    cargo clippy --fix                # auto-fix issues
    //    cargo fix --allow-dirty           # apply compiler suggestions
    //
    //  Cleanup:
    //    cargo clean                       # remove target/
    //    cargo clean -p serde              # remove only serde's artifacts
    //
    // ========================================================================

    // ========================================================================
    //  THE Cargo.toml MANIFEST
    // ========================================================================
    //
    //  [package]                          # metadata about the package
    //  name = "my_app"
    //  version = "0.1.0"
    //  edition = "2024"                   # 2015 | 2018 | 2021 | 2024
    //  authors = ["Your Name <you@example.com>"]
    //  description = "A short description"
    //  license = "MIT OR Apache-2.0"
    //  repository = "https://github.com/you/my_app"
    //  readme = "README.md"
    //  keywords = ["cli", "web"]
    //  categories = ["command-line-utilities"]
    //
    //  [dependencies]                     # runtime dependencies
    //  serde = { version = "1.0", features = ["derive"] }
    //  tokio = { version = "1", features = ["full"] }
    //  anyhow = "1.0"
    //  my_other_crate = { path = "../my_other_crate" }   # local path
    //  my_git_crate = { git = "https://github.com/foo/bar" }
    //  my_git_crate2 = { git = "...", branch = "dev" }
    //  my_git_crate3 = { git = "...", tag = "v1.0.0" }
    //
    //  [dev-dependencies]                 # only for tests/benchmarks
    //  proptest = "1.0"
    //  pretty_assertions = "1.3"
    //
    //  [build-dependencies]               # for build.rs
    //  cc = "1.0"
    //
    //  [features]                         # optional feature flags
    //  default = ["json"]
    //  json = ["dep:serde_json"]
    //  yaml = ["dep:serde_yaml"]
    //
    //  [lib]                              # library target config
    //  name = "my_app"
    //  path = "src/lib.rs"
    //  crate-type = ["lib", "cdylib", "staticlib"]
    //
    //  [[bin]]                            # binary target config
    //  name = "my_app"
    //  path = "src/main.rs"
    //
    //  [[bench]]                          # benchmark target
    //  name = "my_bench"
    //  harness = false                    # use custom harness (criterion)
    //
    //  [profile.release]                  # release build options
    //  opt-level = 3                      # 0-3, "s" (size), "z" (smaller)
    //  lto = true                         # link-time optimization
    //  codegen-units = 1                  # slower build, better codegen
    //  panic = "abort"                    # smaller binary, no unwinding
    //  strip = true                       # strip debug symbols
    //
    //  [profile.dev]                      # dev build options
    //  opt-level = 0
    //  debug = true
    //
    //  ========================================================================
    //  WORKSPACES — multiple crates sharing one target/ and Cargo.lock
    //  ========================================================================
    //
    //  # Top-level Cargo.toml:
    //  [workspace]
    //  members = ["cli", "lib", "common"]
    //  resolver = "2"
    //
    //  # Run a command for a specific member:
    //  cargo build -p lib
    //  cargo test -p cli
    //
    //  ========================================================================
    //  BUILD SCRIPTS (build.rs) — run before compilation
    //  ========================================================================
    //
    //  // build.rs
    //  fn main() {
    //      // Read environment variables, generate code, compile C code, etc.
    //      println!("cargo:rerun-if-changed=some_file.txt");
    //      println!("cargo:rustc-link-lib=png");
    //  }
    //
    //  ========================================================================
    //  EDITIONS — opt-in to new language features without breaking old code
    //  ========================================================================
    //
    //  2015: the original Rust
    //  2018: module system overhaul, async/await, dyn Trait, etc.
    //  2021: IntoIterator for arrays, disjoint closure captures, etc.
    //  2024: unsafe attributes, new edition adjustments
    //
    //  Set in Cargo.toml: edition = "2024"
    //
    //  ========================================================================
    //  PUBLISHING TO crates.io
    //  ========================================================================
    //
    //  1. cargo login <token>               # one-time setup
    //  2. cargo publish --dry-run           # check for issues
    //  3. cargo publish                     # upload
    //
    //  Required fields for publishing: name, version, description, license.
    //
    //  ========================================================================
    //  VERSIONING
    //  ========================================================================
    //
    //  Cargo follows Semantic Versioning: MAJOR.MINOR.PATCH
    //    * PATCH (0.1.0 -> 0.1.1): bug fixes only
    //    * MINOR (0.1.0 -> 0.2.0): new features, backward compatible
    //    * MAJOR (1.0.0 -> 2.0.0): breaking changes
    //
    //  Dependency version syntax:
    //    "1.0"        = >=1.0.0, <2.0.0    (caret, the default)
    //    "^1.0"       = same as "1.0"
    //    "~1.0"       = >=1.0.0, <1.1.0
    //    "=1.0.0"     = exactly 1.0.0
    //    ">=1.0, <2.0" = range
    //    "*"          = any version (dangerous!)
    //    "1.0.0-beta.1" = pre-release
    // ========================================================================
}
