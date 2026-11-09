// ============================================================================
//  CHAPTER 01 — RUST INTRO
//  A first look at Rust. Compile with:  rustc main.rs  &&  ./main
// ============================================================================

// `fn` is the keyword that declares a FUNCTION.
// `main` is the entry point of every Rust executable — execution begins here.
fn main() {
    // `println!` is a MACRO (notice the exclamation mark `!`).
    // Macros look like functions but generate code at compile time.
    // `println!` prints its argument followed by a newline to standard output.
    println!("Hello, World!");

    // ------------------------------------------------------------------
    //  Why is this interesting?
    //  ------------------------------------------------------------------
    //  * "Hello, World!" is a string literal (type &'static str).
    //  * println! is type-checked at compile time: if your placeholders
    //    do not match the arguments, the program will NOT compile.
    //  * There is no hidden runtime, no garbage collector started
    //    behind your back — `main` simply runs and exits.
    // ------------------------------------------------------------------

    // You can print multiple lines with one println! using `\n`:
    println!("Rust is fast.\nRust is safe.\nRust is fun.");

    // Let's prove Rust can do basic arithmetic directly in the macro
    // via a placeholder `{}`:
    let answer = 6 * 7;          // `let` binds a variable (immutable by default)
    println!("The answer to everything is: {}", answer);

    // Rust's println! supports named arguments too:
    println!(
        "{language} was first released in {year}.",
        language = "Rust",
        year = 2010
    );

    // ------------------------------------------------------------------
    //  Try it yourself:
    //    1. Change the text inside the first println!.
    //    2. Add a new `let` and print it.
    //    3. Try removing the `!` in `println!` — notice the compiler error.
    // ------------------------------------------------------------------
}
