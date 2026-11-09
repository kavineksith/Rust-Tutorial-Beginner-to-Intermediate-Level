# Rust Closures & Iterators (in depth)

Closures and iterators are Rust's most expressive tools for data
processing. Together they enable a functional style that compiles down to
the same machine code as hand-written loops — **zero-cost abstractions**.

# Part A — Closures

A **closure** is an anonymous function that can **capture** variables from
its surrounding scope.

## Syntax

```rust
let add_one = |x: i32| x + 1;          // single expression
let double  = |x: i32| { x * 2 };       // block body
let say_hi  = || println!("hi");        // no parameters
```

Closures look like functions but use `|args| body` syntax. Type inference
is more aggressive than for `fn` — you often don't need to annotate the
argument types.

## Capturing the Environment

This is the key difference from `fn`. A closure can use variables from the
surrounding scope:

```rust
let multiplier = 3;
let multiply = |x| x * multiplier;     // captures `multiplier`
println!("{}", multiply(5));           // 15
```

## Three Capture Modes (the `Fn` traits)

The compiler picks the **least restrictive** mode that works:

| Trait | Captures | Can call? | Example |
|-------|----------|-----------|---------|
| `Fn` | by immutable reference | many times | `|| println!("{}", x)` |
| `FnMut` | by mutable reference | many times | `|| counter += 1` |
| `FnOnce` | by ownership (moves) | once | `|| drop(text)` |

```rust
// Fn — only reads
let name = String::from("Ada");
let greet = || println!("Hi {}", name);
greet();  greet();              // call many times
println!("{}", name);           // still usable

// FnMut — mutates
let mut counter = 0;
let mut inc = || { counter += 1; };
inc();  inc();  inc();
println!("{}", counter);        // 3

// FnOnce — consumes
let text = String::from("owned");
let consume = move || { drop(text); };
consume();
// consume();   // ERROR: closure can only be called once
```

## The `move` Keyword

`move` forces the closure to **take ownership** of captured variables.
Essential for threads and async tasks that outlive the current scope:

```rust
let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("{:?}", data);    // data moved INTO the thread
}).join().unwrap();
```

## Closures as Function Arguments

Use `impl Fn(...)` (or generic `<F: Fn(...) -> T>`):

```rust
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

apply(|x| x * x, 5);           // 25
```

## Returning Closures

Closures don't have a named type. Return them with `impl Fn` (if they
don't borrow from locals) or `Box<dyn Fn>`:

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n            // `move` captures n by value
}

let add10 = make_adder(10);
add10(5);    // 15
```

# Part B — Iterators

An **iterator** produces a sequence of values lazily. It implements:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Calling `.next()` returns `Some(item)` or `None` when exhausted.

## Iterators Are LAZY

Creating an iterator does nothing. You must consume it:

```rust
// This runs nothing:
let _lazy = [1, 2, 3].iter().map(|x| x * 2);

// This actually runs:
let doubled: Vec<i32> = [1, 2, 3].iter().map(|&x| x * 2).collect();
```

## Three Forms of Iteration

| Method | Yields | Modifies? | Consumes? |
|--------|--------|-----------|-----------|
| `v.iter()` | `&T` | no | no |
| `v.iter_mut()` | `&mut T` | yes | no |
| `v.into_iter()` | `T` | — | yes |

```rust
let v = vec![1, 2, 3];
for x in v.iter()     { /* &i32 */ }
for x in v.iter_mut() { /* &mut i32 — needs `mut v` */ }
let v2 = vec![4, 5, 6];
for x in v2.into_iter() { /* i32 — v2 is consumed */ }
```

## Iterator Adaptors (Lazy)

Adaptors return a **new iterator** — they don't consume the original.

| Adaptor | Effect |
|---------|--------|
| `.map(f)` | transform each element |
| `.filter(p)` | keep only elements matching predicate |
| `.take(n)` | stop after n elements |
| `.skip(n)` | drop first n elements |
| `.enumerate()` | pair each element with its index |
| `.zip(other)` | pair elements from two iterators |
| `.flat_map(f)` | map then flatten one level |
| `.inspect(f)` | peek without consuming (debugging) |
| `.rev()` | reverse (only on double-ended iterators) |
| `.step_by(n)` | take every nth element |

## Consuming Adaptors (Eager)

Consuming adaptors **drain** the iterator and produce a value.

| Adaptor | Returns |
|---------|---------|
| `.collect::<T>()` | any collection (`Vec`, `HashMap`, `String`, ...) |
| `.sum()` | sum of all elements |
| `.product()` | product of all elements |
| `.min()`, `.max()` | smallest / largest (`Option<T>`) |
| `.count()` | number of elements |
| `.fold(init, f)` | general reduce |
| `.any(p)`, `.all(p)` | boolean checks |
| `.find(p)` | first matching (`Option<T>`) |
| `.for_each(f)` | run a side effect per element |

## Chaining (Zero-Cost)

```rust
let result: i32 = (1..=10)
    .filter(|x| x % 2 == 0)    // keep evens
    .map(|x| x * x)            // square them
    .take(3)                   // first 3
    .sum();                    // sum them
