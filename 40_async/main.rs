// ============================================================================
//  CHAPTER 34 — RUST ASYNC / AWAIT
//  Rust's concurrency model: `async fn` returns a Future; `.await` polls it
//  to completion. This file uses only the standard library — no tokio or
//  async-std required — so you can compile it with plain `rustc`.
//
//  IMPORTANT: async functions return Futures, and Futures do NOTHING until
//  something polls them. To actually run async code you need a RUNTIME
//  (executor). This file includes a tiny spin-loop executor built from
//  std primitives so it is self-contained.
// ============================================================================

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
//  1. A minimal single-threaded executor (built from std primitives)
// ============================================================================
//  This `block_on` polls a future repeatedly, sleeping between polls. A real
//  runtime like tokio is far more sophisticated (epoll, work stealing, etc.)
//  but this is enough to learn the concepts.
fn block_on<F: Future>(future: F) -> F::Output {
    // Pin the future so we can poll it safely. (Pin is covered below.)
    let mut future = Box::pin(future);

    // Build a "noop" waker — when the future says "wake me", we ignore it
    // and just keep polling. (Real executors schedule the wake-up.)
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(out) => return out,
            Poll::Pending    => thread::yield_now(),   // spin politely
        }
    }
}

// Build a Waker that does nothing — enough for our spin loop.
fn noop_waker() -> Waker {
    fn noop_clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &NOOP_VTABLE) }
    fn noop(_: *const ()) {}
    fn noop_wake_by_ref(_: *const ()) {}

    static NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(
        noop_clone,
        noop,
        noop_wake_by_ref,
        noop,
    );
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &NOOP_VTABLE)) }
}

// ============================================================================
//  2. async fn — defines a function that returns a Future
// ============================================================================
//  `async fn foo(args) -> T` is sugar for:
//      fn foo(args) -> impl Future<Output = T>
//
//  Calling an async fn does NOT run its body — it returns a state machine
//  (the Future). The body only progresses when you poll the future.
async fn hello() -> String {
    String::from("hello from async!")
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

// ============================================================================
//  3. .await — pause the async fn until the future is ready
// ============================================================================
//  Inside an async fn, `.await` polls a future. If it returns Pending, the
//  current async fn yields control back to the executor.
async fn greet(name: &str) -> String {
    let part1 = hello().await;            // await the hello future
    let part2 = format!(", {}", name);
    format!("{}{}", part1, part2)
}

// ============================================================================
//  4. Concurrent async tasks with `join` (sequential awaiting shown first)
// ============================================================================
//  Awaiting in sequence — total time = sum of task times:
async fn sequential() -> (String, String) {
    let a = sleep_then("A", 100).await;
    let b = sleep_then("B", 100).await;
    (a, b)
}

// A "fake I/O" async function: sleeps for `ms` then returns the label.
// We use std::thread::sleep inside a future — not how you'd do it in real
// code (you'd use tokio::time::sleep), but it works for demonstration.
async fn sleep_then(label: &str, ms: u64) -> String {
    let start = Instant::now();
    // We poll a future that completes after ms milliseconds.
    YieldFor { until: start + Duration::from_millis(ms) }.await;
    format!("[{} after {}ms]", label, ms)
}

// A primitive future that returns Ready once `until` has passed.
struct YieldFor { until: Instant }

impl Future for YieldFor {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.until {
            Poll::Ready(())
        } else {
            // Real async runtimes register a waker so they don't busy-spin.
            // Our simple executor just yields the thread.
            thread::yield_now();
            Poll::Pending
        }
    }
}

// ============================================================================
//  5. A simple "join" — run two futures concurrently
// ============================================================================
//  Polling both in the same async fn means they progress together.
async fn concurrent() -> (String, String) {
    // Create both futures first (they don't run yet).
    let f1 = sleep_then("A", 100);
    let f2 = sleep_then("B", 100);

    // Pin them so we can poll independently.
    let mut f1 = Box::pin(f1);
    let mut f2 = Box::pin(f2);

    // A naive join: poll both until each is ready.
    let mut a = None;
    let mut b = None;
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    while a.is_none() || b.is_none() {
        if a.is_none() {
            if let Poll::Ready(v) = f1.as_mut().poll(&mut cx) {
                a = Some(v);
            }
        }
        if b.is_none() {
            if let Poll::Ready(v) = f2.as_mut().poll(&mut cx) {
                b = Some(v);
            }
        }
        if a.is_none() || b.is_none() {
            thread::yield_now();
        }
    }
    (a.unwrap(), b.unwrap())
}

// ============================================================================
//  6. Real concurrency with OS threads (sync alternative)
// ============================================================================
//  For comparison: plain threads are Rust's other concurrency model.
//  They are heavier than async tasks but simpler.
fn threads_demo() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("threads_demo: counter = {}", counter.load(Ordering::Relaxed));
}

fn main() {
    println!("=== 1. block_on a single async fn ===");
    let msg = block_on(hello());
    println!("{}", msg);

    println!("\n=== 2. async add ===");
    let sum = block_on(add_async(3, 4));
    println!("3 + 4 = {}", sum);

    println!("\n=== 3. await inside an async fn ===");
    let greeting = block_on(greet("Ada"));
    println!("{}", greeting);

    println!("\n=== 4. Sequential async (100ms + 100ms = ~200ms) ===");
    let start = Instant::now();
    let (a, b) = block_on(sequential());
    println!("{} {}, took {:?}", a, b, start.elapsed());

    println!("\n=== 5. Concurrent async (~100ms total) ===");
    let start = Instant::now();
    let (a, b) = block_on(concurrent());
    println!("{} {}, took {:?}", a, b, start.elapsed());

    println!("\n=== 6. Threads: 4 threads incrementing a counter ===");
    threads_demo();

    // ========================================================================
    //  KEY TAKEAWAYS
    // ========================================================================
    //  * `async fn` returns a Future — a state machine generated by the
    //    compiler. The body does NOT run until polled.
    //  * `.await` polls a Future. If Pending, the current async fn yields
    //    control back to the executor.
    //  * Futures are LAZY. Without an executor, nothing happens.
    //  * Real-world runtimes: tokio, async-std, smol. They use epoll/kqueue
    //    for non-blocking I/O and a thread pool for execution.
    //  * Use `async` for many concurrent I/O-bound tasks (web servers,
    //    scrapers, network clients).
    //  * Use plain threads for CPU-bound work or simple scripts.
    //  * NEVER call `std::thread::sleep` inside async code — use the
    //    runtime's sleep function (e.g. `tokio::time::sleep`).
    //  * NEVER use `std::sync::Mutex` for long-held async locks — use
    //    `tokio::sync::Mutex` or `futures::lock::Mutex` instead.
    // ========================================================================
}

// ============================================================================
//  Appendix: what a real Cargo project looks like
// ============================================================================
//  To use async in a real project, add tokio to Cargo.toml:
//
//    [dependencies]
//    tokio = { version = "1", features = ["full"] }
//
//  Then mark `main` with the tokio attribute:
//
//    #[tokio::main]
//    async fn main() {
//        // You can use `.await` here directly.
//        let body = reqwest::get("https://example.com").await.unwrap()
//                                                      .text().await.unwrap();
//        println!("{}", body);
//    }
//
//  `#[tokio::main]` desugars to:
//
//    fn main() {
//        tokio::runtime::Runtime::new().unwrap().block_on(async_main())
//    }
//
//  where `async_main` is your async fn. This is the `block_on` you wrote by
//  hand above, but production-quality.
// ============================================================================
