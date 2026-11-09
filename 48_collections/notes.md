# Collections (in depth)

Beyond `Vec` and `HashMap`, Rust's standard library provides several more
collection types. Each has different performance and ordering guarantees.

## The Standard Collections

| Collection | Lookup | Ordered? | Use when... |
|------------|--------|----------|-------------|
| `Vec<T>` | O(1) by index | insertion | default list; need indexing |
| `VecDeque<T>` | O(1) at ends | insertion | push/pop both ends (deque) |
| `LinkedList<T>` | O(n) | insertion | almost never — prefer Vec |
| `HashMap<K, V>` | O(1) avg | no | key-value lookup, no order needed |
| `BTreeMap<K, V>` | O(log n) | sorted by key | sorted keys / range queries |
| `HashSet<T>` | O(1) avg | no | unique values, fast membership |
| `BTreeSet<T>` | O(log n) | sorted | unique values, sorted iteration |
| `BinaryHeap<T>` | O(log n) | heap order | priority queue |

## `BTreeMap<K, V>` — Sorted Map

A balanced tree. Iteration is **always in sorted key order**.

```rust
use std::collections::BTreeMap;

let mut m: BTreeMap<&str, i32> = BTreeMap::new();
m.insert("zebra", 1);
m.insert("apple", 2);
m.insert("mango", 3);

for (k, v) in &m {
    println!("{} -> {}", k);  // apple, mango, zebra (sorted)
}
```

### Range Queries (BTreeMap-specific)

```rust
let middle: BTreeMap<&str, &i32> = m.range("banana".."mango").collect();
```

### When to Use BTreeMap vs HashMap

| | `HashMap` | `BTreeMap` |
|---|-----------|------------|
| Lookup | O(1) average | O(log n) |
| Iteration order | undefined | sorted |
| Key requirements | `Eq + Hash` | `Ord` |
| Range queries | no | yes |
| Memory | one allocation | tree of nodes |

Use `BTreeMap` when you need sorted iteration or range queries.

## `HashSet<T>` — Unordered Set

A `HashMap` without values — just unique keys.

```rust
use std::collections::HashSet;

let mut fruits: HashSet<&str> = HashSet::new();
fruits.insert("apple");
fruits.insert("banana");
fruits.insert("apple");          // ignored — already present

println!("{}", fruits.contains(&"banana"));   // true
println!("{}", fruits.len());                  // 2
```

### Set Operations

```rust
let a: HashSet<i32> = [1, 2, 3].iter().copied().collect();
let b: HashSet<i32> = [3, 4, 5].iter().copied().collect();

a.intersection(&b);          // {3}            — in both
a.union(&b);                 // {1, 2, 3, 4, 5} — in either
a.difference(&b);            // {1, 2}         — in a, not in b
a.symmetric_difference(&b);  // {1, 2, 4, 5}   — in exactly one
```

Each returns an iterator — collect it into a `Vec` or `HashSet` to use.

## `BTreeSet<T>` — Sorted Set

Like `HashSet` but iteration is in sorted order. Key must implement `Ord`.

```rust
use std::collections::BTreeSet;

let mut s: BTreeSet<i32> = BTreeSet::new();
s.insert(5);
s.insert(1);
s.insert(3);

for x in &s {
    println!("{}", x);   // 1, 3, 5 (sorted)
}
```

## `VecDeque<T>` — Double-Ended Queue

A growable ring buffer. O(1) `push_front`, `pop_front`, `push_back`,
`pop_back`. Use when you need to add/remove from **both ends** efficiently
(`Vec` is O(n) at the front).

```rust
use std::collections::VecDeque;

let mut d: VecDeque<i32> = VecDeque::new();
d.push_back(2);
d.push_back(3);
d.push_front(1);
d.push_front(0);
// d = [0, 1, 2, 3]

d.pop_front();    // Some(0)
d.pop_back();     // Some(3)
```

### Use Cases

* **Queue** (FIFO): `push_back` + `pop_front`
* **Stack** (LIFO): `push_back` + `pop_back`
* **Deque** (both ends): all four operations
* **Sliding window** algorithms

## `LinkedList<T>` — Rarely the Right Choice

A doubly linked list. **In Rust, almost never what you want.**

```rust
use std::collections::LinkedList;

let mut list: LinkedList<i32> = LinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_front(0);
```

### Why Not LinkedList?

