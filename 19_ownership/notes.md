# Rust Ownership

Ownership is Rust's most unique feature. It lets Rust guarantee **memory
safety without a garbage collector** — and at **zero runtime cost**.

## The Three Rules

1. Each value has **one owner** at a time.
2. When the owner goes out of scope, the value is **dropped** (memory freed).
3. You can have multiple **borrows** (references) but only one owner.

## Move Semantics

When you assign a heap-owning value to another variable, **ownership moves**:

```rust
let a = String::from("Hello");
let b = a;                  // ownership moves from a to b
// println!("{}", a);       // ERROR: a no longer owns the value
println!("{}", b);          // OK
```

The original `a` is no longer valid. This prevents double-free bugs.

## Copy Semantics

Simple types (integers, floats, `bool`, `char`, and tuples/arrays of `Copy`
types) are **copied** instead of moved:

```rust
let x = 5;
let y = x;                  // x is COPIED to y
println!("{} {}", x, y);    // BOTH still usable
```

A type is `Copy` if all its fields are `Copy` and it does not own heap data.

## Clone — Explicit Deep Copy

If you really need a deep copy of a heap type, use `.clone()`:

```rust
let a = String::from("Hello");
let b = a.clone();          // explicit deep copy
println!("{} {}", a, b);    // both usable
```

`.clone()` is explicit so you notice when an expensive copy happens.

## Ownership and Functions

Passing a value to a function **moves** it (for non-Copy types):

```rust
fn take_ownership(s: String) {
    println!("I own: {}", s);
}                           // s is dropped here

let text = String::from("hi");
take_ownership(text);
// println!("{}", text);    // ERROR: text was moved
```

For `Copy` types, a copy is passed:

```rust
fn make_copy(n: i32) {
    println!("copy: {}", n);
}

let number = 42;
make_copy(number);
println!("{}", number);     // STILL usable — i32 is Copy
```

## Returning Values Transfers Ownership

```rust
fn give_ownership() -> String {
    String::from("from inside")
}

let s = give_ownership();   // ownership moves to caller
```

## Take and Give Back

```rust
fn take_and_give_back(s: String) -> String {
    s   // ownership moves in, then back out
}
```

This pattern is verbose. The cleaner solution is **borrowing** (next chapter):

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let greeting = String::from("Hello, world!");
let len = calculate_length(&greeting);  // borrow, no move
println!("'{}' has length {}", greeting, len);   // still usable
```

## Why Ownership?

* No garbage collector → predictable performance.
* No `free()` calls → no leaks or double-frees.
* No use-after-free → no security holes.
* No data races → safe concurrency.
* All checks happen at **compile time** → zero runtime overhead.

## Mental Model

Think of ownership like a baton in a relay race:

* Only one runner holds the baton at a time.
* You can pass the baton (move) — then only the new runner holds it.
* Or you can let someone **borrow** it briefly — but they must give it back.

| Operation | Effect on owner |
|-----------|-----------------|
| `let b = a;` (move type) | `a` becomes invalid |
| `let b = a;` (Copy type) | both valid |
| `let b = a.clone();` | both valid (deep copy) |
| `f(a)` (move type) | `a` becomes invalid |
| `f(&a)` | `a` still valid (borrowed) |

**Next:** [Borrowing](../20_borrowing) — sharing without taking ownership.
