// ============================================================================
//  CHAPTER 36 — RUST SMART POINTERS
//  Smart pointers OWN the data they point to (unlike plain references which
//  only borrow). They add capabilities: heap allocation, shared ownership,
//  interior mutability, reference counting.
// ============================================================================

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

fn main() {
    // ======================================================================
    //  1. Box<T> — heap allocation, single ownership
    // ======================================================================
    //  * Moves a value from the stack to the heap.
    //  * Single owner (no sharing).
    //  * Zero overhead vs a regular pointer — but allocates.
    //
    //  Common uses:
    //    - Large values you don't want to copy around the stack.
    //    - Recursive types (a struct that contains itself).
    //    - Trait objects (`Box<dyn Trait>`).
    let boxed_int = Box::new(42);
    println!("boxed_int = {}", boxed_int);   // dereferences automatically
    let unboxed = *boxed_int;                 // explicit deref
    println!("unboxed = {}", unboxed);

    // ---- Recursive type: a linked list ---------------------------------
    // Without Box this would be infinite-sized. Box breaks the cycle by
    // storing the recursive part on the heap (a pointer has known size).
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("list = {:?}", list);

    // ---- Trait objects -------------------------------------------------
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
    ];
    for s in &shapes {
        println!("area = {:.2}", s.area());
    }

    // ======================================================================
    //  2. Rc<T> — reference-counted, SHARED ownership, single-threaded
    // ======================================================================
    //  Multiple owners of the same data. The data is dropped when the LAST
    //  Rc is dropped. Uses a non-atomic counter (cheap, but NOT thread-safe).
    //
    //  Use only on a single thread. For multi-threaded code, use Arc.
    let a = Rc::new(String::from("shared data"));
    let b = Rc::clone(&a);          // bumps the refcount, does NOT copy data
    let c = Rc::clone(&a);
    println!("strong count after 3 clones: {}", Rc::strong_count(&a));  // 3
    println!("a = {}, b = {}, c = {}", a, b, c);
    drop(b);
    println!("strong count after dropping b: {}", Rc::strong_count(&a)); // 2
    // The String is freed only when a AND c are both dropped.

    // ---- Shared graph node (Rc is essential for this) -------------------
    // We need interior mutability to mutate children AFTER construction.
    // Wrapping the Node in RefCell lets us borrow_mut through Rc.
    let root = Rc::new(RefCell::new(Node { value: 1, children: vec![] }));
    let child1 = Rc::new(Node { value: 2, children: vec![] });
    let child2 = Rc::new(Node { value: 3, children: vec![] });

    // Both child1 and child2 are referenced from root AND from our local vars:
    root.borrow_mut().children = vec![Rc::clone(&child1), Rc::clone(&child2)];
    println!("root after building: {:?}", root.borrow());

    // ======================================================================
    //  3. Weak<T> — non-owning reference (breaks cycles)
    // ======================================================================
    //  Rc forms a strong reference. Two Rcs that point at each other form
    //  a CYCLE — neither will ever be dropped (memory leak).
    //
    //  Weak<T> is a non-owning reference. It does NOT prevent the data from
    //  being dropped. Use .upgrade() to get Option<Rc<T>> when you need it.
    let strong = Rc::new(42);
    let weak: Weak<i32> = Rc::downgrade(&strong);
    println!("strong count: {}, weak count: {}", Rc::strong_count(&strong), Rc::weak_count(&strong));

    if let Some(value) = weak.upgrade() {
        println!("weak upgraded to Rc: {}", value);
    }
    drop(strong);
    println!("after dropping strong, weak.upgrade() = {:?}", weak.upgrade()); // None

    // ======================================================================
    //  4. RefCell<T> — interior mutability, single-threaded
    // ======================================================================
    //  Lets you mutate through an IMMUTABLE reference. The borrow rules
    //  (one mutable XOR many immutable) are checked at RUNTIME instead of
    //  compile time.
    //
    //  Use when the borrow checker can't see that your code is safe (e.g.
    //  mutating fields inside an Rc, or implementing an observer pattern).
    let cell = RefCell::new(vec![1, 2, 3]);
    {
        let mut borrowed = cell.borrow_mut();    // runtime-checked mutable borrow
        borrowed.push(4);
    }
    println!("cell after push: {:?}", cell.borrow());

    // Combining Rc + RefCell is the classic pattern for shared mutable
    // state on a single thread:
    let shared_list: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared_list);
    let clone2 = Rc::clone(&shared_list);
    clone1.borrow_mut().push(4);     // mutate via one clone
    clone2.borrow_mut().push(5);     // mutate via the other
    println!("shared list: {:?}", shared_list.borrow());   // [1, 2, 3, 4, 5]

    // ---- Runtime borrow violation (panics!) ----------------------------
    // let bad = cell.borrow_mut();
    // let also_bad = cell.borrow_mut();   // PANIC: already mutably borrowed
    // drop(bad);

    // ======================================================================
    //  5. Arc<T> — atomic reference counting (thread-safe Rc)
    // ======================================================================
    //  Like Rc but uses ATOMIC operations for the counter — safe to share
    //  between threads. Slightly slower than Rc but the only option for
    //  multi-threaded shared ownership.
    let arc_data = Arc::new(Mutex::new(0));
    println!("initial Arc count: {}", Arc::strong_count(&arc_data));

    let mut handles = vec![];
    for _ in 0..4 {
        let arc_clone = Arc::clone(&arc_data);
        handles.push(std::thread::spawn(move || {
            // Each thread increments the shared counter 1000 times.
            for _ in 0..1000 {
                let mut guard = arc_clone.lock().unwrap();
                *guard += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final Arc count: {}, value: {}", Arc::strong_count(&arc_data), *arc_data.lock().unwrap());

    // ======================================================================
    //  SUMMARY TABLE
    // ======================================================================
    //  Box<T>            heap, single owner, no overhead
    //  Rc<T>             shared owner, single thread
    //  Weak<T>           non-owning reference (for cycles)
    //  RefCell<T>        interior mutability, single thread (runtime borrow check)
    //  Arc<T>            shared owner, multi thread (atomic counter)
    //  Mutex<T> / RwLock<T>  interior mutability, multi thread
    //
    //  Combos:
    //    Rc<RefCell<T>>       single-thread shared mutable
    //    Arc<Mutex<T>>        multi-thread shared mutable  (most common)
    //    Arc<RwLock<T>>       multi-thread shared mutable, mostly reads
}

// ======================================================================
//  Helper types for the examples above
// ======================================================================
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}
