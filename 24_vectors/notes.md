# Rust Vectors

`Vec<T>` is a **growable**, **heap-allocated** list of values of the same
type. It is Rust's most-used collection.

## Creating a `Vec`

```rust
let v1: Vec<i32> = Vec::new();           // empty, type annotated
let v2 = vec![1, 2, 3, 4, 5];            // macro, type inferred
let v3: Vec<&str> = vec!["a", "b", "c"];
```

## Adding Elements

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
v.push(30);

v.insert(1, 15);    // insert at index, shifts later elements
// v = [10, 15, 20, 30]

let r = v.remove(0);  // remove at index, shifts later elements
// v = [15, 20, 30], r = 10
```

## Accessing Elements

```rust
let third = v[2];        // panics if out of bounds
let first = v.get(0);    // Option<&T> — safe, returns None
let oob   = v.get(100);  // None — never panics
```

Use `.get()` when the index might be out of bounds.

## Length, Capacity, Empty

```rust
v.len();          // number of elements
v.capacity();     // allocated capacity (>= len)
v.is_empty();
```

### Pre-allocate for Performance

```rust
let mut v: Vec<i32> = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);      // no reallocation
}
```

## Popping

```rust
let mut stack = vec![1, 2, 3];
let last = stack.pop();   // Some(3) — removes and returns the last element
```

## Iterating

```rust
for n in &v {                  // borrow — keeps v usable
    println!("{}", n);
}

for n in v.iter_mut() {        // mutable borrow
    *n *= 10;
}

for s in owned {               // consume — moves each element out
    println!("{}", s);
}
```

| Method | Yields | Modifies? | Consumes? |
|--------|--------|-----------|-----------|
| `for x in &v` | `&T` | no | no |
| `for x in &mut v` | `&mut T` | yes | no |
| `for x in v` | `T` | — | yes |

## Slicing

```rust
let slice: &[i32] = &v[1..3];   // [20, 30]
```

Functions that take `&[T]` accept both `Vec` and array slices.

## Common Operations

```rust
v.iter().sum::<i32>();          // sum all
v.iter().max()                  // Option<&T>
v.iter().min()                  // Option<&T>
v.contains(&3);                 // bool
v.retain(|&x| x % 2 == 0);     // keep only matching
v.extend([6, 8, 10]);          // append another iterable
```

## Sorting and Reversing

```rust
let mut v = vec![3, 1, 4, 1, 5];
v.sort();                                  // [1, 1, 3, 4, 5]
v.sort_by(|a, b| b.cmp(a));                // descending
v.reverse();
```

## Building a New Vec with `collect`

```rust
let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
// [1, 4, 9, 16, 25]

let evens: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
// [2, 4, 6, 8, 10]
```

## Vec of Mixed Types?

Rust `Vec`s must have a single element type. To store mixed types, use an
enum:

```rust
enum Value {
    Int(i32),
    Text(String),
}

let v: Vec<Value> = vec![
    Value::Int(42),
    Value::Text(String::from("hi")),
];
```

Or use `Vec<Box<dyn Trait>>` for trait objects (dynamic dispatch).

## Vec vs Array

| | `Vec<T>` | `[T; N]` |
|---|----------|----------|
| Size | dynamic | fixed |
| Storage | heap | stack |
| Methods | rich (push, pop, sort...) | basic |
| Use case | most code | small fixed buffers |

## Performance Notes

* `push` is **amortized O(1)** — sometimes triggers reallocation (capacity
  doubling).
* `insert` / `remove` are **O(n)** — they shift elements.
* Random access `v[i]` is **O(1)**.
* `Vec` stores elements contiguously — excellent cache locality.

**Next:** [Tuples](../25_tuples) — fixed-size, mixed types.
