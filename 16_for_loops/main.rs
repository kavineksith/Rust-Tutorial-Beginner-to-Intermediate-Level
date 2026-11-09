// ============================================================================
//  CHAPTER 15 — RUST FOR LOOPS
//  `for` is the most common loop in Rust. It iterates over anything that
//  implements the `Iterator` trait.
// ============================================================================

fn main() {
    // ---- Ranges -----------------------------------------------------------
    for n in 1..=5 {            // inclusive range: 1, 2, 3, 4, 5
        print!("{} ", n);
    }
    println!();

    for n in 1..5 {             // exclusive range: 1, 2, 3, 4
        print!("{} ", n);
    }
    println!();

    // ---- Reverse a range with `.rev()` -----------------------------------
    for n in (1..=5).rev() {
        print!("{} ", n);       // 5 4 3 2 1
    }
    println!();

    // ---- Step with `.step_by()` -----------------------------------------
    for n in (0..10).step_by(2) {
        print!("{} ", n);       // 0 2 4 6 8
    }
    println!();

    // ---- Iterating over an array ----------------------------------------
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("I like {}", fruit);
    }

    // ---- Iterating with index using `.iter().enumerate()` ---------------
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", i, fruit);
    }

    // ---- Iterating over a Vec -------------------------------------------
    let numbers = vec![10, 20, 30];
    for n in &numbers {         // borrow — does not move the vec
        print!("{} ", n);
    }
    println!();
    println!("vec still usable: {:?}", numbers);

    // ---- Iterating by value (consuming) ---------------------------------
    let words: Vec<String> = vec!["a".to_string(), "b".to_string()];
    for w in &words {           // borrow, just to print
        print!("[{}] ", w);
    }
    println!();

    // ---- Mutable iteration ----------------------------------------------
    let mut counts = [0; 5];
    for c in counts.iter_mut() {
        *c += 1;                // dereference and modify
    }
    println!("counts: {:?}", counts);   // [1, 1, 1, 1, 1]

    // ---- `break` and `continue` work as expected ------------------------
    for n in 1..=10 {
        if n == 5 { continue; }
        if n == 8 { break; }
        print!("{} ", n);       // 1 2 3 4 6 7
    }
    println!();

    // ---- Loop labels with `for` -----------------------------------------
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i * j == 4 {
                break 'outer;   // breaks out of the outer for loop
            }
            print!("({},{}) ", i, j);
        }
    }
    println!();

    // ---- Using `_` when you don't need the value ------------------------
    let mut rendered = 0;
    for _ in 0..10 {
        rendered += 1;          // we just want to repeat 10 times
    }
    println!("rendered = {}", rendered);
}
