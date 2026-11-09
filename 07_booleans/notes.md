# Rust Booleans

The `bool` type has exactly two values: `true` and `false`.

## Declaring

```rust
let t: bool = true;
let f = false;            // type inferred
```

## From Comparisons

```rust
let age = 18;
let is_adult = age >= 18;       // true
let can_drink = age >= 21;      // false
```

All comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`) return `bool`.

## Logical Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `&&` | AND (short-circuit) | `true && false` → `false` |
| `\|\|` | OR (short-circuit) | `true \|\| false` → `true` |
| `!` | NOT | `!true` → `false` |

### Short-circuit Evaluation

`&&` only evaluates its right side if the left side is `true`.
`||` only evaluates its right side if the left side is `false`.

This is essential for safe conditional access:

```rust
if option.is_some() && option.unwrap() > 5 {
    // unwrap only runs if is_some() returned true
}
```

## In `if` Conditions

Rust is strict: `if` requires a `bool`. There is **no implicit conversion**
from numbers or other types.

```rust
let age = 18;
if age >= 18 {
    println!("adult");
}

// THIS WOULD NOT COMPILE in Rust (unlike C or JavaScript):
// let n = 5;
// if n { ... }   // ERROR: expected bool, found integer
```

You must be explicit:

```rust
if n != 0 { ... }
```

This strictness catches a whole class of bugs (e.g. `if (x = 5)` typos).

## Booleans as Values

`if` is an expression, so you can use it to produce a bool:

```rust
let status = if age >= 18 { "adult" } else { "minor" };
```

## Converting `bool` to Integer

```rust
let t = true as i32;   // 1
let f = false as i32;  // 0
```

## Size in Memory

`bool` is **1 byte** (not 1 bit) — addresses must be byte-aligned. For
compact bit storage, use a `Vec<bool>` alternative or bitflags crate.

## Common Methods

```rust
let b = true;
b.then(|| "yes");           // Some("yes") — turns bool into Option
b.then_some("yes");         // Some("yes") — same, lazy-free
// false.then_some("yes") returns None
```

Useful for conditionally producing an `Option<T>` without an `if` block.

**Next:** [If..Else](../13_if_else) — branching your code.
