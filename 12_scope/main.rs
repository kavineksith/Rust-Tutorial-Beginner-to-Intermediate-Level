// ============================================================================
//  CHAPTER 17 — RUST SCOPE
//  Where a variable is valid. Closely tied to ownership and memory safety.
// ============================================================================

fn main() {
    // ---- Block scope -----------------------------------------------------
    // A variable is valid from where it is declared until the END of the
    // block `{ ... }` it lives in.
    {
        let x = 5;             // x is born here
        println!("x = {}", x); // x is valid
    }                          // x goes out of scope here -> x is dropped
    // println!("{}", x);      // ERROR: x is not in scope

    // ---- Nested blocks can shadow outer variables ------------------------
    let name = "outer";
    println!("before block: {}", name);
    {
        let name = "inner";        // new binding, shadows outer
        println!("inside block: {}", name);
    }                              // inner `name` goes out of scope
    println!("after block: {}", name);   // back to outer

    // ---- The same name can be re-declared in the same scope --------------
    let count = 1;
    let count = count + 1;     // shadowing in the SAME scope
    let count = count * 10;
    println!("count = {}", count);   // 20

    // ---- Variables declared in a loop body only live one iteration ------
    for i in 0..3 {
        let temp = i * i;       // `temp` is created fresh each iteration
        println!("i={}, temp={}", i, temp);
    }                           // `temp` is dropped at the end of each iter
    // println!("{}", temp);    // ERROR: temp not in scope

    // ---- Function scope --------------------------------------------------
    // Parameters are scoped to the function body:
    let result = square(7);
    println!("7^2 = {}", result);

    // ---- `'static` lifetime: lives for the whole program ----------------
    let s: &'static str = "I am a string literal — I live forever.";
    println!("{}", s);
    // String literals have the `'static` lifetime — they are baked into the
    // binary, so they are valid for the entire program.

    // ---- Scope and drop --------------------------------------------------
    // When a value's owner goes out of scope, the value is DROPPED
    // (its memory is freed automatically). This is the heart of ownership.
    {
        let owned = String::from("I am allocated on the heap");
        println!("{}", owned);
    }                          // <- `owned` is dropped here; memory freed

    // ---- Variables can be moved OUT of a block by returning them --------
    let recovered = make_string();
    println!("recovered: {}", recovered);
}

fn square(n: i32) -> i32 {
    // `n` is in scope for the whole function body.
    let result = n * n;
    result                    // returned — ownership transferred to caller
    // `n` and `result` are dropped here (but `result`'s value was returned).
}

fn make_string() -> String {
    let s = String::from("created inside a function");
    s                         // ownership moves to the caller — s is NOT dropped
}
