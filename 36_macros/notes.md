# Rust Macros

**Macros** generate code at **compile time**. They are Rust's
metaprogramming tool — used for things functions cannot do:

* Variadic arguments (`println!`, `vec!`, `format!`)
* Compile-time string formatting / inspection (`file!`, `stringify!`)
* Custom DSLs (`sqlx::query!`, axum routes)
* Derive implementations (`#[derive(Debug)]`)

## Macros vs Functions

| | Function | Macro |
|---|----------|-------|
| Evaluated at | runtime | compile time (expanded) |
| Number of args | fixed | variable |
| Types | strict, fixed | operate on syntax |
| Can be passed around | yes | no |
| Error messages | clear | can be cryptic |
| When to use | normal case | when functions can't |

Rule of thumb: **prefer functions; use macros only when functions genuinely
can't do what you need.**

## Two Kinds of Macros

| Kind | Syntax | Where defined |
|------|--------|---------------|
| **Declarative** | `macro_rules!` | anywhere |
| **Procedural** | `#[proc_macro]`, `#[proc_macro_derive]`, etc. | separate `proc-macro` crate |

# Part A — Declarative Macros (`macro_rules!`)

Pattern-match over tokens. The most common kind — `println!`, `vec!`,
`format!` are all `macro_rules!` macros.

## A Simple Macro

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name)
    };
}

greet!("world");   // expands to: println!("Hello, {}!", "world")
```

* `macro_rules! NAME { ... }` declares a macro.
* Each `($pattern) => { $body }` is an **arm**, like `match`.
* `$name:expr` binds `$name` to any expression.
* Arms are tried top-to-bottom; first match wins.

## Variadic Macros — Repetition

`$( ... ),*` matches zero or more comma-separated repetitions:

```rust
macro_rules! sum {
    ($single:expr) => { $single };
    ($first:expr, $($rest:expr),+) => {
        $first + sum!($($rest),+)
    };
}

sum!(1);              // 1
sum!(1, 2);           // 3
sum!(1, 2, 3, 4, 5);  // 15
```

### Repetition Operators

| Syntax | Matches |
|--------|---------|
| `$(...)*` | zero or more |
| `$(...)+` | one or more |
| `$(...)?` | zero or one (optional) |

### Separator

`$( ... ),*` — the `,` is the separator. Other separators work too:
`$( ... );+` for semicolons, etc.

## The `vec![]` Macro (Simplified)

```rust
macro_rules! my_vec {
    () => { Vec::new() };
    ( $first:expr $(, $rest:expr)* $(,)? ) => {{
        let mut v = Vec::new();
        v.push($first);
        $( v.push($rest); )*
        v
    }};
    ( $val:expr ; $n:expr ) => {{
        let mut v = Vec::with_capacity($n as usize);
        for _ in 0..$n { v.push($val); }
        v
    }};
}

my_vec![]           // empty Vec
my_vec![1, 2, 3]    // [1, 2, 3]
my_vec![0; 5]       // [0, 0, 0, 0, 0]
```

### Optional Trailing Comma

`$(,)?` allows an optional trailing comma so users can write
`vec![1, 2, 3,]` as well as `vec![1, 2, 3]`.

## A HashMap Macro

```rust
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )*
        m
    }};
}

let scores = hashmap! {
    "Alice" => 10,
    "Bob" => 20,
    "Carol" => 30,
};
```

## Fragment Specifiers

Each `$x:TYPE` binds `$x` to a specific kind of syntax:

| Specifier | Matches |
|-----------|---------|
| `:expr` | any expression (`1 + 2`, `foo()`, `vec![]`) |
| `:ident` | an identifier (`foo`, `bar`) |
| `:ty` | a type (`i32`, `Vec<String>`) |
| `:path` | a path (`std::vec::Vec`) |
| `:stmt` | a statement |
| `:pat` | a pattern (`Some(x)`, `(a, b)`) |
| `:literal` | a literal (`42`, `"hi"`) |
| `:meta` | a meta item (inside `#[...]`) |
| `:tt` | a single token tree (most flexible) |
| `:vis` | visibility (`pub`, `pub(crate)`, nothing) |
| `:lifetime` | a lifetime (`'a`, `'static`) |
| `:block` | a block `{ ... }` |

