// ============================================================================
//  CHAPTER 05 — RUST COMMENTS
//  How to document and annotate your code.
// ============================================================================

// ---- Single-line comments -------------------------------------------------
// Anything after `//` on a line is ignored by the compiler.
// Use them to explain WHY, not WHAT (the code already says what).

// This is a single-line comment. It can be on its own line.
//   let x = 5; // Or at the end of a line of code.  (shown as illustration)

// ---- Block comments -------------------------------------------------------
/* Block comments use /* ... */ and can span multiple lines.
   They are useful for temporarily disabling a chunk of code,
   or for long explanations that would clutter the line with //. */

/* Block comments can also be /* nested */ in Rust,
   which is great for commenting out code that already contains
   block comments. */

// ---- Doc comments ---------------------------------------------------------
/// Adds two integers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function never panics.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// `//!` is an INNER doc comment — used to document the enclosing item
// (typically the module or crate itself). It MUST be placed at the very
// top of a file (or before items). Example (shown here as plain comments
// because they cannot legally appear mid-file):
//
//   //! # Chapter 05 — Comments
//   //!
//   //! A small demo of every kind of comment Rust supports.

fn main() {
    // Using the function we documented above:
    let result = add(2, 3);
    println!("2 + 3 = {}", result);

    // ------------------------------------------------------------------
    //  Comment style tips:
    //
    //  * Prefer // for everyday comments — it's the most common.
    //  * Use /// to document public functions, structs, and enums.
    //    `cargo doc` turns these into HTML documentation.
    //  * Use //! at the top of a file to describe the whole module.
    //  * Avoid stating the obvious:
    //      let x = 5; // assign 5 to x          <- useless
    //      let max_login_attempts = 5;          // good name, no comment
    // ------------------------------------------------------------------

    // You can also use comments to "comment out" code while debugging:
    // println!("This line is disabled");
    println!("Only this line runs.");
}