// 4 + 16 + 36 = 56
```

This compiles down to the same machine code as a hand-written `for` loop.
**Zero-cost abstraction** in action.

## `fold` — The General Reduce

```rust
let sum: i32 = (1..=5).fold(0, |acc, x| acc + x);    // 15
let product: i32 = (1..=5).fold(1, |acc, x| acc * x); // 120
let concat: String = ["a", "b", "c"].iter().fold(String::new(), |acc, &s| acc + s);
```

## `collect` Into Different Types

`collect()` is smart — it can build many container types via the `FromIterator`
trait:

```rust
let v: Vec<i32>         = (1..=5).collect();
let s: HashSet<i32>     = vec![1, 1, 2, 3].into_iter().collect();
let m: HashMap<char, i32> = vec![('a', 1), ('b', 2)].into_iter().collect();
let text: String        = vec!['h', 'i'].into_iter().collect();
```

## Custom Iterators

Implement the `Iterator` trait:

```rust
struct Counter { count: u32, max: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

let v: Vec<u32> = Counter::new(5).collect();   // [1, 2, 3, 4, 5]
let sum: u32 = Counter::new(10).map(|x| x * x).sum();   // 385
```

You get **all** iterator methods for free (`map`, `filter`, `sum`, ...).

## Common Patterns

### Map-Filter-Collect

```rust
let names: Vec<String> = people
    .iter()
    .filter(|p| p.active)
    .map(|p| p.name.clone())
    .collect();
```

### Group By

```rust
use std::collections::HashMap;
let mut by_age: HashMap<u32, Vec<&str>> = HashMap::new();
for (age, name) in [(30, "Ada"), (30, "Bob"), (25, "Cy")] {
    by_age.entry(age).or_insert_with(Vec::new).push(name);
}
```

### Windowed Sums

```rust
let v = [1, 2, 3, 4, 5];
let window_sums: Vec<i32> = v.windows(3).map(|w| w.iter().sum()).collect();
// [6, 9, 12]
```

### Find First Match

```rust
let first_even = vec![1, 3, 5, 8, 9].into_iter().find(|x| x % 2 == 0);
// Some(8)
```

## Closures + Iterators = Functional Style

```rust
// Compute the average word length in a sentence:
let sentence = "the quick brown fox jumps over the lazy dog";
let avg = sentence
    .split_whitespace()
    .map(|w| w.len())
    .sum::<usize>() as f64
    / sentence.split_whitespace().count() as f64;
```

## When to Use Closures & Iterators

| Use them | Don't use them |
|----------|----------------|
| Data transformations | When the loop has complex state |
| Filtering / mapping / reducing | When performance profiling shows a hot loop |
| Pipeline-style code | When early-exit logic is non-trivial |
| Reading streams | When you need indexed access |

## Performance

* Iterators compile down to tight loops — same machine code as `for`.
* No heap allocation unless you `.collect()`.
* No virtual dispatch unless you use `Box<dyn Iterator>`.
* All checks are at compile time — zero runtime cost.

**Next:** [Smart Pointers](../35_smart_pointers) — `Box`, `Rc`, `Arc`, `RefCell`.
