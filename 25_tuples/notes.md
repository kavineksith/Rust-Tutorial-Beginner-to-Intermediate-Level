# Rust Tuples

A tuple groups a **fixed number** of values, possibly of **different types**.

## Creating a Tuple

```rust
let pair: (i32, f64) = (1, 2.5);
let person: (&str, u32, f64) = ("Ada", 36, 1.68);
let unit: () = ();   // the empty tuple
```

## Accessing Elements by Index

```rust
let person = ("Ada", 36, 1.68);
println!("{}", person.0);   // Ada
println!("{}", person.1);   // 36
println!("{}", person.2);   // 1.68
```

Indices are `.0`, `.1`, `.2`, ... — they start at zero.

## Destructuring

```rust
let person = ("Ada", 36, 1.68);
let (name, age, height) = person;
println!("{} is {} years old", name, age);
```

Use `_` for values you don't need:

```rust
let (name, _, _) = person;
```

## Returning Multiple Values from a Function

Tuples are the standard way to return multiple values:

```rust
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in numbers {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

let (min, max) = min_max(&[3, 1, 4, 1, 5]);
```

## Nested Tuples

```rust
let nested: ((i32, i32), (i32, i32)) = ((1, 2), (3, 4));
let ((a, b), (c, d)) = nested;
```

## Tuples With Mixed Types

```rust
let record = (String::from("Alice"), vec![90, 85, 88]);
let (student, scores) = record;
```

## The Unit Type `()`

The empty tuple `()` is called **unit**. It represents "no meaningful value":

```rust
fn print_hi() -> () {       // () is the default return type
    println!("hi");
}

let u: () = ();             // the only value of type ()
```

Functions that return `()` are sometimes called "procedures" or "void
functions" in other languages.

## Single-element Tuple (rare)

Needs a trailing comma to distinguish from a parenthesized expression:

```rust
let single: (i32,) = (5,);
```

## Pattern Matching on Tuples

```rust
let point = (3, -4);
match point {
    (0, 0)             => println!("origin"),
    (x, 0)             => println!("on x-axis at {}", x),
    (0, y)             => println!("on y-axis at {}", y),
    (x, y) if x > 0 && y > 0 => println!("quadrant I"),
    (x, y)             => println!("elsewhere ({}, {})", x, y),
}
```

## Swapping Values

```rust
let (a, b) = (1, 2);
let (a, b) = (b, a);   // rebind, swaps
```

Since Rust 1.59, you can also use destructuring assignment to mutable
bindings:

```rust
let (mut a, mut b) = (1, 2);
(a, b) = (b, a);       // swap in place
```

## Tuples vs Structs

| | Tuple | Struct |
|---|-------|--------|
| Field access | by index (`.0`, `.1`) | by name (`.name`) |
| Readability | low (no names) | high |
| Use case | temporary grouping, multiple returns | named records |

When a tuple grows beyond 3-4 elements, consider converting it to a struct
for readability.

## Common Uses

* Returning multiple values from a function.
* Temporarily grouping related values.
* Pattern-matching against fixed shapes (coordinates, key-value pairs).
* `Result<T, E>` and `Option<T>` — internally use enums, not tuples, but
  follow similar ideas.

**Next:** [HashMap](../26_hashmap) — key-value maps.
