// ============================================================================
//  CHAPTER 16 — RUST FUNCTIONS
//  `fn`, parameters, return types, expressions, statements, and diverging.
// ============================================================================

// ---- Basic function: no parameters, no return value ---------------------
fn say_hello() {
    println!("Hello!");
}

// ---- Function with parameters -------------------------------------------
// Parameters MUST have explicit types. The compiler does not infer them.
fn greet(name: &str) {
    println!("Hi, {}!", name);
}

// ---- Function with a return type ----------------------------------------
// `-> i32` declares the return type. The LAST expression (no semicolon)
// in the body is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b          // <-- no semicolon: this is the return value
}

// ---- Early return with `return` keyword ---------------------------------
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;          // early return
    }
    x                       // normal return (no `return` needed)
}

// ---- Function with multiple parameters of different types ---------------
fn build_info(name: &str, age: u32, is_admin: bool) -> String {
    format!("{} ({} years old){}", name, age, if is_admin { " [admin]" } else { "" })
}

// ---- Function returning a tuple ----------------------------------------
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in numbers {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

// ---- Statements vs Expressions in a function body ----------------------
// A statement performs an action and returns `()`.
// An expression evaluates to a value.
// Adding a semicolon to an expression turns it into a statement.
fn area(width: i32, height: i32) -> i32 {
    let product = width * height;   // statement (returns ())
    product                         // expression (returns i32) — no semicolon
}

// ---- Diverging functions: return type `!` ------------------------------
// A function that never returns (loops forever, panics, or exits the process).
fn infinite() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// ---- Function with default-like behaviour via Option -------------------
// Rust does not have default arguments. Use Option<T> instead:
fn greet_optional(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None    => println!("Hello, stranger!"),
    }
}

// ---- The entry point ---------------------------------------------------
fn main() {
    say_hello();
    greet("Ada");
    greet("Linus");

    let sum = add(2, 3);
    println!("2 + 3 = {}", sum);

    println!("|-5| = {}", abs(-5));

    println!("{}", build_info("Ada", 36, true));

    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&nums);
    println!("min={}, max={}", min, max);

    println!("area = {}", area(3, 4));

    greet_optional(Some("Ada"));
    greet_optional(None);

    // ---- Closures (anonymous functions) — preview -----------------------
    // Closures capture their environment. Syntax: |args| body
    let double = |x: i32| x * 2;
    println!("double(5) = {}", double(5));

    let multiplier = 3;
    let multiply = |x| x * multiplier;   // captures `multiplier` from env
    println!("multiply(5) = {}", multiply(5));
}
