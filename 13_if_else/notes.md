# Rust If..Else

`if` runs code only when a condition is `true`.

## Basic `if`

```rust
let number = 6;
if number > 5 {
    println!("greater than 5");
}
```

The condition **must be a `bool`**. Rust does not implicitly convert other
types (unlike C or JavaScript).

## `if` / `else`

```rust
let age = 17;
if age >= 18 {
    println!("adult");
} else {
    println!("minor");
}
```

## `if` / `else if` / `else`

```rust
let score = 78;
if score >= 90 {
    println!("A");
} else if score >= 80 {
    println!("B");
} else if score >= 70 {
    println!("C");
} else {
    println!("F");
}
```

## `if` Is an Expression

Rust's `if` returns a value. The last expression in each arm (no semicolon)
is the result:

```rust
let n = 7;
let parity = if n % 2 == 0 { "even" } else { "odd" };
```

**Both arms must have the same type** — this is a compile-time check:

```rust
// ERROR: mismatched types
let bad = if true { 1 } else { "two" };
```

## Condition Must Be `bool`

```rust
let count = 0;
// if count { ... }   // ERROR: expected bool, found integer
if count != 0 { ... } // OK
```

This strictness prevents bugs like `if (x = 5)` typos that plague C.

## Multi-line Arms

```rust
let advice = if temp > 30 {
    let diff = temp - 30;
    format!("{} degrees above 30", diff)
} else {
    String::from("comfortable")
};
```

## `if let` (Lightweight Pattern Match)

For the common case of "do something if a pattern matches, otherwise fall
through":

```rust
let maybe_name: Option<&str> = Some("Ada");
if let Some(name) = maybe_name {
    println!("Hello, {}!", name);
} else {
    println!("No name.");
}
```

This is equivalent to (but shorter than):

```rust
match maybe_name {
    Some(name) => println!("Hello, {}!", name),
    None       => println!("No name."),
}
```

## Nesting vs Early Returns

Deeply nested `if`s are hard to read. Prefer early returns inside functions:

```rust
// Hard to read:
fn check(x: i32) {
    if x > 0 {
        if x < 100 {
            println!("valid");
        }
    }
}

// Better — early return (guard clause):
fn check2(x: i32) {
    if x <= 0   { return; }
    if x >= 100 { return; }
    println!("valid");
}
```

**Next:** [Match](../17_match) — Rust's powerful pattern matching.
