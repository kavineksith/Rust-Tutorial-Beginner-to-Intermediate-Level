# Rust Enums

An **enum** lets a value be **one of several variants**. Each variant can
optionally carry data. Enums are one of Rust's most powerful features.

## A Simple Enum

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;
```

Variants are accessed with the `::` syntax: `Direction::Up`.

## Pattern Matching

```rust
match dir {
    Direction::Up    => println!("up"),
    Direction::Down  => println!("down"),
    Direction::Left  => println!("left"),
    Direction::Right => println!("right"),
}
```

`match` must be **exhaustive** — every variant must be handled (or use `_`
as a catch-all).

## Variants Carrying Data

This is what makes Rust enums far more powerful than C enums:

```rust
enum Shape {
    Circle(f64),                              // one value
    Square(f64),
    Rectangle(f64, f64),                      // tuple-like
    Triangle { a: f64, b: f64, c: f64 },      // struct-like
}

let s = Shape::Circle(2.0);

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r)         => std::f64::consts::PI * r * r,
        Shape::Square(side)      => side * side,
        Shape::Rectangle(w, h)   => w * h,
        Shape::Triangle { a, b, c } => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}
```

Each variant can have **different types and amounts of data**.

## Real Example: IP Address

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

In C you'd need a struct with a tag + a union; Rust folds both into one
ergonomic construct.

## Methods on Enums

Enums can have `impl` blocks just like structs:

```rust
impl IpAddr {
    fn new_v4(a: u8, b: u8, c: u8, d: u8) -> Self {
        IpAddr::V4(a, b, c, d)
    }
    fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(127, _, _, _) => true,
            IpAddr::V6(s) if s == "::1" => true,
            _ => false,
        }
    }
}
```

## The `Option<T>` Enum

Rust has **no `null`**. Instead, the `Option<T>` enum encodes "maybe a
value":

```rust
enum Option<T> {
    Some(T),
    None,
}

let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

The compiler forces you to handle the `None` case — no NullPointerException
ever.

### Common `Option` Methods

```rust
opt.unwrap();                  // panics if None
opt.unwrap_or(default);        // returns default if None
opt.unwrap_or_else(|| compute_default());
opt.map(|x| x + 1);            // transform Some's value
opt.filter(|x| *x > 10);       // keep Some only if predicate holds
opt.is_some();  opt.is_none();
```

## The `Result<T, E>` Enum

For operations that can **fail**, Rust uses `Result`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

let ok: Result<i32, &str>  = Ok(5);
let err: Result<i32, &str> = Err("oops");
```

### The `?` Operator

Short-circuit return on `Err` — extremely common in functions that can fail:

```rust
fn read_config() -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open("config.txt")?;  // returns Err if open fails
    let mut s = String::new();
    f.read_to_string(&mut s)?;                         // returns Err if read fails
    Ok(s)
}
```

`?` is sugar for:

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
}
```

## `if let` — Lightweight Match

When you only care about one variant:

```rust
if let Some(n) = maybe_number {
    println!("got {}", n);
}
// equivalent to:
match maybe_number {
    Some(n) => println!("got {}", n),
    None => {},
}
```

## Enums as State Machines

```rust
enum OrderState {
    Pending,
    Shipped { tracking: String },
    Delivered { signed_by: String },
    Cancelled(String),
}

match state {
    OrderState::Pending => println!("waiting"),
    OrderState::Shipped { tracking } => println!("shipped: {}", tracking),
    OrderState::Delivered { signed_by } => println!("delivered to {}", signed_by),
    OrderState::Cancelled(reason) => println!("cancelled: {}", reason),
}
```

The compiler ensures you handle **every** state. Adding a new state later
will trigger compile errors everywhere you forgot to handle it.

## Derivable Traits

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
enum Coin { Penny, Nickel, Dime, Quarter }
```

| Trait | Useful for |
|-------|-----------|
| `Debug` | printing with `{:?}` |
| `Clone`, `Copy` | duplicating values |
| `PartialEq`, `Eq` | equality comparisons |
| `Hash` | using as a `HashMap` key |

## Why Enums Are Awesome

* **Sum types** — a value is exactly one variant, no invalid combinations.
* **Exhaustive matching** — the compiler forces you to handle every case.
* **No null** — `Option<T>` makes absence explicit.
* **No exceptions** — `Result<T, E>` makes errors part of the type system.
* **Zero cost** — enums compile down to tagged unions, just like C.

**Next:** [Practice Problems](../51_practice) — test yourself!
