# Rust Syntax

A Rust program is made of **items** (functions, structs, enums, constants,
modules) and **statements / expressions** inside function bodies.

## The Smallest Program

```rust
fn main() {
    println!("Hello, World!");
}
```

* `fn` — keyword to declare a function.
* `main` — the entry point of every executable program.
* `()` — empty parameter list.
* `{ ... }` — the function body.
* `println!(...)` — a macro that prints text. The `!` says "this is a macro".
* `;` — ends a statement.

## Function Definition

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

* Parameters **must** have explicit types.
* The `-> i32` declares the return type.
* The last expression in the body (no semicolon) is the return value.
  No `return` keyword needed (though you can use it for early returns).

## Variables

```rust
let x = 5;          // immutable by default
let mut y = 10;     // `mut` makes it mutable
y = 20;             // OK
// x = 6;           // ERROR: cannot assign twice to immutable variable
```

## Statements vs Expressions

| | Statement | Expression |
|---|-----------|------------|
| Does something | yes | yes |
| Returns a value | no (returns `()`) | yes |
| Ends with `;` | yes | no |

```rust
let x = 5;            // statement
let y = {             // block expression
    let a = 1;
    a + 2             // no semicolon -> the block evaluates to 3
};
```

## `if` Is an Expression

```rust
let n = if true { 1 } else { 0 };
```

Both arms must have the **same type**.

## Macros vs Functions

| | Function | Macro |
|---|----------|-------|
| Call syntax | `foo(x)` | `foo!(x)` |
| Number of args | fixed | variable |
| Checked | at runtime | at compile time (expanded) |
| Examples | `add(1, 2)` | `println!`, `vec!`, `format!`, `panic!` |

## Semicolons and Braces

* Every statement ends with `;`.
* Every block is wrapped in `{ }`.
* An expression with a trailing `;` becomes a statement (returns `()`).

**Next:** [Output](../09_output) — printing text with `println!`.
