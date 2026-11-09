// ============================================================================
//  CHAPTER 08 — RUST CONSTANTS
//  `const` and `static`: values that live for the entire program.
// ============================================================================

// ---- `const` -------------------------------------------------------------
//  * Always immutable (no `mut` allowed).
//  * Value must be a compile-time constant expression.
//  * Type MUST be annotated explicitly.
//  * Naming convention: UPPER_SNAKE_CASE.
//  * Inlined wherever it is used — no fixed memory address.
const MAX_USERS: u32 = 100;
const PI: f64 = 3.141592653589793;
const GREETING: &str = "Welcome to Rust";
// const MULTIPLIED: i32 = MAX_USERS * 2;  // also OK — constant expression

// ---- `static` ------------------------------------------------------------
//  * Has a fixed memory address for the whole program's lifetime.
//  * May be mutable (with `static mut`), but accessing it is `unsafe`.
//  * Type MUST be annotated explicitly.
//  * Use sparingly — usually a `const` is what you want.
static APP_NAME: &str = "MyApp v1.0";
static COUNTER_START: i32 = 0;

fn main() {
    // Constants are used just like variables:
    println!("{} allows up to {} users.", GREETING, MAX_USERS);
    println!("Pi is approximately {:.4}", PI);
    println!("App: {}", APP_NAME);

    // ---- const vs let ----------------------------------------------------
    // `let` evaluates at runtime, can be `mut`, type can be inferred.
    // `const` evaluates at compile time, always immutable, type required.
    let runtime_value = std::env::var("HOME").unwrap_or(String::from("/"));
    println!("HOME = {}", runtime_value);   // cannot be a const!

    // ---- const can be used anywhere a literal can ------------------------
    // Including array sizes:
    let buffer = [0u8; MAX_USERS as usize];   // array of 100 zeros
    println!("Buffer length: {}", buffer.len());

    // ---- Shadowing a const with a let is allowed -------------------------
    let max_users: u32 = 50;
    println!("Local shadowed max_users = {}", max_users);

    // ---- Compile-time computations with const ----------------------------
    const SECONDS_PER_HOUR: u32 = 60 * 60;
    const SECONDS_PER_DAY: u32 = SECONDS_PER_HOUR * 24;
    println!("Seconds in a day: {}", SECONDS_PER_DAY);
}

// ---- Constants in functions ---------------------------------------------
fn area_of_circle(radius: f64) -> f64 {
    // Constants are visible throughout their scope:
    PI * radius * radius
}
