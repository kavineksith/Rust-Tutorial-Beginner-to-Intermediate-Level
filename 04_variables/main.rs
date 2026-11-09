// ============================================================================
//  CHAPTER 06 — RUST VARIABLES
//  `let`, `mut`, shadowing, and type annotations.
// ============================================================================

fn main() {
    // ---- Creating a variable: `let` --------------------------------------
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // ---- Variables are IMMUTABLE by default ------------------------------
    // This is one of Rust's most important features. If you write:
    //   let x = 5;
    //   x = 10;        // ERROR: cannot assign twice to immutable variable
    // The compiler will refuse to build your program. This prevents
    // accidental mutation bugs.

    // ---- Making a variable mutable: `mut` --------------------------------
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;                       // OK because we declared `mut`
    println!("After: {}", x);

    // ---- Type annotations ------------------------------------------------
    // Rust usually infers types. But you can (and sometimes must) annotate:
    let pi: f64 = 3.14159;        // explicit f64
    let count: u32 = 1_000_000;   // underscores for readability
    println!("pi = {}, count = {}", pi, count);

    // ---- Shadowing --------------------------------------------------------
    // You can declare a new variable with the SAME name as a previous one.
    // The new one "shadows" the old. This is different from `mut` because
    // it creates a brand-new binding (and can even change the type).
    let y = 5;
    let y = y + 1;          // shadows the previous `y`
    let y = y * 2;          // shadows again
    println!("y is now {}", y);  // 12

    // Shadowing can CHANGE THE TYPE — useful for parsing:
    let spaces = "   ";                  // &str
    let spaces = spaces.len();           // usize — same name, new type
    println!("Number of spaces: {}", spaces);

    // Contrast: with `mut` you CANNOT change type:
    //   let mut s = "   ";
    //   s = s.len();   // ERROR: expected &str, found usize

    // ---- Constants (preview, see chapter 08) -----------------------------
    // Constants use `const`, are UPPER_CASE, and need an explicit type:
    const MAX_USERS: u32 = 100;
    println!("Max users: {}", MAX_USERS);

    // ---- Tuple destructuring (preview, see chapter 24) -------------------
    let (a, b, c) = (1, 2, 3);
    println!("a={}, b={}, c={}", a, b, c);

    // ---- Unused variables ------------------------------------------------
    // Rust warns about unused variables. Prefix with `_` to silence:
    let _unused = 42;          // no warning
    // let unused = 42;        // would warn: unused variable
}
