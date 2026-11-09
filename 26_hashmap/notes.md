# Rust HashMap

`HashMap<K, V>` stores **key-value pairs** with **O(1) average** lookup,
insertion, and removal. It is Rust's standard dictionary type.

## Creating a HashMap

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("Alice"), 10);
scores.insert(String::from("Bob"), 20);
```

## From Two Vectors

```rust
let teams = vec![String::from("Red"), String::from("Blue")];
let initial = vec![10, 50];
let map: HashMap<_, _> = teams.into_iter().zip(initial.into_iter()).collect();
```

## Accessing Values

```rust
let alice = scores.get(&String::from("Alice"));   // Option<&V> -> Some(&10)
let charlie = scores.get(&String::from("Charlie")); // None
```

`.get()` returns an `Option<&V>` — safe lookup. Use it instead of indexing
(which would panic on a missing key).

## Iterating

```rust
for (name, score) in &scores {
    println!("{} -> {}", name, score);
}
```

Iteration order is **not defined** — it depends on the hasher. If you need
sorted keys, use `BTreeMap`.

## Inserting and Updating

### Overwrite (insert replaces the existing value)

```rust
scores.insert(String::from("Alice"), 15);   // was 10, now 15
```

### Insert Only If Absent

```rust
scores.entry(String::from("Alice")).or_insert(99);    // already present — no change
scores.entry(String::from("Charlie")).or_insert(99);  // not present — inserted
```

`entry()` returns an `Entry` enum; `or_insert()` adds the value only if the
key is missing.

### Update Based on Old Value

Counting words in a sentence:

```rust
let text = "hello world wonderful world";
let mut counts: HashMap<&str, i32> = HashMap::new();
for word in text.split_whitespace() {
    let count = counts.entry(word).or_insert(0);
    *count += 1;
}
// {"hello": 1, "world": 2, "wonderful": 1}
```

## Removing

```rust
let removed = scores.remove(&String::from("Charlie"));   // Option<V>
```

## Length and Contains

```rust
scores.len();
scores.is_empty();
scores.contains_key(&String::from("Alice"));
```

## Ownership

* **Copy types** (`i32`, `&str`, ...) are copied into the map.
* **Non-Copy types** (`String`, `Vec`, ...) are **moved** into the map.

```rust
let key = String::from("owned");
let val = vec![1, 2, 3];
let mut map: HashMap<String, Vec<i32>> = HashMap::new();
map.insert(key, val);
// println!("{}", key);   // ERROR: key was moved
// println!("{:?}", val); // ERROR: val was moved
```

If you want to keep using `key` after inserting, pass a reference instead
(`&key`), or insert a clone.

## Key Requirements

The key type `K` must implement `Eq` and `Hash`. Most standard types do.
For your own types, derive them:

```rust
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }

let mut map: HashMap<Point, &str> = HashMap::new();
map.insert(Point { x: 1, y: 2 }, "A");
```

Tuples also implement `Eq + Hash` (when their elements do), so they are
commonly used as composite keys:

```rust
let mut m: HashMap<(i32, i32), &str> = HashMap::new();
m.insert((0, 0), "origin");
```

## HashMap vs BTreeMap

| | `HashMap<K, V>` | `BTreeMap<K, V>` |
|---|------------------|---------------------|
| Lookup | O(1) average | O(log n) |
| Iteration order | undefined | sorted by key |
| Key requirements | `Eq + Hash` | `Ord` |

Use `BTreeMap` when you need **sorted** iteration.

## HashSet (related)

`HashSet<T>` is the set version — stores keys only, no values:

```rust
use std::collections::HashSet;
let mut s: HashSet<i32> = HashSet::new();
s.insert(1);
s.insert(2);
s.insert(1);   // ignored (already present)
s.contains(&1);   // true
```

## Common Patterns

### Counter

```rust
let mut counts: HashMap<char, i32> = HashMap::new();
for c in "hello world".chars() {
    *counts.entry(c).or_insert(0) += 1;
}
```

### Grouping

```rust
let mut by_first_letter: HashMap<char, Vec<&str>> = HashMap::new();
for word in ["apple", "ant", "banana", "cherry", "cat"] {
    by_first_letter
        .entry(word.chars().next().unwrap())
        .or_insert_with(Vec::new)
        .push(word);
}
```

**Next:** [Structs](../27_structs) — custom record types.
