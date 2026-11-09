# Common Traits (Copy, Clone, Drop, Default, ...)

Rust's standard library defines a set of common traits that almost every
type implements. Most are **derivable** — the compiler writes the
implementation for you.

## The Most Common Derivable Traits

| Trait | What it gives you |
|-------|-------------------|
| `Debug` | `println!("{:?}", x)` (developer view) |
| `Clone` | `x.clone()` (explicit deep copy) |
| `Copy` | implicit copy on assignment (small types only) |
| `PartialEq`, `Eq` | `==`, `!=` |
| `PartialOrd`, `Ord` | `<`, `>`, sorting |
| `Hash` | use as `HashMap`/`HashSet` key |
| `Default` | `T::default()` (a natural empty value) |

## `Clone` — Explicit Deep Copy

`Clone` says "you can make a deep copy of me via `.clone()`".

```rust
#[derive(Clone, Debug)]
struct Point { x: i32, y: i32 }

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();   // explicit deep copy
```

Almost every type can derive `Clone`. Exceptions: types holding non-Clone
fields (e.g. a `MutexGuard`).

## `Copy` — Implicit Bitwise Copy

`Copy` is a **marker trait** (no methods). It says "this type is so cheap
to copy that the compiler does it implicitly on every read."

```rust
#[derive(Copy, Clone, Debug)]
struct Color { r: u8, g: u8, b: u8 }

let c1 = Color { r: 255, g: 128, b: 0 };
let c2 = c1;    // IMPLICITLY COPIED — no .clone() needed
println!("{:?} {:?}", c1, c2);   // both still usable
```

### Rules

* The type must be small (no heap allocation, no shared resources).
* All fields must themselves be `Copy`.
* If you implement `Copy`, you must also implement `Clone`. Usually:
  `#[derive(Copy, Clone)]`.

### When NOT to Derive `Copy`

| Type | Why not Copy |
|------|--------------|
| `String` | owns heap memory; copying would duplicate the allocation |
| `Vec<T>` | same |
| `Box<T>` | represents unique ownership of heap memory |
| `HashMap` | owns heap memory and a hasher |
| Anything containing these | same |

For these, derive only `Clone` — callers must explicitly `.clone()`.

### `Copy` vs `Clone`

| | `Copy` | `Clone` |
|---|--------|---------|
| Implicit? | yes, on every read | no, requires `.clone()` |
| Cost | bitwise (cheap) | arbitrary (may allocate) |
| Allowed for | small, all-Copy types | almost any type |
| Method | none (marker) | `.clone(&self) -> Self` |

## `Drop` — The Destructor

`Drop` runs automatically when a value goes out of scope. It's how Rust
frees memory, closes files, releases locks — **without `free()`**.

```rust
struct LogOnDrop { name: &'static str }
impl Drop for LogOnDrop {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn foo() {
    let _a = LogOnDrop { name: "a" };
    {
        let _b = LogOnDrop { name: "b" };
    }   // <- b dropped here
}   // <- a dropped here
```

### When to Implement `Drop` Manually

* Custom resource cleanup (FFI types, file handles).
* Logging when something is dropped (debugging).
* Invariants that must be checked at drop time.

You rarely need to — `Vec`, `String`, `Box`, `Mutex`, etc. all implement
`Drop` for you.

### Early Drop with `std::mem::drop`

```rust
let file = open_file();
do_something(&file);
drop(file);   // close the file NOW, before end of scope
do_other_work_without_file();
```

`drop()` is just `std::mem::drop` — it takes ownership and immediately
drops the value.

### Drop Order

* Local variables drop in **reverse declaration order**.
* Struct fields drop in **declaration order**.
* Cannot call methods on a partially-dropped value.

## `Default` — A Sensible Empty Value

`Default::default()` returns a "natural" zero/empty value:

| Type | Default |
|------|---------|
| Numbers | `0` |
| `String` | `""` |
| `Vec<T>` / `HashMap` | empty |
| `bool` | `false` |
| `Option<T>` | `None` |

```rust
#[derive(Debug, Default)]
struct Config {
    host: String,
    port: u16,
    retries: u32,
    debug: bool,
}

let c = Config::default();
// Config { host: "", port: 0, retries: 0, debug: false }

// Partial initialization with struct update syntax:
let custom = Config {
    host: String::from("localhost"),
    port: 8080,
    ..Default::default()
};
```

