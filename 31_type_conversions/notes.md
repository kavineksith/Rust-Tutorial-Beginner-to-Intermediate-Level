# Type Conversions

Rust is **strict about types** — there are no implicit conversions. Every
conversion is explicit, which prevents a whole class of bugs.

## The Conversion Toolkit

| Goal | Use |
|------|-----|
| Primitive numeric cast (`i32` → `u8`) | `as` (may truncate) |
| Infallible type conversion | `From` / `Into` |
| Fallible type conversion | `TryFrom` / `TryInto` |
| Cheap reference-to-reference | `AsRef` / `AsMut` |
| Smart pointer deref | `Deref` / `DerefMut` (auto for `Box`, `Rc`, etc.) |
| `&str` → `String` | `.to_string()`, `String::from(...)`, `.to_owned()` |
| `String` → `&str` | `&s` (free) |
| `&str` → number | `.parse::<T>()` (returns `Result`) |

## `as` — Primitive Numeric Casts

Only works on primitive numeric types. May truncate or lose precision:

```rust
let big: i64 = 1_000_000;
let small: i32 = big as i32;     // may truncate on overflow

let float: f64 = 3.99;
let int_part: i32 = float as i32;   // truncates toward zero -> 3
```

Use sparingly. For safe casts, use `TryFrom`.

## `From` and `Into` — Infallible Conversions

`From<T> for U` says "you can always convert `T` into `U`". Implementing
`From` gives you `Into` for free.

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

let c = Celsius(100.0);
let f: Fahrenheit = c.into();             // via Into (auto from From)
let f2 = Fahrenheit::from(Celsius(0.0));  // explicit From
```

### Implementing `From` for Multiple Source Types

```rust
struct UserId(String);

impl From<String> for UserId {
    fn from(s: String) -> Self { UserId(s) }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self { UserId(s.to_string()) }
}

let u1: UserId = "ada".into();                  // via From<&str>
let u2: UserId = String::from("ada").into();    // via From<String>
```

### Standard Library `From` Impls

```rust
let s: String = String::from("hello");
let v: Vec<u8> = Vec::from(&[1, 2, 3][..]);
let n: i32 = i32::from(42u8);
```

## `TryFrom` and `TryInto` — Fallible Conversions

When conversion can fail, use `TryFrom`. It returns `Result<Self, Error>`.

```rust
use std::convert::TryFrom;

struct PositiveInt(u32);

impl TryFrom<i32> for PositiveInt {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("value must be non-negative")
        } else {
            Ok(PositiveInt(value as u32))
        }
    }
}

let good: Result<PositiveInt, _> = PositiveInt::try_from(5);
let bad:  Result<PositiveInt, _> = PositiveInt::try_from(-1);
```

`try_into()` is the auto-derived counterpart:

```rust
use std::convert::TryInto;

let n: Result<PositiveInt, _> = 10i32.try_into();
```

## `AsRef` and `AsMut` — Cheap Reference-to-Reference

`AsRef<T>` lets a type expose a `&T` view of itself. Functions that accept
`impl AsRef<str>` work with `&str`, `String`, `&String`, `Path`, `&Path`,
etc.

```rust
fn print_length<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref().len());
}

print_length("literal");                       // &str
print_length(String::from("owned"));           // String
print_length(&String::from("borrowed"));       // &String
```

### Implementing `AsRef` for Your Own Type

```rust
struct Wrapper(String);

impl AsRef<str> for Wrapper {
    fn as_ref(&self) -> &str { &self.0 }
}

print_length(Wrapper("wrapped".to_string()));
```

### `AsMut` — Mutable Variant

```rust
fn fill_with_zero<T: AsMut<[i32]>>(s: &mut T) {
    for x in s.as_mut() { *x = 0; }
}
```

## `Deref` and `DerefMut` — Deref Coercion

Smart pointers like `Box<T>`, `Rc<T>`, `Arc<T>`, `String`, `Vec<T>`
implement `Deref`. This lets you call methods on `T` directly through the
pointer.

```rust
use std::ops::Deref;

struct MyString(String);

