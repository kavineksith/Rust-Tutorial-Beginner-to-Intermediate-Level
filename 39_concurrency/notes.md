# Rust Concurrency

Rust's slogan is **"fearless concurrency"** — the ownership system and the
`Send`/`Sync` traits let the compiler **prove** your concurrent code is
data-race free, with no runtime cost.

## Spawning Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from a thread");
    42
});

let result = handle.join().unwrap();   // blocks until thread finishes
```

* `thread::spawn` takes a closure (FnOnce).
* `.join()` blocks until the thread finishes and returns its result.

## `move` — Capturing Data Into a Thread

Closures borrow by default, but threads may outlive the creating scope.
Use `move` to take ownership:

```rust
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", data);          // data moved INTO the thread
});
handle.join().unwrap();
// println!("{:?}", data);          // ERROR: data was moved
```

## `Send` and `Sync`

Two auto-traits the compiler uses to enforce thread safety:

| Trait | Meaning |
|-------|---------|
| `Send` | type can be transferred **across** thread boundaries |
| `Sync` | `&T` can be **shared** across threads |

Most types are `Send + Sync`. Notable exceptions:

* `Rc<T>` — **not** `Send` (non-atomic counter).
* `RefCell<T>` — **not** `Sync` (non-atomic borrow flags).
* `MutexGuard` — **not** `Send` (must release on the same thread).

The compiler checks these at `thread::spawn` time. You cannot accidentally
share an `Rc` across threads — it just won't compile.

## Channels — Message Passing

`std::sync::mpsc` = "**m**ultiple **p**roducer, **s**ingle **c**onsumer".

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<String>();

let tx2 = tx.clone();   // multiple producers — clone the sender
thread::spawn(move || {
    tx2.send("hello".to_string()).unwrap();
});

drop(tx);    // drop the original so the channel closes when tx2 is gone
for msg in rx {
    println!("{}", msg);
}
```

* `tx.send(v)` returns `Result` (recv may have been dropped).
* `rx` is an iterator — `for msg in rx` ends when all senders are dropped.
* Cloning `tx` gives multiple producers; `rx` is single-consumer.

### Bounded Channels

`mpsc::sync_channel(n)` creates a **bounded** channel — `send` blocks when
the buffer is full. Useful for backpressure.

```rust
let (tx, rx) = mpsc::sync_channel::<i32>(4);   // buffer size 4
```

## Shared State with `Arc<Mutex<T>>`

When threads need to **mutate** shared data:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..4 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            let mut g = counter.lock().unwrap();
            *g += 1;
        }
    }));
}
for h in handles { h.join().unwrap(); }

println!("{}", *counter.lock().unwrap());   // 4000
```

* `Arc<T>` — atomic reference counting (shared ownership across threads).
* `Mutex<T>` — exclusive access (one thread at a time).
* `.lock()` blocks until exclusive access is available; returns a `Result`
  (see "poisoning" below).

### Why `Arc<Mutex<T>>` and not just `Mutex<T>`?

`Mutex<T>` gives exclusive access but has a single owner. To **share**
the mutex across threads, you need `Arc`. The combination
`Arc<Mutex<T>>` is the canonical pattern for multi-thread shared mutable
state.

## `RwLock<T>` — Many Readers OR One Writer

`Mutex` is exclusive for everyone. `RwLock` allows multiple concurrent
readers, but only one writer (and no readers while writing):

```rust
use std::sync::{Arc, RwLock};

let data = Arc::new(RwLock::new(vec![1, 2, 3]));

// Multiple readers can hold read locks simultaneously:
let r1 = data.read().unwrap();
let r2 = data.read().unwrap();   // OK — both readers

