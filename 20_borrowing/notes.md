# Rust Borrowing

**Borrowing** means using a value through a **reference** without taking
ownership. It is the everyday alternative to moving values around.

## The Two Kinds of References

| Syntax | Name | Can modify? |
|--------|------|-------------|
| `&T` | immutable reference | no |
| `&mut T` | mutable reference | yes |

## Immutable Borrow: `&T`

You can have **as many immutable borrows as you want** at the same time:

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} {} {}", s, r1, r2);   // all valid together
```

## Mutable Borrow: `&mut T`

You can have **only ONE mutable borrow** at a time:

```rust
let mut s = String::from("hello");
let r = &mut s;
r.push_str(" world");
println!("{}", r);
// let r2 = &mut s;   // ERROR: cannot borrow mutably twice
```

The variable you borrow must itself be `mut`.

## The Borrow Checker's Rule

At any point, a value can be borrowed EITHER:

* by **any number of immutable references**, OR
* by **exactly one mutable reference**.

**Never both at the same time.** This rule prevents data races at compile
time — one of Rust's signature features.

## Borrowing in Functions

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let greeting = String::from("Hello, world!");
let length = calculate_length(&greeting);   // borrow, don't move
println!("'{}' has length {}", greeting, length);   // still usable
```

The function borrows the value — the caller keeps ownership.

## Mutable Borrowing in Functions

```rust
fn append_exclamation(s: &mut String) {
    s.push('!');
}

let mut s = String::from("hi");
append_exclamation(&mut s);
println!("{}", s);   // "hi!"
```

## Slices — A Special Borrow

A **slice** is a borrowed view into a contiguous sequence. The most common
slice is `&str` (a slice of a string):

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];    // "hello"
let world: &str = &s[6..11];   // "world"
```

A classic function that returns a slice:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

The returned slice is **tied to the input's lifetime** — Rust makes sure
you cannot use the slice after the original string is gone.

## Lifetimes (Briefly)

Every reference has a **lifetime** — the region of code where it is valid.
The compiler usually infers it. When it cannot, you annotate explicitly:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` says: the returned reference lives at least as long as the **shorter**
of the two inputs.

## Common Borrow Checker Errors

### Dangling Reference

```rust
let r;
{
    let x = 5;
    r = &x;          // ERROR: x does not live long enough
}
println!("{}", r);
```

`x` is dropped at the end of the inner block; `r` would point to nothing.

### Mutable + Immutable in the Same Scope

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);            // ERROR: mutable borrow while immutable borrow is alive
println!("{}", first);
```

`push` may reallocate, invalidating `first`. The borrow checker catches this
at compile time.

## Why This Matters

* **No data races** — the compiler proves your code is race-free.
* **No dangling pointers** — the compiler proves every reference is valid.
* **No iterator invalidation** — the classic C++ bug is impossible in Rust.
* All checks happen at **compile time** — zero runtime cost.

## Mental Model

Think of references like library books:

* Multiple people can read the same book at the same time (immutable borrows).
* Only one person can write in the book at a time (mutable borrow).
* You can't write in the book while someone else is reading it.

**Next:** [Data Structures](../22_data_structures) — overview of Rust's
collection types.
