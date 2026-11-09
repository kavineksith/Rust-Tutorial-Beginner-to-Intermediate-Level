# Rust For Loops

`for` is the most common loop in Rust. It iterates over anything that
implements the `Iterator` trait (ranges, arrays, vectors, hashmaps, etc.).

## Ranges

```rust
for n in 1..=5 { print!("{} ", n); }   // 1 2 3 4 5 (inclusive)
for n in 1..5  { print!("{} ", n); }   // 1 2 3 4    (exclusive)
```

## Reverse and Step

```rust
for n in (1..=5).rev()      { print!("{} ", n); }  // 5 4 3 2 1
for n in (0..10).step_by(2) { print!("{} ", n); }  // 0 2 4 6 8
```

## Iterating over an Array

```rust
let fruits = ["apple", "banana", "cherry"];
for fruit in fruits {
    println!("I like {}", fruit);
}
```

## With Index: `.iter().enumerate()`

```rust
let fruits = ["apple", "banana", "cherry"];
for (i, fruit) in fruits.iter().enumerate() {
    println!("{}: {}", i, fruit);
}
```

## Iterating over a `Vec`

```rust
let numbers = vec![10, 20, 30];
for n in &numbers {            // borrow — keeps the vec usable
    print!("{} ", n);
}
```

### Three Flavours of Iteration

| Method | Yields | Modifies? | Consumes? |
|--------|--------|-----------|-----------|
| `for x in &v` / `v.iter()` | `&T` | no | no |
| `for x in &mut v` / `v.iter_mut()` | `&mut T` | yes | no |
| `for x in v` / `v.into_iter()` | `T` | — | yes |

```rust
let mut counts = [0; 5];
for c in counts.iter_mut() {
    *c += 1;
}
```

## `break` and `continue`

```rust
for n in 1..=10 {
    if n == 5 { continue; }
    if n == 8 { break; }
    print!("{} ", n);    // 1 2 3 4 6 7
}
```

## Loop Labels with `for`

```rust
'outer: for i in 1..=3 {
    for j in 1..=3 {
        if i * j == 4 { break 'outer; }
        print!("({},{}) ", i, j);
    }
}
```

## Using `_` When You Don't Need the Value

```rust
let mut count = 0;
for _ in 0..10 {
    count += 1;          // we just want to repeat 10 times
}
```

`_` tells the compiler (and reader) "I'm ignoring this value".

## Iterating over a `HashMap`

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob",   20);

for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

## `for` vs `while` vs `loop`

| Construct | Best for |
|-----------|----------|
| `for` | iterating over a collection / range |
| `while` | loop while a condition holds |
| `loop` | infinite loop or when exit happens mid-body |

In Rust, `for` is preferred over `while` for almost all collection iteration —
it is safer (no manual index) and clearer.

**Next:** [Functions](../11_functions) — reusable blocks of code.
