# Rust Smart Pointers

Smart pointers **own** the data they point to (unlike plain references,
which only borrow). They add capabilities: heap allocation, shared
ownership, interior mutability, reference counting.

## The Family

| Type | Ownership | Thread-safe? | Borrow check |
|------|-----------|--------------|--------------|
| `Box<T>` | single | yes | compile time |
| `Rc<T>` | shared | no (single thread) | compile time |
| `Weak<T>` | non-owning | no | compile time |
| `RefCell<T>` | single | no | **runtime** |
| `Arc<T>` | shared | yes | compile time |
| `Mutex<T>` / `RwLock<T>` | single | yes | runtime (via lock) |

## 1. `Box<T>` — Heap Allocation, Single Owner

`Box` moves a value from the stack to the heap. It has a single owner —
no sharing.

```rust
let boxed = Box::new(42);
println!("{}", boxed);          // auto-derefs for method calls / Display
let unboxed = *boxed;            // explicit deref
```

### Common Uses

* **Large values** you don't want to copy around the stack.
* **Recursive types** (a struct that contains itself — needs a known size).
* **Trait objects** (`Box<dyn Trait>`).

### Recursive Type Example

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
```

Without `Box`, `Cons` would have infinite size. `Box<List>` is just a
pointer — known size.

### Trait Objects

```rust
trait Shape { fn area(&self) -> f64; }
struct Circle { radius: f64 }
impl Shape for Circle { fn area(&self) -> f64 { /* ... */ 0.0 } }

let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 2.0 })];
for s in &shapes { println!("{}", s.area()); }
```

`Box<dyn Trait>` enables heterogeneous collections (multiple types in one
`Vec`) via dynamic dispatch.

## 2. `Rc<T>` — Reference-Counted, Single Thread

`Rc` (reference count) enables **shared ownership** on a single thread.
The data is dropped when the **last** `Rc` is dropped.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);    // bumps refcount, does NOT copy data
let c = Rc::clone(&a);

println!("{}", Rc::strong_count(&a));   // 3
println!("{} {} {}", a, b, c);          // all print "shared"
```

* `Rc::clone(&x)` is cheap — it only increments a counter.
* **NOT thread-safe** (uses a non-atomic counter).
* Use only within a single thread.

### Why Not Always `Rc`?

Because it has runtime overhead (counter maintenance) and prevents the
data from being sent across threads. Use `Rc` only when you actually need
shared ownership.

## 3. `Weak<T>` — Non-Owning Reference

Two `Rc`s pointing at each other form a **cycle** — neither will ever be
dropped, causing a memory leak.

`Weak<T>` is a non-owning reference that does **not** prevent dropping:

```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(42);
let weak: Weak<i32> = Rc::downgrade(&strong);

if let Some(value) = weak.upgrade() {
    println!("still alive: {}", value);
}

drop(strong);
println!("{:?}", weak.upgrade());   // None
```

Use `Weak` for child→parent links in graphs/trees (parent owns children
via `Rc`, children point back via `Weak`).

## 4. `RefCell<T>` — Interior Mutability

Normally, if you have `&x`, you cannot mutate `x`. `RefCell` relaxes this
by moving the borrow check to **runtime**.

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2, 3]);
{
    let mut borrowed = cell.borrow_mut();   // runtime-checked mutable borrow
    borrowed.push(4);
}
println!("{:?}", cell.borrow());   // [1, 2, 3, 4]
```

### Runtime Borrow Rules

`RefCell` enforces the same rules as the compile-time borrow checker, but
at runtime:

* Many `borrow()` OR one `borrow_mut()` — not both.
* Violations **panic** instead of failing to compile.

```rust
let cell = RefCell::new(0);
let b1 = cell.borrow_mut();
let b2 = cell.borrow_mut();   // PANIC: already mutably borrowed
```

### Use Case: Mutating Through `Rc`

`Rc<T>` is immutable — you cannot mutate the data inside. To get shared
mutable data on one thread, combine:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));

let clone1 = Rc::clone(&shared);
let clone2 = Rc::clone(&shared);
clone1.borrow_mut().push(1);
clone2.borrow_mut().push(2);
println!("{:?}", shared.borrow());   // [1, 2]
```

