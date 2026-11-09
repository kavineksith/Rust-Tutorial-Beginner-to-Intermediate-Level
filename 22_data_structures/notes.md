# Rust Data Structures

Rust provides several built-in ways to group multiple values together.
This chapter is an overview — each type gets its own dedicated chapter.

## The Built-in Collections

| Type | Size | Types | Storage | Chapter |
|------|------|-------|---------|---------|
| Array `[T; N]` | fixed | same | stack | 22 |
| Vector `Vec<T>` | growable | same | heap | 23 |
| Tuple `(T1, T2, ...)` | fixed | mixed | stack | 24 |
| HashMap `HashMap<K, V>` | growable | mixed (key/value) | heap | 25 |
| Struct | fixed | mixed (named) | depends | 26 |
| Enum | one of several | mixed (per variant) | depends | 27 |

## Quick Comparison

### Array — `[T; N]`

Fixed size, same type, lives on the stack:

```rust
let arr: [i32; 4] = [10, 20, 30, 40];
let zeros = [0; 100];   // 100 zeros
```

Best when the size is known at compile time and small.

### Vector — `Vec<T>`

Growable, heap-allocated, same type:

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
```

The most common list type in Rust.

### Tuple — `(T1, T2, ...)`

Fixed size, mixed types:

```rust
let person: (&str, i32, f64) = ("Ada", 36, 1.68);
let (name, age, height) = person;
```

### HashMap — `HashMap<K, V>`

Key-value storage:

```rust
use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("apple", 5);
m.insert("banana", 3);
```

### Struct

Custom record type with named fields:

```rust
struct User {
    name: String,
    age: u32,
}
let user = User { name: String::from("Ada"), age: 36 };
```

### Enum

A value that can be **one of several variants**:

```rust
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

let s = Shape::Circle(2.0);
```

Rust's enums are extremely powerful — they can carry data and are
pattern-matched exhaustively. `Option<T>` and `Result<T, E>` are enums.

## Choosing the Right Structure

| You need... | Use |
|-------------|-----|
| A fixed-size list of the same type | Array |
| A growable list | Vec |
| A few related values of different types | Tuple |
| A small record with named fields | Struct |
| A key -> value mapping | HashMap |
| A value that can be one of several variants | Enum |
| A sorted key -> value mapping | `BTreeMap` |
| A unique set of values | `HashSet` |
| A double-ended queue | `VecDeque` |

## Standard Library Collections

Beyond the basics, the standard library provides:

* `VecDeque<T>` — double-ended queue
* `LinkedList<T>` — doubly linked list (rarely needed)
* `HashSet<T>` / `BTreeSet<T>` — sets
* `HashMap<K, V>` / `BTreeMap<K, V>` — maps
* `BinaryHeap<T>` — priority queue

Most of the time, you will reach for `Vec` and `HashMap`.

## Where to Go Next

* [22_arrays](22_arrays) — fixed-size arrays
* [23_vectors](23_vectors) — growable vectors
* [24_tuples](24_tuples) — mixed-type tuples
* [25_hashmap](25_hashmap) — key-value maps
* [26_structs](26_structs) — custom records
* [27_enums](27_enums) — sum types and Option/Result
