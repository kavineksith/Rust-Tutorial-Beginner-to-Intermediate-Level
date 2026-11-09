# Rust Match

`match` compares a value against a series of **patterns** and runs the code
for the first pattern that fits. It is Rust's most powerful control-flow tool.

## Basic Match

```rust
let number = 3;
match number {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("something else"),  // `_` matches anything
}
```

* Each `=>` arm is `pattern => expression`.
* `_` is the **wildcard** — matches any value.
* Arms are checked **top to bottom**; the first match wins.
* Match must be **exhaustive** — every possible value must be covered. The
  compiler errors if you miss a case.

## Match Is an Expression

```rust
let result = match number {
    1 => "one",
    2 => "two",
    _ => "many",
};
```

Every arm must return the **same type**.

## Combining Patterns with `|`

```rust
match n {
    1 | 2 | 3 => println!("small"),
    _         => println!("big"),
}
```

## Range Patterns

```rust
match n {
    1..=5   => println!("1-5"),     // inclusive range
    6..=10  => println!("6-10"),
    _       => println!("other"),
}
```

## Matching `Option<T>`

```rust
let maybe: Option<i32> = Some(5);
let v = match maybe {
    Some(x) => x,
    None    => 0,
};
```

This is the idiomatic way to safely extract a value from an `Option`.

## Matching Enums

```rust
enum Coin { Penny, Nickel, Dime, Quarter }
let coin = Coin::Dime;
let v = match coin {
    Coin::Penny   => 1,
    Coin::Nickel  => 5,
    Coin::Dime    => 10,
    Coin::Quarter => 25,
};
```

## Destructuring

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
match msg {
    Message::Quit => println!("quit"),
    Message::Move { x, y } => println!("move to ({}, {})", x, y),
    Message::Write(s) => println!("write: {}", s),
}
```

## Match Guards

Add an extra `if` condition to a pattern:

```rust
match (a, b) {
    (x, y) if x == y  => println!("equal"),
    (x, y) if x == -y => println!("opposite"),
    _                 => println!("other"),
}
```

## Binding with `@`

Capture a matched value while still testing a pattern:

```rust
match age {
    n @ 0..=12   => println!("child: {}", n),
    n @ 13..=19  => println!("teen: {}", n),
    n @ 20..     => println!("adult: {}", n),
}
```

## `match` vs `if let`

| | `match` | `if let` |
|---|---------|----------|
| Handles all arms | yes (exhaustive) | no (one arm only) |
| Returns a value | yes | no (statement form) |
| When to use | multiple cases | one case + fallback |

## Exhaustiveness

Rust forces you to handle every case — or use `_` as a catch-all. This
prevents bugs when you later add a new variant to an enum.

**Next:** [Loops](../14_loops) — repeating code.
