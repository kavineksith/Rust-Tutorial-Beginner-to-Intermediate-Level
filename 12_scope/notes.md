# Rust Scope

A variable's **scope** is the part of the program where it is valid (alive).

## Block Scope

A block `{ ... }` creates a new scope. Variables declared inside it are
dropped when the block ends:

```rust
{
    let x = 5;
    println!("x = {}", x);   // OK
}
// println!("{}", x);        // ERROR: x is not in scope
```

## Why Scope Matters

When a variable goes out of scope, Rust **drops** it — its memory is freed
automatically. This is the heart of the **ownership** system:

* No garbage collector needed.
* No manual `free()` calls.
* No use-after-free bugs.

```rust
{
    let owned = String::from("on the heap");
    println!("{}", owned);
}                            // <- owned is dropped here, memory freed
```

## Shadowing

A new variable with the same name **shadows** the old one:

```rust
let name = "outer";
{
    let name = "inner";            // shadows outer
    println!("{}", name);          // inner
}
println!("{}", name);              // outer (inner is gone)
```

## Same Scope Shadowing

```rust
let count = 1;
let count = count + 1;
let count = count * 10;
println!("{}", count);   // 20
```

Each `let count` creates a new binding. The old one is dropped.

## Loop Body Scope

Variables declared inside a loop body only live for **one iteration**:

```rust
for i in 0..3 {
    let temp = i * i;
    println!("{}", temp);
}                            // temp is dropped at the end of each iteration
// println!("{}", temp);     // ERROR: temp not in scope
```

## Function Scope

Parameters are scoped to the function body:

```rust
fn square(n: i32) -> i32 {
    let result = n * n;
    result
}
```

`n` and `result` are dropped when the function returns.

## `'static` Lifetime

String literals live for the entire program — they have the `'static`
lifetime:

```rust
let s: &'static str = "I live forever.";
```

They are baked into the compiled binary, so they are always valid.

## Moving Out of a Scope

You can move a value OUT of a block by **returning** it. Ownership transfers
to the caller, so the value is NOT dropped:

```rust
fn make_string() -> String {
    let s = String::from("hi");
    s                         // ownership moves to the caller
}
```

## Scope and Ownership — The Rule

> A value is dropped as soon as its **owner** goes out of scope.

This single rule gives Rust both memory safety and zero-cost abstraction.

**Next:** [Strings](../18_strings) — `String` vs `&str`.