// A writer blocks until all readers release:
let mut w = data.write().unwrap();   // exclusive
w.push(4);
```

Use `RwLock` when **reads vastly outnumber writes**. Otherwise `Mutex` is
simpler and often faster.

| | `Mutex<T>` | `RwLock<T>` |
|---|-----------|--------------|
| Readers | exclusive (one at a time) | shared (many at once) |
| Writers | exclusive | exclusive |
| Best when | mixed read/write | mostly read |
| API | `.lock()` | `.read()`, `.write()` |

## Poisoned Mutexes

If a thread **panics** while holding a `Mutex`, the mutex becomes
**poisoned** — the data may be in an inconsistent state. Subsequent
`.lock()` calls return `Err(PoisonError)`:

```rust
let m = Mutex::new(0);
{
    let _g = m.lock().unwrap();
    panic!("oops");        // mutex is now poisoned
}
// let g = m.lock();        // returns Err(PoisonError)
```

This is a **safety feature** — you can opt out via `.lock().unwrap_or_else(|e| e.into_inner())`
if you want to access the data anyway.

## Scoped Threads (Rust 1.63+) — Borrow Without `Arc`

`thread::scope` lets child threads **borrow** from the parent scope
without needing `move` or `Arc`. The scope guarantees all child threads
are joined before it returns:

```rust
let mut arr = [0u32; 4];
std::thread::scope(|s| {
    for chunk in arr.chunks_mut(2) {
        s.spawn(move || {
            for x in chunk.iter_mut() { *x = 42; }
        });
    }
});   // <- all threads joined here, before `arr` is used again
println!("{:?}", arr);   // [42, 42, 42, 42]
```

This is much cleaner than `Arc<Mutex<...>>` when you just need to borrow
data temporarily.

## Common Patterns

### Worker Pool

```rust
let (tx, rx) = mpsc::channel();
let rx = Arc::new(Mutex::new(rx));

let mut workers = vec![];
for _ in 0..4 {
    let rx = Arc::clone(&rx);
    workers.push(thread::spawn(move || {
        while let Ok(job) = rx.lock().unwrap().recv() {
            // process job
        }
    }));
}
```

### Parallel Map

```rust
let data = vec![1, 2, 3, 4];
let results: Vec<i32> = data.into_iter().map(|x| {
    thread::spawn(move || x * 2).join().unwrap()
}).collect();
```

### Fan-Out / Fan-In

```rust
let (tx, rx) = mpsc::channel();
for i in 0..4 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(process(i)).unwrap();
    });
}
drop(tx);
let all: Vec<_> = rx.iter().collect();
```

## `Send` + `Sync` Reference

| Type | Send | Sync | Notes |
|------|------|------|-------|
| `i32`, `f64`, `bool`, ... | yes | yes | primitive types |
| `String`, `Vec<T>` | yes | yes | owned heap types |
| `Box<T>` | if T: Send | if T: Sync | heap single-owner |
| `Rc<T>` | **no** | **no** | non-atomic counter |
| `Arc<T>` | if T: Send + Sync | if T: Send + Sync | atomic counter |
| `RefCell<T>` | **no** | **no** | runtime borrow check |
| `Mutex<T>` | if T: Send | yes | thread-safe lock |
| `RwLock<T>` | if T: Send + Sync | yes | reader/writer lock |
| `mpsc::Sender<T>` | if T: Send | no | single-consumer |

## Common Pitfalls

### Deadlock

Two threads each hold a lock the other wants:

```rust
// Thread 1: lock A, then B
// Thread 2: lock B, then A
// Both wait forever.
```

Avoid by always acquiring locks in the **same order**.

### Forgetting to Drop a Guard

```rust
let g = mutex.lock().unwrap();
// ... do work that doesn't need g ...
// Other threads can't make progress because g still holds the lock.
```

Drop the guard explicitly: `drop(g);` or scope it tightly.

### Sharing `Rc` Across Threads

```rust
let rc = Rc::new(0);
thread::spawn(move || {           // ERROR: Rc is not Send
    println!("{}", rc);
});
```

Fix: use `Arc` instead.

### Calling `.unwrap()` on a Poisoned Lock

```rust
let g = mutex.lock().unwrap();   // panics if mutex is poisoned
```

Decide intentionally whether to propagate the poison or recover.

## `std::thread` vs Async

| | Threads | Async |
|---|---------|-------|
| Per-task overhead | ~2 MB stack | ~few KB state machine |
| Best for | CPU-bound, few tasks | I/O-bound, many tasks (1000s) |
| Preemption | OS preempts | cooperative (must `.await`) |
| Library | `std::thread` | `tokio` / `async-std` |

Use **threads** for CPU-bound parallelism. Use **async** for many
concurrent I/O operations (chapter 34).

## Summary

| Need | Use |
|------|-----|
| Spawn a thread | `thread::spawn(move \|\| { ... })` |
| Send data to a thread | `mpsc::channel` |
| Shared mutable state | `Arc<Mutex<T>>` |
| Mostly-read shared state | `Arc<RwLock<T>>` |
| Borrow across threads (temporary) | `thread::scope` |
| Backpressure | `mpsc::sync_channel(n)` |
| Worker pool | channels + `Arc<Mutex<rx>>` |

**Next:** [Testing](../38_testing) — `#[test]`, `cargo test`, doctests.
