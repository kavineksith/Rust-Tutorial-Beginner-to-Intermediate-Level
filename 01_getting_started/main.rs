// ============================================================================
//  CHAPTER 02 — RUST GET STARTED
//  Demonstrates the minimum structure of a runnable Rust program and the
//  most common things you do on day one.
//
//  Run with:   rustc main.rs  &&  ./main
// ============================================================================

// ----- Imports from the standard library -------------------------------
// `use` brings items into scope, like Python's `from ... import ...`.
// Here we import the `HashMap` type from the standard library.
use std::collections::HashMap;

// ----- A free function -------------------------------------------------
// Functions outside `main` are visible to the whole file. We will call
// `greet` from `main` below.
fn greet(name: &str) {
    // `&str` is a "string slice" — a borrowed view of some text.
    // We will cover borrowing in chapter 20.
    println!("Hello, {}! Welcome to Rust.", name);
}

// ----- The entry point -------------------------------------------------
fn main() {
    // Call our helper function with a string literal.
    greet("Ada");

    // Show that the standard library is available without extra setup:
    let mut fruits: Vec<&str> = Vec::new();   // a growable list of strings
    fruits.push("apple");
    fruits.push("banana");
    fruits.push("cherry");
    println!("Fruits: {:?}", fruits);          // `{:?}` is debug formatting

    // A HashMap is also part of std and ready to use:
    let mut ages: HashMap<&str, i32> = HashMap::new();
    ages.insert("Ada", 36);
    ages.insert("Linus", 54);
    println!("Ages: {:?}", ages);

    // ------------------------------------------------------------------
    //  Recap of the structure of a Rust file:
    //    1. (optional) `use` imports
    //    2. (optional) free functions, structs, enums, constants
    //    3. `fn main() { ... }` — the program's entry point
    //
    //  Every statement ends with a semicolon `;`.
    //  Every block is wrapped in curly braces `{ ... }`.
    // ------------------------------------------------------------------
}
