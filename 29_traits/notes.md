# Rust Traits

A **trait** defines a set of methods a type must implement. It is Rust's
equivalent of an **interface** (Java), **type class** (Haskell), or
**protocol** (Swift).

## Defining a Trait

```rust
trait Summary {
    fn summarize(&self) -> String;        // required method

    fn preview(&self) -> String {         // default method (optional)
        format!("{}...", &self.summarize()[..20])
    }
}
```

## Implementing a Trait for a Type

```rust
struct Article { title: String, author: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
    // preview() uses the default — we don't have to write it.
}
```

You can implement your own traits on your own types, your traits on
foreign types, or foreign traits on your own types — but **not** a foreign
trait on a foreign type (the "orphan rule").

## Trait Bounds on Generics

```rust
fn announce<T: Summary>(item: &T) -> String {
    format!("BREAKING: {}", item.summarize())
}
```

`T: Summary` means "T must implement Summary". Inside the function we can
only call `Summary` methods on `T`.

### Multiple Bounds

```rust
fn show<T: Summary + std::fmt::Debug>(item: &T) { /* ... */ }
```

### `where` Clauses

When bounds get long, use `where`:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: std::fmt::Debug,
{
    /* ... */
}
```

## `impl Trait` Syntax (Sugar)

```rust
fn announce(item: &impl Summary) -> String { /* ... */ }
```

Equivalent to `<T: Summary>` but more readable for simple cases. Also works
as a return type:

```rust
fn make() -> impl Summary { Article { /* ... */ } }
```

## Default Methods

A trait method can have a body — that becomes the default implementation.
Types can use the default or override it:

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String { format!("@{}", self.username) }
    fn preview(&self) -> String { self.summarize() }   // override
}
```

## Trait Objects — `dyn Trait`

When you don't know the concrete type at compile time, use a **trait
object**:

```rust
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(article),
    Box::new(tweet),
];
```

* `dyn Trait` uses **dynamic dispatch** (vtable, like virtual methods in C++).
* Slight runtime cost vs generics (which use **static dispatch** = monomorphization).
* Enables heterogeneous collections (multiple types in one `Vec`).

| | Generics `<T: Trait>` | Trait objects `dyn Trait` |
|---|------------------------|---------------------------|
| Dispatch | static (compile time) | dynamic (runtime) |
| Speed | faster | slight overhead |
| Binary size | larger (per-type code) | smaller (shared code) |
| Heterogeneous collections | no | yes |

## Common Standard Traits

| Trait | What it gives you |
|-------|-------------------|
| `Display` | `{}` formatting for users |
| `Debug` | `{:?}` formatting for developers |
| `Clone` | `.clone()` deep copy |
| `Copy` | implicit copy on assignment |
| `PartialEq`, `Eq` | `==`, `!=` |
| `PartialOrd`, `Ord` | `<`, `>`, sorting |
| `Hash` | use as a `HashMap` key |
| `Default` | `T::default()` |
| `Iterator` | `.next()`, `.map()`, etc. |
| `From`, `Into` | type conversions |
| `Drop` | destructor — runs when value goes out of scope |

## Derivable Traits

Many standard traits can be auto-derived:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }
```

The compiler writes the implementations for you.

## Implementing `Display` Manually

```rust
use std::fmt;

struct Kelvin(f64);

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} K", self.0)
    }
}

let t = Kelvin(273.15);
println!("{}", t);   // "273.15 K"
```

## Associated Types

A trait can declare a placeholder type that the implementor chooses:

```rust
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

struct NumberList(Vec<i32>);
impl Container for NumberList {
    type Item = i32;
    fn first(&self) -> Option<&i32> { self.0.first() }
}
```

Unlike generics, there can only be **one** `impl Container for NumberList`
— the associated type is part of the implementation, not chosen by the
caller.

## Trait Inheritance

```rust
trait Circle: Shape {     // any Circle must also be a Shape
    fn radius(&self) -> f64;
}
```

`Circle: Shape` means "to implement `Circle`, you must also implement
`Shape`".

## The Orphan Rule

You can implement a trait on a type **only if** at least one of them is
local to your crate. This prevents two crates from accidentally defining
conflicting impls.

## When to Use Traits

* **Abstraction** — define common behaviour across types (e.g. `Display`).
* **Polymorphism** — `Vec<Box<dyn Animal>>` for heterogeneous collections.
* **Generic constraints** — `fn max<T: Ord>(...)`.
* **Extension methods** — add methods to foreign types via your own trait.

**Next:** [Generics](../30_generics) — write code that works on many types.