## 5. `Arc<T>` — Atomic Reference Counting

`Arc` is `Rc` for **multi-threaded** code. It uses atomic operations for
the counter — slightly slower, but safe to share across threads.

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(42);
let mut handles = vec![];

for _ in 0..4 {
    let data = Arc::clone(&data);    // atomic increment
    handles.push(thread::spawn(move || {
        println!("{}", data);
    }));
}
for h in handles { h.join().unwrap(); }
```

`Rc` cannot cross thread boundaries (the compiler will refuse). Use `Arc`.

## 6. `Mutex<T>` and `RwLock<T>` — Thread-Safe Interior Mutability

`RefCell` is single-threaded. For multi-threaded shared mutable state,
use `Mutex` or `RwLock` (usually wrapped in `Arc`).

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }
    }));
}
for h in handles { h.join().unwrap(); }

println!("{}", *counter.lock().unwrap());   // 10000
```

### `Mutex` vs `RwLock`

| | `Mutex<T>` | `RwLock<T>` |
|---|-----------|--------------|
| Access | exclusive (one writer OR one reader) | many readers OR one writer |
| Faster when | mixed read/write | mostly read |
| API | `.lock()` | `.read()`, `.write()` |

## Common Combinations

| Combination | Use case |
|-------------|----------|
| `Box<T>` | heap allocation, recursive types, trait objects |
| `Rc<RefCell<T>>` | single-thread shared mutable state |
| `Arc<Mutex<T>>` | multi-thread shared mutable state |
| `Arc<RwLock<T>>` | multi-thread, mostly-read shared state |
| `Rc<Weak<T>>` | graph back-references (no cycle) |

## Smart Pointers and Traits

All smart pointers implement:

* `Deref` — lets you call methods on `T` directly through the pointer.
* `Drop` — automatically runs cleanup when the pointer goes out of scope.

```rust
let s = Box::new(String::from("hi"));
println!("{}", s.len());    // works because Box derefs to String
```

## Deref Coercion

`Box<String>` can be passed where `&String` or `&str` is expected:

```rust
fn takes_str(s: &str) { println!("{}", s); }

let boxed = Box::new(String::from("hi"));
takes_str(&boxed);    // Box<String> -> &String -> &str via Deref
```

The compiler chains `Deref` calls automatically.

## Choosing the Right Pointer

```
Do you need shared ownership?
├── No
│   ├── Heap allocation or trait objects? → Box<T>
│   └── Need interior mutability? → RefCell<T>
└── Yes
    ├── Single thread?
    │   ├── Owning ref? → Rc<T>
    │   ├── Back-reference? → Weak<T>
    │   └── Need mutability? → Rc<RefCell<T>>
    └── Multi thread?
        ├── Owning ref? → Arc<T>
        ├── Back-reference? → Arc<Weak<T>>
        └── Need mutability? → Arc<Mutex<T>> or Arc<RwLock<T>>
```

## When to Use Each

| Want | Use |
|------|-----|
| Heap allocation | `Box<T>` |
| Recursive type | `Box<T>` |
| Heterogeneous collection | `Vec<Box<dyn Trait>>` |
| Single-thread shared ownership | `Rc<T>` |
| Single-thread shared mutable | `Rc<RefCell<T>>` |
| Multi-thread shared ownership | `Arc<T>` |
| Multi-thread shared mutable | `Arc<Mutex<T>>` |
| Break reference cycle | `Weak<T>` |
| Tree back-pointer | `Rc<Weak<T>>` |

**Next:** [Concurrency](../39_concurrency) — threads, `Mutex`, `RwLock`, channels.
