// ============================================================================
//  CHAPTER 19 — RUST OWNERSHIP
//  Ownership is THE feature that makes Rust memory-safe without a GC.
//  Every value has exactly one owner; when the owner goes out of scope,
//  the value is dropped.
// ============================================================================

fn main() {
    // ======================================================================
    //  RULE 1: Each value has ONE owner at a time.
    // ======================================================================
    let a = String::from("Hello");
    let b = a;                          // ownership MOVES from a to b
    // println!("{}", a);               // ERROR: a no longer owns the value
    println!("b owns it: {}", b);

    // ======================================================================
    //  RULE 2: When the owner goes out of scope, the value is dropped.
    // ======================================================================
    {
        let local = String::from("I live in this block");
        println!("{}", local);
    }                                   // <- local is dropped here, memory freed
    // println!("{}", local);           // ERROR: local is not in scope

    // ======================================================================
    //  Copy vs Move — depends on the type
    // ======================================================================
    // Simple types (integers, floats, bools, chars, and tuples/arrays of
    // them) implement the `Copy` trait. They are COPIED instead of moved:
    let x = 5;
    let y = x;                          // x is COPIED to y
    println!("x = {}, y = {}", x, y);   // BOTH still usable

    // Heap types like String MOVE:
    let s1 = String::from("Hi");
    let s2 = s1;                        // s1 is MOVED to s2
    // println!("{}", s1);              // ERROR: s1 was moved
    println!("{}", s2);

    // ======================================================================
    //  Clone — explicitly make a deep copy of a heap type
    // ======================================================================
    let original = String::from("Deep copy me");
    let cloned = original.clone();      // explicit deep copy
    println!("original = {}, cloned = {}", original, cloned);   // both usable

    // ======================================================================
    //  Ownership and functions — passing a value MOVES it
    // ======================================================================
    let text = String::from("pass me");
    take_ownership(text);               // text's ownership moves into the function
    // println!("{}", text);            // ERROR: text was moved

    // But Copy types are copied, not moved:
    let number = 42;
    make_copy(number);                  // number is COPIED (i32 is Copy)
    println!("number still usable: {}", number);

    // ======================================================================
    //  Returning a value TRANSFERS ownership back to the caller
    // ======================================================================
    let new_string = give_ownership();
    println!("I now own: {}", new_string);

    let from_arg = String::from("from main");
    let returned = take_and_give_back(from_arg);   // ownership in then back out
    println!("got it back: {}", returned);

    // ======================================================================
    //  The borrow alternative (preview of chapter 20)
    // ======================================================================
    let greeting = String::from("Hello, world!");
    let len = calculate_length(&greeting);   // pass a REFERENCE — no move
    println!("'{}' has length {}", greeting, len);   // greeting still usable
}

fn take_ownership(s: String) {
    println!("I now own: {}", s);
}                                       // s is dropped here, memory freed

fn make_copy(n: i32) {
    println!("I received a copy: {}", n);
}                                       // n is a copy, nothing to free

fn give_ownership() -> String {
    String::from("from inside the function")   // ownership moves to caller
}

fn take_and_give_back(s: String) -> String {
    s                       // ownership moves in, then back out
}

fn calculate_length(s: &String) -> usize {
    s.len()                 // we BORROW s, we do not own it
}                           // s (the reference) goes out of scope, but the
                            // String it points to is NOT dropped
