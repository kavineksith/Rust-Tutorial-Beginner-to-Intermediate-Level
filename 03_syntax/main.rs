// ============================================================================
//  CHAPTER 03 — RUST SYNTAX
//  The anatomy of a Rust program: functions, variables, statements,
//  expressions, blocks, and macros.
// ============================================================================

// ----- Function definition -------------------------------------------------
//  fn <name>(<parameters>) -> <return type> { <body> }
//
//  Parameters must have explicit types in function signatures —
//  Rust does NOT infer types at API boundaries (only inside function bodies).
fn add(a: i32, b: i32) -> i32 {
    // The LAST expression in a block (with NO trailing semicolon) is the
    // block's value. This is how functions "return" a value without `return`.
    a + b
    // ^^^ expression: a + b evaluates to i32, becomes the return value.
    // If we wrote `a + b;` it would become a statement returning `()`.
}

// A function that returns nothing has the implicit return type `()` (unit).
fn print_twice(message: &str) {
    println!("{}", message);
    println!("{}", message);
}

// ----- The entry point -----------------------------------------------------
fn main() {
    // ---- `let` bindings ----
    // By default variables are IMMUTABLE. Their value cannot change.
    let x = 5;            // type inferred as i32
    let y: i64 = 10;      // type explicitly annotated as i64

    // ---- Calling a function ----
    let sum = add(x, y as i32);   // `as` is an explicit type cast
    println!("{} + {} = {}", x, y, sum);

    // ---- Statements vs Expressions ----
    // A STATEMENT performs an action and returns `()` (unit).
    //   let z = 5;            // statement
    //   println!("hi");       // statement
    //
    // An EXPRESSION evaluates to a value.
    //   5 + 6                 // expression -> 11
    //   { let a = 1; a + 2 }  // block expression -> 3
    //
    // Blocks are expressions! The last expression (no semicolon) is the value:
    let block_value = {
        let a = 2;
        let b = 3;
        a * b            // <-- no semicolon: this is the block's value
    };
    println!("Block value: {}", block_value); // 6

    // ---- `if` is also an expression ----
    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("number = {}", number);

    // ---- Macros vs Functions ----
    // `println!` is a MACRO, not a function. The `!` is the giveaway.
    // Macros are expanded at compile time. We use them for things that
    // functions cannot do, like taking a variable number of arguments
    // or formatting strings at compile time.
    print_twice("Macros look like functions but end with !");
}
