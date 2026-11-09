// ============================================================================
//  CHAPTER 37 — RUST CONCURRENCY
//  Rust's "fearless concurrency": the ownership and Send/Sync traits let
//  the compiler PROVE your concurrent code is data-race free.
// ============================================================================

use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // ======================================================================
    //  1. Spawning threads
    // ======================================================================
    //  thread::spawn takes a closure (FnOnce). The closure runs on a new
    //  OS thread. The return value is a JoinHandle.
    let handle = thread::spawn(|| {
        for i in 0..3 {
            println!("  spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        42
    });

    for i in 0..3 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // .join() blocks until the thread finishes and returns its result.
    let result = handle.join().unwrap();
    println!("spawned thread returned: {}", result);

    // ======================================================================
    //  2. The `move` keyword — moving data INTO a thread
    // ======================================================================
    //  By default, closures borrow. But threads may outlive the scope that
    //  created them — so borrowed references usually don't work.
    //  `move` forces the closure to TAKE OWNERSHIP of captured variables.
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  thread sees: {:?}", data);
    });
    // println!("{:?}", data);   // ERROR: data was moved into the thread
    handle.join().unwrap();

    // ======================================================================
    //  3. `Send` and `Sync` — the concurrency traits
    // ======================================================================
    //  Send: a type can be transferred ACROSS thread boundaries.
    //  Sync: &T can be shared across threads (i.e. T is safe to reference
    //        from multiple threads simultaneously).
    //
    //  Most types are Send + Sync. Notable exceptions:
    //    * Rc<T>        — NOT Send (non-atomic counter)
    //    * RefCell<T>   — NOT Sync (non-atomic borrow flags)
    //    * MutexGuard   — NOT Send (must release on the same thread)
    //
    //  The compiler checks these at spawn time. You cannot accidentally
    //  share an Rc across threads.

    // ======================================================================
    //  4. Channels — message passing between threads
    // ======================================================================
    //  mpsc = "multiple producer, single consumer".
    //  let (tx, rx) = mpsc::channel();
    //  You can CLONE tx to have multiple senders. rx is single-consumer.
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread that sends 3 messages then exits:
    let tx_clone = tx.clone();   // multiple producers — clone the sender
    thread::spawn(move || {
        let messages = ["hello", "from", "thread"];
        for m in messages {
            tx_clone.send(m.to_string()).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Drop the original tx so the channel closes when the spawned tx is gone:
    drop(tx);

    // Receive until the channel closes (rx becomes empty):
    for received in rx {
        println!("  received: {}", received);
    }

    // ======================================================================
    //  5. Shared state with Arc + Mutex
    // ======================================================================
    //  When threads need to MUTATE shared data, wrap it in Arc<Mutex<T>>.
    //  Arc gives shared ownership; Mutex gives exclusive access.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // .lock() blocks until exclusive access is available.
                // It returns a Result (in case a thread panicked while
                // holding the lock — called "poisoning").
                let mut guard = counter.lock().unwrap();
                *guard += 1;
                // guard is dropped here, releasing the lock
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final counter = {}", *counter.lock().unwrap());   // 4000

    // ======================================================================
    //  6. RwLock — many readers OR one writer
    // ======================================================================
    //  Mutex gives exclusive access to everyone. RwLock lets multiple
    //  READERS in at once — but only one WRITER (and no readers while writing).
    //  Best when reads vastly outnumber writes.
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Spawn several readers:
    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let read = data.read().unwrap();      // shared read lock
            println!("  reader {} sees: {:?}", i, *read);
        }));
    }
    // Spawn a writer:
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write = data.write().unwrap(); // exclusive write lock
            write.push(4);
        });
        handle.join().unwrap();
    }
    for h in handles { h.join().unwrap(); }
    println!("after writer: {:?}", *data.read().unwrap());   // [1, 2, 3, 4]

    // ======================================================================
    //  7. Poisoned mutexes
    // ======================================================================
    //  If a thread panics while holding a Mutex, the Mutex becomes
    //  "poisoned". Subsequent .lock() calls return Err.
    //  This is a safety feature: the data might be in an inconsistent
    //  state, so further access is refused.
    //
    //  let m = Mutex::new(0);
    //  let _g = m.lock().unwrap();
    //  panic!("oops");           // mutex is now poisoned
    //  // let _g2 = m.lock();     // returns Err(PoisonError)

    // ======================================================================
    //  8. Scoped threads (Rust 1.63+) — borrow without `move`
    // ======================================================================
    //  thread::scope lets child threads BORROW from the parent scope
    //  without needing `move` or `Arc`. The scope guarantees all child
    //  threads are joined before it returns — so borrowed refs are safe.
    let mut arr = [0u32; 4];
    thread::scope(|s| {
        // each thread handles a slice of the array
        for chunk in arr.chunks_mut(2) {
            s.spawn(move || {
                for x in chunk.iter_mut() {
                    *x = 42;
                }
            });
        }
    });   // <- all spawned threads are joined here, before arr is used again
    println!("scoped threads result: {:?}", arr);   // [42, 42, 42, 42]

    // ======================================================================
    //  SUMMARY
    // ======================================================================
    //  * thread::spawn       — start an OS thread (use `move` for captured data)
    //  * Send + Sync         — compiler-checked thread-safety traits
    //  * mpsc channels       — message passing (multiple producer, single consumer)
    //  * Arc<T>              — atomic reference counting (multi-thread shared owner)
    //  * Mutex<T>            — exclusive access (one at a time)
    //  * RwLock<T>           — many readers OR one writer
    //  * thread::scope       — borrow across threads without Arc (Rust 1.63+)
    //  * Arc<Mutex<T>>       — the canonical multi-thread shared mutable state
    //
    //  The compiler PREVENTS:
    //    * sharing non-Send types (e.g. Rc<T>) across threads
    //    * sharing non-Sync types (e.g. RefCell<T>) across threads
    //    * data races (no two threads can mutate without synchronization)
    // ======================================================================
}
