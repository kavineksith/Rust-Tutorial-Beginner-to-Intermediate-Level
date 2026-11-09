// ============================================================================
//  CHAPTER 13 — RUST LOOPS
//  The `loop` keyword: an infinite loop you break out of explicitly.
// ============================================================================

fn main() {
    // ---- Basic infinite loop --------------------------------------------
    // `loop` runs forever until you `break`.
    let mut count = 0;
    loop {
        count += 1;
        if count >= 5 {
            break;            // exit the loop
        }
    }
    println!("Final count: {}", count);

    // ---- `break` with a value: loop is an expression --------------------
    // `loop` can RETURN a value via `break value;`:
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;   // this becomes the value of the loop
        }
    };
    println!("loop result = {}", result);   // 20

    // ---- `continue` skips the rest of the current iteration -------------
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 { break; }
        if n % 2 == 0 {
            continue;             // skip even numbers
        }
        print!("{} ", n);         // 1 3 5 7 9
    }
    println!();

    // ---- Loop labels (for nested loops) ---------------------------------
    // When you have loops inside loops, you can name them with `'label:`
    // and `break 'label` / `continue 'label` to control which loop is affected.
    let mut i = 0;
    'outer: loop {
        let mut j = 0;
        loop {
            j += 1;
            if j == 3 {
                continue;                          // continues the inner loop
            }
            if i == 2 && j == 2 {
                break 'outer;                      // breaks the OUTER loop
            }
            if j > 3 {
                break;                             // breaks the inner loop
            }
            println!("i={}, j={}", i, j);
        }
        i += 1;
        if i > 2 {
            break;
        }
    }

    // ---- Common use: retry logic ----------------------------------------
    let mut attempts = 0;
    let success = loop {
        attempts += 1;
        if attempts >= 3 {
            break true;            // pretend the 3rd attempt succeeded
        }
        // ... try something ...
    };
    println!("succeeded after {} attempts: {}", attempts, success);

    // ---- Returning from loop with labelled break ------------------------
    let found = 'search: loop {
        for n in 1..=100 {
            if n * n > 50 {
                break 'search n;   // return n from the labelled loop
            }
        }
        break 0;                   // fallback
    };
    println!("first n where n*n > 50: {}", found);
}
