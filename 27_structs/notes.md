# Rust Structs

A **struct** is a custom record type that groups named fields. It is the
Rust equivalent of a "class" (without inheritance) or a "struct" in C.

## Three Kinds of Struct

### 1. Named-field struct (most common)

```rust
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

let user = User {
    name: String::from("Ada"),
    age: 36,
    email: String::from("ada@example.com"),
    active: true,
};
```

### 2. Tuple struct (unnamed fields)

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let red = Color(255, 0, 0);
let origin = Point(0, 0, 0);

println!("{}", red.0);   // 255
```

Each tuple struct is its **own type** — even with the same fields. A `Color`
cannot be passed where a `Point` is expected.

### 3. Unit struct (no fields)

```rust
struct AlwaysEnabled;
```

Useful as a marker type or for implementing traits without data.

## Accessing and Mutating Fields

```rust
let mut user = User { /* ... */ };
user.age = 37;          // mutation requires `mut` on the binding
```

> Note: the whole binding must be `mut`. You cannot mark individual fields
> as mutable like in C++.

## Struct Update Syntax

Copy remaining fields from another instance:

```rust
let user2 = User {
    email: String::from("lovelace@example.com"),
    ..user1            // take the rest from user1
};
```

Fields that are not `Copy` (like `String`) are **moved** out of `user1`.
You can still use the `Copy` fields of `user1` afterwards.

## Methods: `impl` Blocks

```rust
struct Rectangle { width: f64, height: f64 }

impl Rectangle {
    // Associated function (no self) — like a static method.
    fn square(size: f64) -> Self {
        Rectangle { width: size, height: size }
    }

    // Method (borrows self immutably).
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method (borrows self mutably).
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method (consumes self).
    fn into_square(self) -> Rectangle {
        let side = (self.width + self.height) / 2.0;
        Rectangle { width: side, height: side }
    }
}
```

### `self` flavours

| Signature | Borrows? | Modifies? | Consumes? |
|-----------|----------|-----------|-----------|
| `&self` | yes, immutably | no | no |
| `&mut self` | yes, mutably | yes | no |
| `self` | no | — | yes |
| no `self` | — (associated fn) | — | — |

Call methods with the dot operator: `r.area()`.
Call associated functions with `::`: `Rectangle::square(5.0)`.

## Derived Traits

Most structs benefit from these derivable traits:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Rectangle { width: f64, height: f64 }
```

| Trait | What it gives you |
|-------|-------------------|
| `Debug` | `println!("{:?}", r)` formatting |
| `Clone` | `r.clone()` deep copy |
| `Copy` | implicit copy on assignment (only for small all-`Copy` structs) |
| `PartialEq` | `r1 == r2` equality |
| `Eq` | full equality (with `PartialEq`) |
| `Hash` | use as a `HashMap` key |
| `Default` | `Rectangle::default()` |

## Printing Structs

```rust
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

let r = Rectangle { width: 1.0, height: 2.0 };
println!("{:?}", r);      // Rectangle { width: 1.0, height: 2.0 }
println!("{:#?}", r);     // pretty-printed, multi-line
```

## Tuple Struct Destructuring

```rust
struct Color(i32, i32, i32);
let red = Color(255, 0, 0);
let Color(r, g, b) = red;
```

## When to Use Each Kind

| Use | When |
|-----|------|
| Named-field struct | you have several distinct fields — best for clarity |
| Tuple struct | 1-3 fields where names would be redundant (e.g. coordinates) |
| Unit struct | you need a type with no data, just to implement traits |

## Structs vs Tuples

| | Struct | Tuple |
|---|--------|-------|
| Fields | named | positional |
| Self-documenting | yes | no |
| Field access | `s.name` | `t.0` |
| Adding a field | no breaking change to callers using `..` | breaking change |

For long-lived data, prefer structs. For quick groupings (multiple return
values), tuples are fine.

**Next:** [Enums](../28_enums) — sum types and Option/Result.