impl Deref for MyString {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

let m = MyString(String::from("hi"));
println!("{}", m.len());     // String::len, via Deref
println!("{}", m.to_uppercase());  // str::to_uppercase, via double Deref
```

### Deref Coercion Chain

`&MyString` → `&String` → `&str` — the compiler walks the chain
automatically.

### The Newtype Pattern

Wrap a type in a single-field struct to give it a distinct type. Implement
`Deref` to expose the inner type's methods:

```rust
struct Meters(f64);
struct Feet(f64);

impl Deref for Meters { type Target = f64; /* ... */ }
impl Deref for Feet   { type Target = f64; /* ... */ }
```

Now `Meters` and `Feet` are distinct types — you can't accidentally add
them — but they still act like `f64`.

### When NOT to Implement `Deref`

Don't use `Deref` to emulate inheritance. It's intended for smart pointers
and newtypes that wrap a single value. Misusing it produces confusing
behavior.

## String Conversions

### `&str` → `String`

```rust
let s1: String = "hello".to_string();
let s2: String = String::from("hello");
let s3: String = "hello".to_owned();
let s4: String = format!("{} {}", "hello", "world");
```

All four are equivalent. Use `.to_string()` for readability,
`String::from(...)` for explicitness, `format!` for combinations.

### `String` → `&str`

Free — just take a reference:

```rust
let s = String::from("hello");
let borrowed: &str = &s;
```

### `&str` → Number

```rust
let n: i32 = "42".parse().unwrap();
let f: f64 = "3.14".parse().unwrap();
let bad: Result<i32, _> = "abc".parse();    // Err
```

`.parse::<T>()` works on anything that implements `FromStr`.

### Number → `String`

```rust
let s: String = 42.to_string();
let s2: String = format!("answer = {}", 42);
```

### `String` ↔ `Vec<u8>`

```rust
let s = String::from("hello");
let bytes: Vec<u8> = s.into_bytes();           // free (no copy)

let bytes = vec![104, 105];                    // "hi"
let s: String = String::from_utf8(bytes).unwrap();  // fallible
```

## `ToString` and `Display`

If your type implements `Display`, it gets `ToString` for free:

```rust
use std::fmt;

struct Kelvin(f64);

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} K", self.0)
    }
}

let k = Kelvin(273.15);
let s: String = k.to_string();   // "273.15 K"
```

## Conversion Cheat Sheet

```rust
// Primitive numeric:
let n = 42i32 as u8;

// Infallible:
let s: String = "hi".to_string();

// Fallible:
let n: Result<i32, _> = "42".parse();

// From/Into (custom):
let f: Fahrenheit = Celsius(100.0).into();

// TryFrom/TryInto (custom):
let p: Result<PositiveInt, _> = (-1i32).try_into();

// AsRef:
fn take_str(s: impl AsRef<str>) { /* ... */ }

// Deref coercion (automatic):
let v: Vec<i32> = vec![1, 2, 3];
let slice: &[i32] = &v;     // Vec derefs to slice
```

## Common Pitfalls

### Forgetting `as` Doesn't Check Overflow

```rust
let big: i64 = 1_000_000_000_000;
let small: i32 = big as i32;   // silently truncates!
```

Use `TryFrom` for safety:

```rust
use std::convert::TryFrom;
let small: Result<i32, _> = i32::try_from(big);
```

### Confusing `into()` with `as`

```rust
let n = 42i32;
// let s: String = n.into();   // ERROR: i32 doesn't impl Into<String>
let s: String = n.to_string(); // OK
```

`into()` only works for types that implement `Into` for the target.
Primitives don't implement `Into<String>` — use `to_string()` instead.

### `parse` Returns `Result`

```rust
let n: i32 = "42".parse();   // ERROR: parse returns Result, not i32
let n: i32 = "42".parse().unwrap();   // OK
```

Always handle the `Result` (or use `.unwrap()` / `?`).

## When to Implement Your Own Conversions

| Trait | Implement when |
|-------|---------------|
| `From<T>` | you can always convert from `T` |
| `TryFrom<T>` | conversion may fail |
| `AsRef<T>` | your type exposes a `&T` view |
| `AsMut<T>` | your type exposes a `&mut T` view |
| `Deref` | your type is a smart pointer / newtype (use sparingly) |
| `Display` | your type has a user-facing string representation |
| `FromStr` | your type can be parsed from a string |

**Next:** [Common Traits](../32_common_traits) — `Copy`, `Clone`, `Drop`, `Default`.
