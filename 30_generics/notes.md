# Rust Generics

**Generics** let you write code that works on many types without
duplication. Rust uses **monomorphization** — the compiler generates a
separate copy of the code for each concrete type used. Result: **zero
runtime cost**.

## Generic Functions

```rust
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T {
    let mut max = slice[0];
    for &item in slice.iter() {
        if item > max { max = item; }
    }
    max
}

largest(&[3, 1, 4]);           // works on i32
largest(&['a', 'z', 'm']);     // works on char
largest(&[1.5, 2.5, 0.5]);     // works on f64
```

* `T` is a **type parameter** (single uppercase letter by convention).
* `T: PartialOrd + Copy` is a **trait bound** — T must support `<`/`>` and be `Copy`.
* `+` combines multiple bounds.

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let p1 = Point { x: 5, y: 10 };         // Point<i32>
let p2 = Point { x: 1.0, y: 2.0 };      // Point<f64>
// let p3 = Point { x: 1, y: 2.0 };     // ERROR: T must be the same!
```

### Multiple Type Parameters

```rust
struct Pair<A, B> {
    first: A,
    second: B,
}

let p = Pair { first: 42, second: String::from("hi") };
```

## Methods on Generic Structs

```rust
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

// Methods restricted to specific type parameters:
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

The `impl<T> Point<T>` block defines methods for **all** `Point<T>`.
The `impl Point<f64>` block defines methods only for `Point<f64>`.

## Generic Enums

```rust
enum Either<L, R> {
    Left(L),
    Right(R),
}

let x: Either<i32, &str> = Either::Left(42);
let y: Either<i32, &str> = Either::Right("hi");
```

`Option<T>` and `Result<T, E>` from the standard library are generic enums.

## A Generic Stack

```rust
struct Stack<T> { items: Vec<T> }

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
}

let mut s: Stack<i32> = Stack::new();
s.push(1); s.push(2);
let str_stack: Stack<&str> = Stack::new();   // same code, different type
```

## `where` Clauses

When bounds get long, move them into a `where` clause for readability:

```rust
fn sum_iter<T, I>(iter: I) -> T
where
    T: Add<Output = T> + Default,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().fold(T::default(), |acc, x| acc + x)
}
```

## Const Generics — Array Size as a Parameter

```rust
fn sum_array<T: Default + Add<Output = T> + Copy, const N: usize>(arr: [T; N]) -> T {
    let mut total = T::default();
    for x in arr { total = total + x; }
    total
}

sum_array([1, 2, 3]);                // N = 3
sum_array([10, 20, 30, 40, 50]);     // N = 5
```

`const N: usize` is a **compile-time constant** parameter — the array size
becomes part of the type, but the function works for any size.

## Generics vs Trait Objects

| | Generics `<T: Trait>` | Trait objects `dyn Trait` |
|---|------------------------|---------------------------|
| Dispatch | static (compile time) | dynamic (runtime, vtable) |
| Code generation | one copy per type | shared code |
| Speed | faster | slight overhead |
| Heterogeneous collections | no | yes |

## Monomorphization — How Generics Work

When you write:

```rust
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T { ... }

largest(&[1, 2, 3]);
largest(&[1.0, 2.0]);
```

The compiler generates two separate functions:

```rust
fn largest_i32(slice: &[i32]) -> i32 { ... }
fn largest_f64(slice: &[f64]) -> f64 { ... }
```

This is **zero-cost** — there's no runtime lookup, just like writing the
code by hand for each type. The trade-off is a slightly larger binary size.

## Generics + Lifetimes

You can combine type parameters, lifetime parameters, and trait bounds:

```rust
fn longest_in<'a, T: PartialOrd>(slice: &'a [T]) -> Option<&'a T> {
    slice.iter().max()
}
```

* `'a` is a lifetime parameter (output reference lives as long as input).
* `T` is a type parameter.
* `T: PartialOrd` is the trait bound.

## Common Standard Library Generics

| Type | Description |
|------|-------------|
| `Vec<T>` | growable list |
| `HashMap<K, V>` | key-value map |
| `Option<T>` | maybe a value |
| `Result<T, E>` | success or error |
| `Box<T>` | heap allocation |
| `Rc<T>` / `Arc<T>` | shared ownership |
| `&[T]` | slice (borrowed view) |

## Naming Conventions

* `T` for "type" (most common).
* `E` for "error" (in `Result<T, E>`).
* `K`, `V` for "key" and "value".
* `A`, `B` for paired types.
* Single uppercase letters are conventional; longer names are allowed.

## When to Use Generics

* You're writing the **same algorithm** for multiple types.
* You want a **single source of truth** — change the algorithm once.
* You need **maximum performance** (vs trait objects).

## When NOT to Use Generics

* You only have 2-3 types — duplicating may be simpler.
* You need **heterogeneous collections** (use trait objects instead).
* The type system gets too complex (consider redesigning).

**Next:** [Error Handling](../37_error_handling) — `Result`, `?`, custom errors.
