# Rust Functions

Functions are reusable blocks of code. Use the `fn` keyword to declare them.

## Basic Function

```rust
fn say_hello() {
    println!("Hello!");
}
```

## Parameters (must have types)

```rust
fn greet(name: &str) {
    println!("Hi, {}!", name);
}
```

Rust does **not** infer parameter types — you must annotate them.

## Return Type

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b          // last expression, NO semicolon, is the return value
}
```

* `-> Type` declares the return type.
* The **last expression** in the block (with no trailing semicolon) is the
  return value.
* You don't need the `return` keyword — except for early returns.

## Early Return

```rust
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;   // early return
    }
    x                // normal return
}
```

## Multiple Parameters

```rust
fn build_info(name: &str, age: u32, is_admin: bool) -> String {
    format!("{} ({} years old){}", name, age, if is_admin { " [admin]" } else { "" })
}
```

## Returning a Tuple

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

## Statements vs Expressions

| | Statement | Expression |
|---|-----------|------------|
| Returns | `()` (unit) | a value |
| Semicolon | yes | no |
| Example | `let x = 5;` | `5 + 6` |

```rust
fn area(width: i32, height: i32) -> i32 {
    let product = width * height;   // statement
    product                         // expression -> the return value
}
```

Adding a `;` to an expression turns it into a statement (returning `()`).
Forgetting to remove the trailing `;` is the #1 source of "expected i32,
found ()" errors in Rust.

## Diverging Functions: `!`

A function that **never returns** (loops forever, panics, or exits the
process) has return type `!`:

```rust
fn infinite() -> ! {
    loop { /* ... */ }
}
```

`!` can be used where any type is expected, because the function never
produces a value.

## Default Arguments?

Rust does not have default arguments. Use `Option<T>` or builder pattern:

```rust
fn greet_optional(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None    => println!("Hello, stranger!"),
    }
}
```

## Closures (Anonymous Functions)

```rust
let double = |x: i32| x * 2;
println!("{}", double(5));     // 10

let multiplier = 3;
let multiply = |x| x * multiplier;   // captures `multiplier` from the env
```

Closures can capture their environment — regular functions cannot.

## Why Functions?

* Reuse: write once, call many times.
* Abstraction: hide complexity behind a name.
* Testing: small, focused functions are easier to test.
* Readability: a good function name replaces a comment.

**Next:** [Scope](../12_scope) — where variables live.
