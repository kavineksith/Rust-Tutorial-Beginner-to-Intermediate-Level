# Rust Error Handling

Rust has **no exceptions**. Instead, errors are **values**, represented
by the `Result` type. This makes error handling explicit, type-checked,
and impossible to forget.

## Two Kinds of Errors

| | Recoverable | Unrecoverable |
|---|-------------|---------------|
| Type | `Result<T, E>` | `panic!` |
| When to use | expected failures (I/O, parse, network) | bugs, impossible states |
| Behaviour | propagate or handle | unwind the stack, exit |

## Unrecoverable: `panic!`

```rust
panic!("something is very wrong");
```

When `panic!` runs, Rust prints an error message, unwinds the stack
(running destructors), and exits the process. Use it for:

* Bugs that should never happen.
* Violated invariants.
* Array index out of bounds (Rust panics automatically).

```rust
let v = vec![1, 2, 3];
let _x = v[10];   // PANIC: index out of bounds
```

## Recoverable: `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Use it for any operation that can fail in expected ways.

```rust
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

match parse_number("42") {
    Ok(n)  => println!("parsed: {}", n),
    Err(e) => println!("error: {}", e),
}
```

## The `?` Operator — Propagating Errors

Inside a function that returns `Result`, `?` returns early on `Err`:

```rust
fn read_number(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;   // returns Err if open fails
    let n: i32 = content.trim().parse()?;          // returns Err if parse fails
    Ok(n)
}
```

`?` is sugar for:

```rust
match expr {
    Ok(v)  => v,
    Err(e) => return Err(e.into()),
}
```

The `.into()` lets `?` convert error types automatically — see `From` impls
below.

## Useful `Result` Methods

```rust
let ok: Result<i32, &str>  = Ok(5);
let err: Result<i32, &str> = Err("nope");

ok.unwrap();                    // 5 (panics if Err)
err.unwrap_or(0);              // 0
err.unwrap_or_default();       // 0 (i32::default())
err.unwrap_or_else(|e| -1);    // -1
ok.map(|x| x * 2);             // Ok(10)
err.map_err(|e| e.to_uppercase()); // Err("NOPE")
ok.is_ok();   err.is_err();
ok.ok();   // Some(5)  (Result -> Option)
```

| Method | Behaviour |
|--------|-----------|
| `unwrap()` | panic on `Err`, value on `Ok` |
| `expect("msg")` | like `unwrap` with a custom message |
| `unwrap_or(v)` | use `v` on `Err` |
| `unwrap_or_default()` | use `T::default()` on `Err` |
| `unwrap_or_else(f)` | compute fallback on `Err` |
| `map(f)` | transform `Ok` value |
| `map_err(f)` | transform `Err` value |
| `?` | propagate `Err` |

## Custom Error Type

For libraries with multiple failure modes, define your own error enum:

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
    Empty,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e)    => write!(f, "I/O error: {}", e),
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Empty    => write!(f, "input was empty"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e)    => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::Empty    => None,
        }
    }
}
```

## `From` Impl — Let `?` Auto-Convert

For `?` to convert underlying errors to your custom error type, implement
`From` for each:

```rust
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn read_number(filename: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(filename)?;   // io::Error -> AppError
    let n: i32 = content.trim().parse()?;          // ParseIntError -> AppError
    Ok(n)
}
```

Now `?` works for both `io::Error` and `ParseIntError` — they're
auto-converted to `AppError`.

## `Box<dyn Error>` — Quick Prototyping

When you don't want to define your own error type:

```rust
fn do_something() -> Result<i32, Box<dyn Error>> {
    let n = parse()?;          // any error converts to Box<dyn Error>
    Ok(n)
}
```

`Box<dyn Error>` accepts any error type via `From`. Great for examples
and small scripts; less clean for libraries.

## The `thiserror` and `anyhow` Crates (Real Projects)

| Crate | Use case |
|-------|----------|
| `thiserror` | ergonomic custom error enums for libraries |
| `anyhow` | application-level error chaining with context |

### `thiserror` Example

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseIntError),
    #[error("input was empty")]
    Empty,
}
```

`#[from]` auto-generates the `From` impl — much less boilerplate.

### `anyhow` Example

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("failed to read config.toml")?;
    Ok(content)
}
```

`anyhow::Result` uses `anyhow::Error` which can hold any error plus
context strings. Perfect for applications.

## `Option` vs `Result`

| | `Option<T>` | `Result<T, E>` |
|---|-------------|----------------|
| Meaning | maybe nothing | success or failure |
| Variants | `Some` / `None` | `Ok` / `Err` |
| Use when | absence is normal | failure is exceptional |
| Convert | `.ok()` (Result→Option) | `.ok_or(err)` (Option→Result) |

```rust
let opt: Option<i32> = None;
let res: Result<i32, &str> = opt.ok_or("was none");
```

## Error Handling Philosophy

1. **Make impossible states impossible** — use enums, not strings.
2. **Handle errors at the right level** — don't catch too early, don't
   propagate forever.
3. **Use `?` for propagation** — keeps happy path readable.
4. **Add context as you go up** — `anyhow::Context` or custom messages.
5. **Reserve `panic!` for bugs** — never for expected failure.
6. **Don't unwrap in production** — except where failure is truly impossible.

## Common Patterns

### Fallible Parsing

```rust
let n: i32 = "42".parse()?;
```

### Fallible I/O

```rust
let content = fs::read_to_string("file.txt")?;
```

### Multiple Fallible Steps

```rust
fn load_user(id: u32) -> Result<User, AppError> {
    let row = db.find_user(id)?;       // may fail
    let user: User = serde_json::from_str(&row)?;  // may fail
    Ok(user)
}
```

### Retry on Error

```rust
for attempt in 1..=3 {
    match do_thing() {
        Ok(v)  => return Ok(v),
        Err(e) if attempt == 3 => return Err(e),
        Err(_)  => continue,
    }
}
```

## Summary Table

| Want | Use |
|------|-----|
| Crash on bug | `panic!` / `unwrap()` / `expect()` |
| Propagate error | `?` |
| Provide fallback | `unwrap_or` / `unwrap_or_else` |
| Transform Ok value | `map` |
| Transform Err value | `map_err` |
| Custom error type | enum + `Display` + `Error` + `From` |
| Quick prototyping | `Box<dyn Error>` |
| Library | `thiserror` |
| Application | `anyhow` |

**Next:** [Modules & Crates](../41_modules_crates) — organizing larger programs.