### `:expr` vs `:tt`

`:expr` is convenient but greedy — once it matches an expression, you can't
peek inside. `:tt` matches a single token tree, allowing more flexible
parsing (used in complex macros like `vec![]`).

## A Declare Macro

```rust
macro_rules! declare_static {
    ($name:ident, $ty:ty, $val:expr) => {
        static $name: $ty = $val;
    };
}

declare_static!(APP_VERSION, &str, "1.0.0");
```

A function cannot do this — it cannot create new identifiers or statics.
This is where macros shine.

## Debug Macro Example

```rust
macro_rules! dbg_var {
    ($var:ident) => {
        println!(concat!(stringify!($var), " = {:?}"), $var)
    };
}

let x = 42;
dbg_var!(x);   // prints "x = 42"
```

`stringify!` turns the identifier into a string literal at compile time —
a function cannot do this.

# Part B — Procedural Macros

Procedural macros are Rust **functions** that take a `TokenStream` as
input and return a `TokenStream` as output. They live in a separate crate
of type `proc-macro`.

## Four Kinds

| Kind | Attribute | Use |
|------|-----------|-----|
| Function-like | `#[proc_macro]` | `sqlx::query!(...)` |
| Derive | `#[proc_macro_derive(Trait)]` | `#[derive(Serialize)]` |
| Attribute | `#[proc_macro_attribute]` | `#[tokio::main]` |
| Derive + helper attrs | `#[proc_macro_derive(Trait, attributes(helper))]` | `#[serde(rename = "...")]` |

## Project Layout

```
my_macros/                # the proc-macro crate
├── Cargo.toml           # [lib] proc-macro = true
└── src/
    └── lib.rs

my_app/                   # the crate that uses them
├── Cargo.toml           # my_macros = { path = "../my_macros" }
└── src/
    └── main.rs
```

`my_macros/Cargo.toml`:

```toml
[package]
name = "my_macros"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

## A Derive Macro Example

```rust
// in my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    expanded.into()
}
```

Using it in `my_app`:

```rust
use my_macros::HelloMacro;

trait HelloMacro { fn hello_macro(); }

#[derive(HelloMacro)]
struct Pancakes;

Pancakes::hello_macro();   // prints "Hello, Macro! My name is Pancakes"
```

## The Standard Toolkit

| Crate | Purpose |
|-------|---------|
| `syn` | parse `TokenStream` into a syntax tree |
| `quote` | turn Rust code (via `quote! { ... }`) back into `TokenStream` |
| `proc-macro2` | work with tokens outside a proc-macro context (testing) |

# Part C — Built-in Macros You'll Use Every Day

| Macro | Purpose |
|-------|---------|
| `println!` / `print!` | print to stdout |
| `eprintln!` / `eprint!` | print to stderr |
| `format!` | build a `String` |
| `vec!` | build a `Vec` |
| `dbg!` | debug-print an expression (returns the value!) |
| `panic!` | abort the program |
| `assert!` / `assert_eq!` / `assert_ne!` | runtime assertions |
| `debug_assert!` / `debug_assert_eq!` | debug-only assertions |
| `todo!()` | "I haven't implemented this yet" (panics) |
| `unimplemented!()` | same as `todo!` (older name) |
| `unreachable!()` | mark code that should never run |
| `matches!(val, pat)` | pattern match to `bool` |
| `cfg!(feature = "x")` | compile-time feature check |
| `env!("VAR")` | read an env var at compile time |
| `option_env!("VAR")` | same, returns `Option<&str>` (no error) |
| `file!()` / `line!()` / `column!()` | current source location |
| `module_path!()` | current module path |
| `include_str!("path")` | embed a file as `&'static str` |
| `include_bytes!("path")` | embed a file as `&'static [u8]` |
| `stringify!(ident)` | turn an identifier into a string literal |
| `concat!("a", "b")` | concatenate string literals at compile time |
| `write!(buf, ...)` | write formatted text into a buffer |
| `writeln!(buf, ...)` | write + newline |
| `matches!` | test a pattern against a value |

