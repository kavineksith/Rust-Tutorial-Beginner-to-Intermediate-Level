# Rust Comments

Comments are notes in your code that the compiler ignores.

## Single-line Comments — `//`

Anything after `//` to the end of the line is a comment:

```rust
// This is a comment
let x = 5; // This is also a comment
```

## Block Comments — `/* ... */`

Span multiple lines:

```rust
/* This is a
   multi-line comment */
let x = 5;
```

Rust supports **nested** block comments, unlike C:

```rust
/* outer /* inner */ still outer */
```

This makes it safe to comment out a block that already contains comments.

## Documentation Comments

Rust has special comments that become HTML documentation when you run
`cargo doc`.

### Outer doc comments — `///`

Document the **next** item (function, struct, enum, module):

```rust
/// Adds two integers.
///
/// # Examples
///
/// ```
/// let r = add(2, 3);
/// assert_eq!(r, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Inner doc comments — `//!`

Document the **enclosing** item — typically a module or crate:

```rust
//! # My Crate
//!
//! A short description of what the crate does.
```

## Generate Documentation

```bash
cargo doc            # build docs for your crate and dependencies
cargo doc --open     # build and open in the browser
```

## Best Practices

| Do | Don't |
|----|-------|
| Explain *why*, not *what* | `let x = 5; // assign 5 to x` |
| Use good variable names instead of comments | `let n = 5; // number of retries` |
| Keep comments up to date with code | Leave stale comments lying around |
| Use `///` on all public items | Skip documentation on public APIs |

**Next:** [Variables](../04_variables) — storing and naming data.