* Poor cache locality (each node is a separate allocation).
* No O(1) random access.
* Iteration is slower than `Vec`.
* In Rust, the borrow checker makes many "natural" linked-list operations
  painful (you need `Rc<RefCell<Node>>` or `unsafe`).

### Prefer

* `Vec` / `VecDeque` for ordered storage.
* `HashMap` / `HashSet` for lookup.
* `Rc<RefCell<T>>` or `Box<Node>` for graph-like structures.

The `LinkedList` API exists mainly for completeness.

## `BinaryHeap<T>` — Priority Queue

A max-heap by default. O(1) `peek`, O(log n) `push` / `pop`. `pop` always
returns the **largest** element.

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(1);
heap.push(10);
heap.push(3);

while let Some(n) = heap.pop() {
    print!("{} ", n);   // 10 5 3 1 (descending)
}
```

### Min-Heap with `Reverse<T>`

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(5));
min_heap.push(Reverse(1));
min_heap.push(Reverse(10));

while let Some(Reverse(n)) = min_heap.pop() {
    print!("{} ", n);   // 1 5 10 (ascending)
}
```

### Custom Ordering

Implement `Ord` on your type, or wrap in `Reverse<T>` to invert.

```rust
#[derive(Eq, PartialEq)]
struct Task { priority: i32, name: String }

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first (reverse natural order):
        other.priority.cmp(&self.priority)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

## Default Choices

| You need | Default |
|----------|---------|
| A list | `Vec<T>` |
| Push/pop both ends | `VecDeque<T>` |
| Key → value map | `HashMap<K, V>` |
| Sorted map | `BTreeMap<K, V>` |
| Set of unique values | `HashSet<T>` |
| Sorted set | `BTreeSet<T>` |
| Priority queue | `BinaryHeap<T>` |
| Linked list | (reconsider — usually `Vec` is better) |

## Memory and Performance Notes

* `Vec` and `VecDeque` store elements contiguously — excellent cache
  locality.
* `HashMap` and `HashSet` use Robin Hood hashing by default (resistant to
  HashDoS attacks).
* `BTreeMap` and `BTreeSet` store nodes in a tree — more memory per
  element but sorted iteration.
* `LinkedList` allocates each node separately — worst cache locality.
* `BinaryHeap` is stored in a `Vec` (array representation of a tree) —
  cache-friendly.

## Common Patterns

### Counter

```rust
use std::collections::HashMap;

let mut counts: HashMap<char, i32> = HashMap::new();
for c in "hello world".chars() {
    *counts.entry(c).or_insert(0) += 1;
}
```

### Group By

```rust
use std::collections::HashMap;

let mut by_first_letter: HashMap<char, Vec<&str>> = HashMap::new();
for word in ["apple", "ant", "banana", "cherry"] {
    by_first_letter
        .entry(word.chars().next().unwrap())
        .or_insert_with(Vec::new)
        .push(word);
}
```

### LRU Cache (with `HashMap` + `VecDeque`)

```rust
use std::collections::{HashMap, VecDeque};

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        LruCache { capacity, map: HashMap::new(), order: VecDeque::new() }
    }
    fn get(&mut self, k: &K) -> Option<&V> {
        if self.map.contains_key(k) {
            // move to back (most recently used)
            self.order.retain(|x| x != k);
            self.order.push_back(k.clone());
            self.map.get(k)
        } else {
            None
        }
    }
    fn put(&mut self, k: K, v: V) {
        if self.map.len() >= self.capacity && !self.map.contains_key(&k) {
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }
        self.map.insert(k.clone(), v);
        self.order.push_back(k);
    }
}
```

### Merge K Sorted Lists (with `BinaryHeap`)

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
// push first element of each list...
// then pop, push next from same list, repeat
```

## When to Use Each

| Situation | Use |
|-----------|-----|
| Default ordered list | `Vec<T>` |
| Need fast front operations | `VecDeque<T>` |
| Need key → value lookup | `HashMap<K, V>` |
| Need sorted keys / range queries | `BTreeMap<K, V>` |
| Need unique values | `HashSet<T>` |
| Need sorted unique values | `BTreeSet<T>` |
| Need priority queue | `BinaryHeap<T>` |
| Need graph adjacency list | `HashMap<K, Vec<V>>` |

**Next:** [Type Conversions](../31_type_conversions) — `From`, `Into`, `AsRef`, `TryFrom`, `Deref`.
