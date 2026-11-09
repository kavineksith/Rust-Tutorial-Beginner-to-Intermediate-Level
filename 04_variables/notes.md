# Rust Variables

Variables are containers for storing data values.

## Creating a Variable: `let`

```rust
let name = "John";
let age = 30;
println!("{} is {} years old.", name, age);
```

## Variables Are Immutable by Default

This is one of Rust's most important — and surprising — features. By default,
once a variable is bound to a value, it cannot change:

```rust
let x = 5;
// x = 10; // ERROR: cannot assign twice to immutable variable
```

## Making a Variable Mutable: `mut`

Use the `mut` keyword to allow changes:

```rust
let mut x = 5;
println!("Before: {}", x);
x = 10;
println!("After: {}", x);
```

## Type Annotations

Rust usually infers types. You can also be explicit:

```rust
let pi: f64 = 3.14159;
let count: u32 = 1_000_000; // underscores allowed for readability
```

## Shadowing

You can declare a new variable with the **same name** as a previous one.
The new one **shadows** the old:

```rust
let y = 5;
let y = y + 1;
let y = y * 2;
println!("y is {}", y); // 12
```

### Shadowing Can Change the Type

```rust
let spaces = "   ";          // &str
let spaces = spaces.len();   // usize  — same name, different type
```

This is something `mut` cannot do:

```rust
let mut s = "   ";
// s = s.len();  // ERROR: expected &str, found usize
```

## `mut` vs Shadowing

| | `mut` | Shadowing |
|---|-------|-----------|
| Changes value in place | yes | no, creates new binding |
| Can change type | no | yes |
| Reuses the same memory | yes | may reallocate |
| Syntax | `let mut x = ...;` | `let x = ...; let x = ...;` |

## Constants (preview)

```rust
const MAX_USERS: u32 = 100;
```

See chapter 08 for full details.

## Unused Variables

Rust warns about variables you never use. Prefix with `_` to silence the
warning without actually using the value:

```rust
let _unused = 42;   // no warning
// let unused = 42; // warning: unused variable
```

**Next:** [Data Types](../05_data_types) — every value has a type.
