# Pattern Matching (in depth)

**Patterns** are Rust's most expressive tool. They appear in `match`,
`if let`, `while let`, `let`, function parameters, `for` loops, and
closures. Once you master patterns, Rust code becomes much cleaner.

## Where Patterns Appear

| Place | Example |
|-------|---------|
| `match` arms | `match x { 1 => ..., _ => ... }` |
| `if let` | `if let Some(n) = x { ... }` |
| `while let` | `while let Some(n) = stack.pop() { ... }` |
| `let` | `let (a, b) = pair;` |
| `let else` (1.65+) | `let Some(n) = x else { return; };` |
| Function parameters | `fn f(Point { x, y }: Point) { ... }` |
| `for` loops | `for (k, v) in map { ... }` |
| Closures | `\|&(k, v)| ...` |

## Kinds of Patterns

### 1. Literals

```rust
match n {
    1 => "one",
    2 | 3 => "two or three",   // `|` combines patterns
    _ => "other",
}
```

### 2. Variable Bindings

```rust
match x {
    n => println!("{}", n),   // binds n to x
}
```

### 3. The Wildcard `_`

Matches anything and discards the value:

```rust
match x {
    1 => "one",
    _ => "other",   // catch-all
}
```

A `_` prefix on a variable name also silences "unused" warnings:

```rust
let _unused = 42;   // no warning
```

### 4. Range Patterns

```rust
match age {
    0..=12   => "child",       // inclusive: 0 to 12
    13..=19  => "teen",
    20..=64  => "adult",
    65..=120 => "senior",
    _ => "invalid",
}

// Exclusive range (Rust 1.80+):
match n {
    1..5 => "1 to 4",
    _ => "other",
}
```

Char ranges work too:

```rust
match c {
    'a'..='z' => "lowercase",
    'A'..='Z' => "uppercase",
    '0'..='9' => "digit",
    _ => "other",
}
```

### 5. Destructuring Structs

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
match p {
    Point { x, y } => println!("({}, {})", x, y),
}

// Partial — ignore some fields with `..`:
match p {
    Point { x, .. } => println!("x = {}", x),
}

// Match specific field values:
match p {
    Point { x: 0, y } => println!("on y-axis at {}", y),
    Point { x, y: 0 } => println!("on x-axis at {}", x),
    Point { x, y }   => println!("at ({}, {})", x, y),
}
```

### 6. Destructuring Tuples

```rust
let pair = (1, "hello", 3.14);
match pair {
    (1, s, _)    => println!("first=1, second={}", s),
    (n, _, _)    => println!("first={}", n),
}

// Skip elements with `..`:
match (1, 2, 3, 4, 5) {
    (first, .., last) => println!("first={}, last={}", first, last),
}
```

### 7. Destructuring Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => println!("quit"),
    Message::Move { x, y } => println!("move to ({}, {})", x, y),
    Message::Write(text)   => println!("write: {}", text),
    Message::ChangeColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
}
```

Tuple-struct variants use positional destructuring; struct variants use
named fields.

### 8. Binding with `@`

Capture a value while still testing a pattern:

```rust
match age {
    n @ 0..=12   => println!("child (age={})", n),
    n @ 13..=19  => println!("teen (age={})", n),
    n @ 20..=64  => println!("adult (age={})", n),
    n @ 65..     => println!("senior (age={})", n),
    _ => unreachable!(),
}
```

`n @ pattern` binds `n` to whatever the pattern matched.

### 9. Match Guards — `if` Condition

Add an extra condition to a pattern:

```rust
match (a, b) {
    (x, y) if x == y  => println!("equal"),
    (x, y) if x == -y => println!("opposites"),
    (x, _) if x > 0   => println!("positive first: {}", x),
    _                 => println!("other"),
}
```

Guards can use variables from the pattern and external scope. They make
patterns more expressive (e.g. comparing two matched values).

### 10. References — `&`, `ref`, `ref mut`

```rust
let x = 42;
match &x {
    &n => println!("{}", n),    // match a reference: n is i32
}

let s = String::from("hi");
match s {
    ref r => println!("{}", r),   // r: &String — borrow instead of move
}
println!("{}", s);                // still usable
```

### Match Ergonomics (Modern Rust)

Since edition 2018, the compiler automatically inserts `ref` where needed:

```rust
let v: Vec<Option<i32>> = vec![Some(1), None];
for opt in &v {
    match opt {
        Some(n) => println!("some: {}", n),   // n: &i32 (auto)
        None    => println!("none"),
    }
}
```

You rarely need explicit `ref` / `ref mut` anymore.

### 11. Subpatterns — Nesting

Patterns can nest:

```rust
let nested: Option<Option<i32>> = Some(Some(42));
match nested {
    Some(Some(n)) => println!("doubly wrapped: {}", n),
    Some(None)    => println!("inner None"),
    None          => println!("outer None"),
}
```

## `if let` — Single-Pattern Match

When you only care about one variant:

```rust
if let Some(n) = maybe {
    println!("{}", n);
} else {
    println!("none");
}
```

Equivalent to:

```rust
match maybe {
    Some(n) => println!("{}", n),
    None    => println!("none"),
}
```

## `let else` (Rust 1.65+) — Early Return

Refutable patterns in `let` with an early return on no-match:

```rust
let config: Option<&str> = Some("localhost");
let hostname: &str = let Some(h) = config else {
    panic!("hostname is required");
};
// hostname is now a &str (not Option)
```

Great for "unwrap or return early" patterns:

```rust
fn process(input: &str) -> Result<(), String> {
    let n: i32 = let Ok(n) = input.parse() else {
        return Err("not a number".to_string());
    };
    // ... use n ...
    Ok(())
}
```

## `while let` — Loop While Pattern Matches

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);    // 3 2 1
}
```

Equivalent to:

```rust
loop {
    match stack.pop() {
        Some(top) => println!("{}", top),
        None      => break,
    }
}
```

## Patterns in `let` Statements

```rust
let (a, b, c) = (1, 2, 3);
let Point { x, y } = p;

// Partial:
let (first, .., last) = (1, 2, 3, 4, 5);
```

The pattern must be **irrefutable** (always matches). For refutable
patterns, use `let else` or `if let`.

## Patterns in Function Parameters

```rust
fn print_point(Point { x, y }: Point) {
    println!("({}, {})", x, y);
}

fn first((a, _): (i32, i32)) -> i32 { a }
```

## Patterns in `for` Loops

```rust
for (k, v) in &hashmap {
    println!("{} -> {}", k, v);
}

for (i, x) in vec.iter().enumerate() {
    println!("[{}] = {}", i, x);
}
```

## Patterns in Closures

```rust
let sum = |(a, b): (i32, i32)| a + b;
sum((1, 2));   // 3
```

## Exhaustiveness

`match` must cover every possible value. The compiler enforces this:

```rust
enum Color { Red, Green, Blue }

match c {
    Color::Red => ...,
    Color::Green => ...,
    // ERROR: non-exhaustive; Color::Blue not covered
}
```

This is a **feature**: if you add a new variant later, the compiler
forces you to handle it everywhere.

## Refutability

| Pattern type | Always matches? |
|--------------|------------------|
| Irrefutable | yes (`(a, b)`, `Point { x, y }`) |
| Refutable | no (`Some(x)`, `1`, `0..=10`) |

* `let` requires irrefutable patterns (or use `let else`).
* `if let` / `while let` accept refutable patterns.
* `match` accepts refutable patterns but must be exhaustive.

## Pattern Cheat Sheet

```
Literal                | 1, "hello", 'a', true
Variable binding       | x
Wildcard               | _
Range                  | 1..=10, 'a'..='z'
Or                     | 1 | 2 | 3
Struct destructure     | Point { x, y }
Struct partial         | Point { x, .. }
Struct literal match   | Point { x: 0, y }
Tuple destructure      | (a, b, c)
Tuple partial          | (first, .., last)
Enum destructure       | Some(x), Message::Move { x, y }
Binding with @         | n @ 1..=10
Guard                  | pattern if condition
Reference              | &x, ref x, ref mut x
Subpattern             | Some(Some(n))
```

## Common Patterns

### Unwrap Option with Default

```rust
let n = match opt {
    Some(n) => n,
    None    => 0,
};
// Or shorter:
let n = opt.unwrap_or(0);
```

### Match on Multiple Values

```rust
match (a, b) {
    (Some(x), Some(y)) => ...,
    (Some(x), None)    => ...,
    (None, Some(y))    => ...,
    (None, None)       => ...,
}
```

### Match on Enum with Data

```rust
match msg {
    Message::Quit => ...,
    Message::Move { x: 0, y: 0 } => println!("at origin"),
    Message::Move { x, y }       => println!("at ({}, {})", x, y),
    _ => ...,
}
```

### Validate and Bind

```rust
match user_input.parse() {
    Ok(n) if n > 0 => process(n),
    Ok(_)          => println!("must be positive"),
    Err(e)         => println!("parse error: {}", e),
}
```

## When to Use Match vs `if let`

| | `match` | `if let` |
|---|---------|----------|
| Handles all arms | yes (exhaustive) | no (one arm + fallback) |
| Returns a value | yes | no (statement form) |
| When to use | multiple cases | one case + fallback |

## Common Pitfalls

### Forgetting the Catch-All

```rust
match x {
    1 => ...,
    2 => ...,
    // ERROR: non-exhaustive; i32 has other values
}
```

Add `_ => ...` or handle every case explicitly.

### Shadowing Variables in Guards

```rust
let x = 5;
match 10 {
    x if x > 3 => println!("{}", x),   // x is the matched value (10), not 5
    _ => ...,
}
```

The pattern variable shadows the outer one.

### Match Order Matters

```rust
match n {
    _ => "other",     // catches EVERYTHING — arms below are unreachable
    1 => "one",       // WARNING: unreachable pattern
}
```

Patterns are tried top-to-bottom. Put specific cases first, catch-all last.

**Next:** [File I/O & std::fs](../43_file_io) — reading/writing files and directories.