## `dbg!` — The Debugging Power Tool

```rust
let x = 5;
let y = dbg!(x * 2) + 1;
// prints: [src/main.rs:2] x * 2 = 10
// y = 11
```

`dbg!` prints the file/line, the expression, and the value — then
**returns** the value so you can keep using it. Remove before shipping.

## `matches!` — Quick Pattern Test

```rust
let opt: Option<i32> = Some(5);
if matches!(opt, Some(x) if x > 3) {
    println!("got Some(x > 3)");
}

let dir = Direction::Up;
if matches!(dir, Direction::Up | Direction::Down) {
    println!("vertical");
}
```

Equivalent to a `match` with `true` / `false` arms but much shorter.

## `include_str!` — Embed a File

```rust
const README: &str = include_str!("../README.md");
println!("{}", README);
```

The file's contents become a `&'static str` baked into the binary — no
runtime file I/O.

## `env!` — Compile-Time Env Var

```rust
const VERSION: &str = env!("CARGO_PKG_VERSION");
println!("v{}", VERSION);
```

Set by cargo automatically. Also: `CARGO_PKG_NAME`, `CARGO_PKG_AUTHORS`,
etc.

# Macro Hygiene

Rust macros are **hygienic** — identifiers introduced by a macro cannot
accidentally collide with identifiers in the calling scope:

```rust
macro_rules! using_x {
    () => {
        let x = 42;
        println!("{}", x);
    };
}

fn main() {
    let x = "hello";
    using_x!();        // does NOT shadow the outer x
    println!("{}", x); // still "hello"
}
```

This prevents a whole class of bugs that plague C macros.

## When `macro_rules!` Is Not Enough

Declarative macros can't:

* Parse custom syntax that doesn't fit the `:expr` / `:tt` model.
* Generate impls based on struct fields (use a derive macro).
* Inspect the parsed AST (use `syn`).

For these, write a **procedural macro**.

# Common Pitfalls

## Forgetting the Trailing Semicolon in Arms

```rust
macro_rules! bad {
    ($x:expr) => {
        println!("{}", $x)
    }                      // ERROR: arms must end with ;
}
```

## Greedy `:expr` Matching

```rust
macro_rules! bad {
    ($a:expr, $b:expr) => { ... };
}

bad!(1 + 2, 3);   // $a = "1 + 2", $b = "3" — usually fine
// But:
bad!(1, 2, 3);    // ERROR: no matching arm
```

For multi-arg parsing with optional trailing commas, use `$(,)?` and
repetition.

## Macro Expansion Order

Macros expand **outside-in**. This means a macro cannot see what's inside
another macro call — it sees tokens. Design macros accordingly.

# Summary

| Want | Use |
|------|-----|
| Variadic args | `macro_rules!` with `$( ... )*` |
| Compile-time string ops | `stringify!`, `concat!`, `env!` |
| Embed files | `include_str!`, `include_bytes!` |
| Custom DSL | `macro_rules!` or proc-macro |
| Derive new trait impl | `#[proc_macro_derive]` |
| Attribute macros | `#[proc_macro_attribute]` |
| Function-like macros | `#[proc_macro]` |
| Debug a value | `dbg!` |
| Mark unimplemented code | `todo!` / `unimplemented!` |
| Mark impossible code | `unreachable!` |

## When to Reach for Macros

* You need a `println!`-style variadic API.
* You need to inspect syntax (`stringify!`, `file!`).
* You want a custom DSL (e.g. `#[derive(Serialize)]`).
* You need to generate new identifiers / types / impls.

Otherwise — **write a function**.

---

🎉 **You have finished the entire Rust tutorial!** Chapters 1–28 covered
the beginner curriculum; chapters 29–39 added the advanced topics you
need to write real Rust applications. Go forth and build! 🦀
