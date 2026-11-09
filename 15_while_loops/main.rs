// ============================================================================
//  CHAPTER 14 — RUST WHILE LOOPS
//  `while` repeats a block while a condition is true.
//  `while let` combines a pattern match with looping.
// ============================================================================

fn main() {
    // ---- Basic while loop -----------------------------------------------
    let mut n = 1;
    while n <= 5 {
        println!("n = {}", n);
        n += 1;
    }

    // ---- Counting down ---------------------------------------------------
    let mut countdown = 3;
    while countdown > 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // ---- While with a boolean flag --------------------------------------
    let mut keep_going = true;
    let mut iteration = 0;
    while keep_going {
        iteration += 1;
        println!("iteration {}", iteration);
        if iteration >= 3 {
            keep_going = false;
        }
    }

    // ---- Using `break` and `continue` inside while ----------------------
    let mut i = 0;
    while i < 10 {
        i += 1;
        if i % 3 == 0 {
            continue;             // skip multiples of 3
        }
        if i == 8 {
            break;                // stop early
        }
        print!("{} ", i);         // 1 2 4 5 7
    }
    println!();

    // ---- `while let` — pattern match + loop -----------------------------
    // Pops items from a stack (a Vec used as a stack) until it is empty:
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped: {}", top);
    }
    // Output: popped: 3, popped: 2, popped: 1

    // ---- `while let` with Result ----------------------------------------
    // Common when reading lines from a file:
    let mut lines = vec!["first", "second", "third"].into_iter();
    while let Some(line) = lines.next() {
        println!("line: {}", line);
    }

    // ---- When to use while vs for ---------------------------------------
    // `while` is best when the number of iterations is NOT known in advance
    // (e.g. polling, retrying, processing until empty).
    //
    // If you are iterating over a known collection or range, use `for`.

    // ---- Be careful: infinite loops -------------------------------------
    // If you forget to update the condition variable, `while` runs forever:
    //   let mut x = 0;
    //   while x < 10 {
    //       println!("{}", x);   // oops, x never changes -> infinite loop
    //   }
    //
    // Clippy (the Rust linter) will warn about obvious cases.
}
