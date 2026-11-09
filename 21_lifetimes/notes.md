# Rust Lifetimes (in depth)

Lifetimes are the borrow checker's mechanism for tracking **how long
references are valid**. Their only job is to prove that **no reference
outlives the data it points to** — preventing dangling pointers, all at
compile time with zero runtime cost.

## What Is a Lifetime?

A lifetime is a **compile-time label** on a reference, naming the scope
where the reference is valid. Lifetimes do not exist at runtime.

```rust
{
    let r;                          // ----------+- 'a
    {                               //           |
        let x = 5;                  // --+- 'b   |
        r = &x;                     //   |       |
    }                               // --+       |  x is dropped here
    // println!("{}", r);           //           |  ERROR: x does not live long enough
}                                   // ----------+
```

The compiler refuses this because `r` (lifetime `'a`) tries to outlive `x`
(lifetime `'b`).

## Why You Sometimes Need to Annotate

When a function returns a reference, the compiler needs to know which input
the output is tied to. If it can't tell, you must annotate:

```rust
// Without annotations — ERROR:
// fn longest(x: &str, y: &str) -> &str { ... }

// With lifetime annotations — OK:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Anatomy of a Lifetime Annotation

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

* `<'a>` declares a **lifetime parameter** named `'a` (single quote + name).
* `&'a str` is "a `&str` that lives at least as long as `'a`".
* Both inputs share `'a` — so the output lives **as long as the shorter input**.

This is a **contract**, not a runtime value. The compiler checks every call
site to make sure the contract holds.

## Lifetime Elision Rules

The compiler applies three rules to infer lifetimes automatically. If they
fully determine every output lifetime, you can omit annotations.

### Rule 1: Each Input Reference Gets Its Own Lifetime

```rust
fn foo(x: &str)              // becomes
fn foo<'a>(x: &'a str)
```

### Rule 2: Single Input → All Outputs Share That Lifetime

```rust
fn foo(x: &str) -> &str              // becomes
fn foo<'a>(x: &'a str) -> &'a str
```

This is why `fn first_word(s: &str) -> &str { ... }` compiles without
explicit lifetimes.

### Rule 3: Methods — Outputs Tied to `&self`

```rust
fn foo(&self, x: &str) -> &str       // becomes
fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str
```

If there are multiple inputs AND one is `&self`/`&mut self`, all output
references get `self`'s lifetime.

### When Rules Don't Suffice

If the rules can't fully determine every output lifetime, you must
annotate explicitly. Common case:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

Two inputs, one output — rules can't decide which input to tie the output
to, so you must annotate.

## Structs Holding References

A struct that holds a reference **must** declare a lifetime parameter, and
the referenced data must outlive the struct:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn part(&self) -> &'a str { self.part }
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first = Excerpt { part: &novel[..16] };
println!("{}", first.part());
```

`'a` says: the `Excerpt` cannot outlive the string it borrows from.

## The `'static` Lifetime

`'static` means "lives for the entire program". All string literals have
it — they're baked into the binary:

```rust
let s: &'static str = "I live forever";
```

Functions can require `'static`:

```rust
fn takes_static(s: &'static str) { /* ... */ }

takes_static("literal");          // OK
// let owned = String::from("hi");
// takes_static(&owned);          // ERROR: not 'static
```

`'static` is the **default** for statics and constants. Don't sprinkle it
around casually — it can hide borrow checker issues.

## Multiple Lifetimes

When inputs have different relationships to the output, use multiple
lifetimes:

```rust
fn split_at<'a, 'b>(text: &'a str, _marker: &'b str) -> &'a str {
    &text[..text.len() / 2]
}
```

The output is tied to `text` (lifetime `'a`), **not** to `_marker` (`'b`).
The caller knows that the result lives as long as `text`, but the
`_marker` argument can be shorter-lived.

### Lifetime Subtyping

If `'a: 'b` ("`'a` outlives `'b`"), then `&'a T` can be used where `&'b T`
is expected:

```rust
fn longest_with_announcement<'a, 'b>(x: &'a str, y: &'a str, ann: &'b str) -> &'a str
where
    'a: 'b,   // 'a outlives 'b (often elided when obviously true)
{
    println!("{}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## Lifetimes on Generic Types

You can combine `T` with a lifetime bound:

```rust
fn use_ref<'a, T: 'a>(r: &'a T) -> &'a T { r }
```

`T: 'a` means "T itself doesn't contain any references shorter than 'a".
This is required when storing `T` behind a reference.

## Higher-Ranked Trait Bounds (HRTB) — `for<'a>`

`for<'a>` means "for any lifetime `'a`". Useful for closures that accept
references of arbitrary lifetimes:

```rust
fn apply_to_str<F>(s: &str, f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    f(s)
}
```

`for<'a> Fn(&'a str) -> usize` says "F can be called with a `&str` of
**any** lifetime". This is what makes `Fn(&str) -> usize` closures work
regardless of the caller's lifetime.

## Common Patterns

### Returning a Slice

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    &s[..]
}
```

Elided form — Rule 2 ties the output to `s`.

### Struct with Multiple Lifetimes

```rust
struct Parser<'src, 'err> {
    source: &'src str,
    errors: &'err mut Vec<String>,
}
```

### Iterator Adapters

Iterator combinators are full of lifetimes, but you almost never need to
write them — elision handles it:

```rust
fn words(s: &str) -> impl Iterator<Item = &str> {
    s.split_whitespace()
}
```

## Common Mistakes

### Forgetting the Lifetime on a Struct

```rust
// struct Excerpt { part: &str }    // ERROR: missing lifetime
struct Excerpt<'a> { part: &'a str }   // OK
```

### Trying to Return a Reference to a Local

```rust
fn dangle() -> &str {           // ERROR: missing lifetime
    let s = String::from("hi");
    &s                          // s is dropped at end of fn — would dangle
}
```

Fix: return an owned `String` instead.

### Confusing `'static` with "any lifetime"

```rust
fn make_static(s: &str) -> &'static str {
    s       // ERROR: cannot return non-'static as 'static
}
```

You cannot "upgrade" a shorter lifetime to `'static`. To return `'static`,
you need either a literal or a leaked `Box::leak`.

## Mental Model

* Lifetimes are **compile-time tags**, not runtime values.
* `'a` is a name for "the scope where this reference is valid".
* The compiler uses lifetimes to **prove** no dangling references exist.
* You annotate only when the compiler can't infer them.
* Once compiled, lifetimes **disappear** — zero runtime cost.

## When You Need to Write Lifetimes

| Situation | Annotate? |
|-----------|-----------|
| Function with one `&` input, returns `&` | No (rule 2) |
| Method returning `&` tied to `&self` | No (rule 3) |
| Function with multiple `&` inputs, returns `&` | **Yes** |
| Struct holding a `&` field | **Yes** |
| Function returning `impl Trait` capturing inputs | Sometimes |
| Storing `T` behind a reference | **Yes** (`T: 'a`) |

## Cheat Sheet

```rust
// Single input, single output — elided:
fn first_word(s: &str) -> &str

// Multiple inputs — must annotate:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

// Struct holding references — must annotate:
struct Excerpt<'a> { part: &'a str }

// 'static — lives for the entire program:
let s: &'static str = "literal";

// HRTB — closure works with any lifetime:
F: for<'a> Fn(&'a str) -> usize
```

**Next:** [Async/Await](../40_async) — concurrent programming in Rust.