## `PartialEq` and `Eq` — Equality

| Trait | Meaning |
|-------|---------|
| `PartialEq` | `==` and `!=` work |
| `Eq` | marker: equality is **reflexive** (`a == a` always) |

Floating point types implement `PartialEq` but **not** `Eq`, because
`NaN != NaN` violates reflexivity.

```rust
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

let u1 = UserId(1);
let u2 = UserId(1);
println!("{}", u1 == u2);   // true
```

## `PartialOrd` and `Ord` — Ordering

| Trait | Meaning |
|-------|---------|
| `PartialOrd` | `<`, `>`, `<=`, `>=` work |
| `Ord` | marker: total order (every two values are comparable) |

Floating point types implement `PartialOrd` but **not** `Ord` (NaN breaks
totality).

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u32, u32, u32);

let mut v = vec![Version(2, 0, 0), Version(1, 0, 0), Version(1, 2, 0)];
v.sort();
// [Version(1, 0, 0), Version(1, 2, 0), Version(2, 0, 0)]
```

`Ord` is required for `BTreeMap` keys, `BTreeSet` members, and `.sort()`.

## `Hash` — HashMap Key

```rust
#[derive(Debug, PartialEq, Eq, Hash)]
struct Email(String);

let mut map: std::collections::HashMap<Email, &str> = HashMap::new();
map.insert(Email("a@b.com".to_string()), "Ada");
```

Required for `HashMap` and `HashSet` keys. Just derive it.

## `Display` — User-Facing String

`Display` lets your type print with `{}` (vs `{:?}` for Debug). **Cannot
be derived** — implement it manually.

```rust
use std::fmt;

struct Kelvin(f64);

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.*} K", precision, self.0)
        } else {
            write!(f, "{:.2} K", self.0)
        }
    }
}

let k = Kelvin(273.15);
println!("{}", k);       // "273.15 K"
println!("{:.4}", k);    // "273.1500 K"
```

The `Formatter` supports precision, width, alignment, fill, sign — all
the usual `printf`-like flags.

## `Debug` — Developer-Facing String

Derived with `#[derive(Debug)]`. Prints with `{:?}` (single-line) or
`{:#?}` (pretty multi-line):

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

let p = Point { x: 1, y: 2 };
println!("{:?}", p);     // Point { x: 1, y: 2 }
println!("{:#?}", p);    // multi-line pretty version
```

You can also implement `Debug` manually for cleaner output.

## What to Derive by Default

For a typical data struct:

```rust
#[derive(Debug, Clone, PartialEq)]
struct MyType { /* ... */ }
```

* Add `Eq` if your type is fully orderable (no NaN-like values).
* Add `Hash` if you want to use it as a `HashMap` key.
* Add `Copy` only if it's a small, all-Copy-able type.
* Add `Default` if there's a natural "empty" value.
* Add `PartialOrd`/`Ord` if you want to sort it.
* Never derive `Display` — implement it manually.

## Trait Summary

| Trait | Derivable? | Method(s) |
|-------|------------|-----------|
| `Debug` | yes | (used by `{:?}`) |
| `Clone` | yes | `.clone()` |
| `Copy` | yes | (marker) |
| `PartialEq` | yes | `==`, `!=` |
| `Eq` | yes | (marker) |
| `PartialOrd` | yes | `<`, `>`, `<=`, `>=` |
| `Ord` | yes | `.cmp()`, `.min()`, `.max()` |
| `Hash` | yes | `.hash()` |
| `Default` | yes | `T::default()` |
| `Display` | **no** | (used by `{}`) |
| `FromStr` | no | `.from_str()` |
| `From`/`Into` | sometimes | `.into()` |
| `TryFrom`/`TryInto` | sometimes | `.try_into()` |
| `AsRef`/`AsMut` | sometimes | `.as_ref()` |
| `Drop` | no | `.drop()` (auto-called) |
| `Iterator` | no | `.next()` |

## Memory Layout Notes

* `Copy` types are typically stack-allocated and small.
* `Clone` types may own heap memory — `.clone()` may allocate.
* `Drop` types have a destructor — they cannot be `Copy`.
* `Default` types have a "natural" zero value.

**Next:** [Pattern Matching (in depth)](../33_pattern_matching) — destructuring, guards, bindings.
