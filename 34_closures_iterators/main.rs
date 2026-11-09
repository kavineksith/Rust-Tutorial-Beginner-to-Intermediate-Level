// ============================================================================
//  CHAPTER 35 — RUST CLOSURES & ITERATORS (in depth)
//  Closures are anonymous functions that CAPTURE their environment.
//  Iterators are lazy sequences that produce values on demand.
//  Together they are Rust's most expressive data-processing toolset.
// ============================================================================

fn main() {
    // ======================================================================
    //  PART A — CLOSURES
    // ======================================================================

    // ---- A.1 Basic closure syntax ---------------------------------------
    // Closures look like functions but use `|args| body` syntax.
    // Type inference is more aggressive than for `fn`.
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));   // 6

    // Multi-line body — use `{ }`:
    let double = |x: i32| {
        let result = x * 2;
        result                    // last expression is the return value
    };
    println!("double(5) = {}", double(5));     // 10

    // No parameters:
    let say_hi = || println!("hi from a closure");
    say_hi();

    // ---- A.2 Closures CAPTURE their environment -------------------------
    // This is the key difference from regular `fn`. A closure can borrow
    // (or move) variables from the surrounding scope.
    let multiplier = 3;
    let multiply = |x| x * multiplier;   // captures `multiplier` by reference
    println!("multiply(5) = {}", multiply(5));   // 15

    // ---- A.3 Three capture modes (matching the three Fn traits) --------
    // 1. `Fn`        — captures by reference (immutable borrow)
    // 2. `FnMut`     — captures by mutable reference
    // 3. `FnOnce`    — takes ownership (moves the captured variable)
    //
    // The compiler picks the LEAST restrictive mode that works.

    // Fn: only reads from the environment
    let name = String::from("Ada");
    let greet = || println!("Hello, {}!", name);   // borrows `name` immutably
    greet();
    println!("name still usable: {}", name);

    // FnMut: mutates the environment
    let mut counter = 0;
    let mut increment = || { counter += 1; };      // borrows mutably
    increment();
    increment();
    increment();
    println!("counter after 3 increments: {}", counter);   // 3

    // FnOnce: moves the environment (can only be called once)
    let text = String::from("owned");
    let consume = move || {                         // `move` forces ownership
        println!("consuming: {}", text);
        drop(text);
    };
    consume();
    // println!("{}", text);   // ERROR: text was moved into the closure
    // consume();              // ERROR: closure can only be called once

    // ---- A.4 The `move` keyword -----------------------------------------
    // `move` forces the closure to TAKE OWNERSHIP of captured variables.
    // Essential for threads and async tasks that outlive the current scope.
    let data = vec![1, 2, 3];
    let printer = move || println!("{:?}", data);
    printer();
    // println!("{:?}", data);   // ERROR: data was moved

    // ---- A.5 Closures as function arguments (`impl Fn`) -----------------
    let result = apply(|x| x * x, 5);
    println!("apply square to 5: {}", result);   // 25

    let mut acc = 0;
    apply_mut(|x| { acc += x; acc }, 10);
    println!("acc after apply_mut: {}", acc);    // 10

    // ---- A.6 Returning closures (`Box<dyn Fn>` or `impl Fn`) -----------
    let adder = make_adder(10);
    println!("make_adder(10)(5) = {}", adder(5));   // 15

    // ======================================================================
    //  PART B — ITERATORS
    // ======================================================================

    // ---- B.1 The Iterator trait -----------------------------------------
    // Every iterator implements:
    //     trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
    //
    // Calling .next() returns Some(item) or None when exhausted.
    let mut iter = [10, 20, 30].iter();
    println!("{:?}", iter.next());   // Some(10)
    println!("{:?}", iter.next());   // Some(20)
    println!("{:?}", iter.next());   // Some(30)
    println!("{:?}", iter.next());   // None

    // ---- B.2 Iterators are LAZY -----------------------------------------
    // Creating an iterator does NOTHING. You must consume it (with .next(),
    // a `for` loop, or a "consuming adaptor" like .collect / .sum / .count).
    let _lazy = [1, 2, 3].iter().map(|x| x * 2);   // nothing runs yet
    // We need to consume it:
    let doubled: Vec<i32> = [1, 2, 3].iter().map(|&x| x * 2).collect();
    println!("doubled: {:?}", doubled);   // [2, 4, 6]

    // ---- B.3 Three forms of iteration -----------------------------------
    let v = vec![1, 2, 3];
    for _ in v.iter()     { }   // borrows:        yields &i32
    let mut v = vec![1, 2, 3];
    for _ in v.iter_mut() { }   // mutable borrow: yields &mut i32   (needs `mut v`)
    let v2 = vec![4, 5, 6];
    for _ in v2.into_iter() { } // consumes:       yields i32

    // ---- B.4 Adaptors: map / filter / take / skip / enumerate -----------
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let squares: Vec<i32> = nums.iter().map(|&x| x * x).collect();
    println!("squares: {:?}", squares);

    // filter: keep only elements matching a predicate
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // take: stop after N elements
    let first_three: Vec<&i32> = nums.iter().take(3).collect();
    println!("first three: {:?}", first_three);

    // skip: drop the first N elements
    let after_three: Vec<&i32> = nums.iter().skip(3).collect();
    println!("after three: {:?}", after_three);

    // enumerate: pair each element with its index
    for (i, n) in nums.iter().take(3).enumerate() {
        println!("[{}] = {}", i, n);
    }

    // ---- B.5 Chaining adaptors (zero-cost, fused at compile time) -------
    let result: i32 = nums
        .iter()
        .filter(|&&x| x % 2 == 0)        // keep evens
        .map(|&x| x * x)                 // square them
        .take(3)                         // first 3 of those
        .sum();                          // sum them
    println!("sum of first 3 even squares: {}", result);   // 4 + 16 + 36 = 56

    // ---- B.6 Consuming adaptors: sum / product / min / max / count ------
    let sum: i32    = nums.iter().sum();
    let product: i32 = nums.iter().product();
    let max: &i32   = nums.iter().max().unwrap();
    let min: &i32   = nums.iter().min().unwrap();
    let count: usize = nums.iter().count();
    println!("sum={} product={} max={} min={} count={}", sum, product, max, min, count);

    // ---- B.7 fold (a.k.a. reduce) ---------------------------------------
    // fold(initial, |acc, item| next_acc) — the most general consumer.
    let concatenated: String = ["a", "b", "c", "d"]
        .iter()
        .fold(String::new(), |acc, &s| acc + s);
    println!("folded: {}", concatenated);   // abcd

    // ---- B.8 inspect — peek without consuming (great for debugging) -----
    let result: Vec<i32> = (1..=5)
        .inspect(|x| print!("[{}] ", x))    // 1 2 3 4 5
        .map(|x| x * 10)
        .collect();
    println!();
    println!("after inspect+map: {:?}", result);   // [10, 20, 30, 40, 50]

    // ---- B.9 flat_map — map then flatten one level ----------------------
    let words = ["hello world", "foo bar"];
    let chars_per_word: Vec<usize> = words
        .iter()
        .flat_map(|s| s.split_whitespace())   // flatten the iterator of words
        .map(|w| w.len())
        .collect();
    println!("chars per word: {:?}", chars_per_word);   // [5, 5, 3, 3]

    // ---- B.10 zip — pair elements from two iterators --------------------
    let names = ["Alice", "Bob", "Carol"];
    let ages = [30, 25, 40];
    let pairs: Vec<(&str, i32)> = names.iter().copied().zip(ages.iter().copied()).collect();
    println!("zipped: {:?}", pairs);

    // ---- B.11 Custom iterator (implementing Iterator manually) ---------
    struct Counter { count: u32, max: u32 }
    impl Counter {
        fn new(max: u32) -> Self { Counter { count: 0, max } }
    }
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

    let collected: Vec<u32> = Counter::new(5).collect();
    println!("custom Counter: {:?}", collected);   // [1, 2, 3, 4, 5]

    let sum_of_first_10_squares: u32 = Counter::new(10)
        .map(|x| x * x)
        .sum();
    println!("sum of squares 1..=10: {}", sum_of_first_10_squares);   // 385

    // ---- B.12 collect into different containers -------------------------
    // collect() is smart — it can build Vec, HashMap, HashSet, String, etc.
    use std::collections::HashSet;
    let unique: HashSet<i32> = vec![1, 1, 2, 3, 3, 3].into_iter().collect();
    println!("unique: {:?}", unique);   // {1, 2, 3} (order not defined)

    let text: String = vec!['h', 'i'].into_iter().collect();
    println!("as String: {}", text);    // hi
}

// ---- Helper functions for closure examples ------------------------------

// Takes any closure that implements `Fn(i32) -> i32`.
// `impl Fn` is sugar for `<F: Fn(i32) -> i32>`.
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// Takes a closure that needs mutable access to its environment (FnMut).
fn apply_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
    f(x)
}

// Returns a closure. Closures don't have a named type, so we return
// `impl Fn(i32) -> i32` (the compiler generates a unique anonymous type).
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n        // `move` captures `n` by value
}

// Note: returning `impl Fn` only works if the closure does NOT borrow from
// local variables that would go out of scope. Use `move` or `Box<dyn Fn>`.
