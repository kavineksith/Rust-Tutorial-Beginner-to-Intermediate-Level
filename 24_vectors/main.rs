// ============================================================================
//  CHAPTER 23 — RUST VECTORS
//  `Vec<T>` is Rust's growable, heap-allocated list. The most common
//  collection in Rust.
// ============================================================================

fn main() {
    // ---- Creating a Vec --------------------------------------------------
    let v1: Vec<i32> = Vec::new();                  // empty, type annotated
    let v2 = vec![1, 2, 3, 4, 5];                   // macro, inferred type
    let v3: Vec<&str> = vec!["a", "b", "c"];
    println!("v1: {:?}, v2: {:?}, v3: {:?}", v1, v2, v3);

    // ---- Adding elements -------------------------------------------------
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("after pushes: {:?}", v);

    // Insert at a specific index (shifts elements):
    v.insert(1, 15);
    println!("after insert(1, 15): {:?}", v);   // [10, 15, 20, 30]

    // Remove from a specific index (shifts elements):
    let removed = v.remove(0);
    println!("removed {} -> {:?}", removed, v);  // 10, [15, 20, 30]

    // ---- Accessing elements ---------------------------------------------
    let third = v[2];                 // 30 — panics if out of bounds
    let first = v.get(0);             // Option<&T> — safe, returns None
    let oob   = v.get(100);           // Some or None — never panics
    println!("third = {}, first = {:?}, oob = {:?}", third, first, oob);

    // ---- Length, capacity, emptiness ------------------------------------
    println!("len = {}, capacity = {}, empty? {}", v.len(), v.capacity(), v.is_empty());

    // Pre-allocate capacity for performance when you know the size:
    let mut big: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..1000 {
        big.push(i);                  // no reallocation
    }
    println!("big.len = {}, big.cap = {}", big.len(), big.capacity());

    // ---- Popping the last element ---------------------------------------
    let mut stack = vec![1, 2, 3];
    let last = stack.pop();           // Some(3)
    println!("popped: {:?}, remaining: {:?}", last, stack);

    // ---- Iterating -------------------------------------------------------
    for n in &v {                     // borrow
        print!("{} ", n);             // 15 20 30
    }
    println!();

    for (i, n) in v.iter().enumerate() {
        print!("[{}]={} ", i, n);
    }
    println!();

    // ---- Mutable iteration ----------------------------------------------
    let mut nums = vec![1, 2, 3, 4];
    for n in nums.iter_mut() {
        *n *= 10;                     // multiply each by 10
    }
    println!("after *10: {:?}", nums);   // [10, 20, 30, 40]

    // ---- Consuming iteration (takes ownership) --------------------------
    let owned = vec![String::from("a"), String::from("b")];
    for s in owned {                  // moves each String out
        print!("[{}] ", s);
    }
    println!();
    // println!("{:?}", owned);         // ERROR: owned was moved

    // ---- Slicing ---------------------------------------------------------
    let slice: &[i32] = &nums[1..3];  // [20, 30]
    println!("slice: {:?}", slice);

    // ---- Common operations ----------------------------------------------
    let sum: i32 = nums.iter().sum();
    let max: i32 = *nums.iter().max().unwrap();
    let min: i32 = *nums.iter().min().unwrap();
    println!("sum={}, max={}, min={}", sum, max, min);

    // ---- Sorting and reversing ------------------------------------------
    let mut mixed = vec![3, 1, 4, 1, 5, 9, 2, 6];
    mixed.sort();
    println!("sorted: {:?}", mixed);                  // [1, 1, 2, 3, 4, 5, 6, 9]
    mixed.sort_by(|a, b| b.cmp(a));                   // descending
    println!("desc: {:?}", mixed);
    mixed.reverse();
    println!("reversed: {:?}", mixed);

    // ---- Contains, retain, extend ---------------------------------------
    let mut bag = vec![1, 2, 3, 4, 5];
    println!("contains 3? {}", bag.contains(&3));
    bag.retain(|&x| x % 2 == 0);                      // keep only evens
    println!("after retain: {:?}", bag);              // [2, 4]
    bag.extend([6, 8, 10]);
    println!("after extend: {:?}", bag);              // [2, 4, 6, 8, 10]

    // ---- Filtering and mapping (collect into a new Vec) -----------------
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    let evens: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
    println!("squares: {:?}, evens: {:?}", squares, evens);

    // ---- Vec of different types via enum (idiomatic) --------------------
    enum Value {
        Int(i32),
        Text(String),
    }
    let mixed: Vec<Value> = vec![
        Value::Int(42),
        Value::Text(String::from("hello")),
    ];
    for v in &mixed {
        match v {
            Value::Int(n) => println!("int: {}", n),
            Value::Text(s) => println!("text: {}", s),
        }
    }
}
