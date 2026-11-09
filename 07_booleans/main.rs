// ============================================================================
//  CHAPTER 10 — RUST BOOLEANS
//  The `bool` type — `true` and `false`.
// ============================================================================

fn main() {
    // ---- Declaring booleans ---------------------------------------------
    let t: bool = true;          // explicit type
    let f = false;               // type inferred as bool
    println!("t = {}, f = {}", t, f);

    // ---- Booleans from comparisons --------------------------------------
    let age = 18;
    let is_adult = age >= 18;    // true
    let can_drink = age >= 21;   // false
    println!("is_adult = {}, can_drink = {}", is_adult, can_drink);

    // ---- Logical operations ---------------------------------------------
    let a = true;
    let b = false;
    println!("a && b = {}", a && b);   // false (AND)
    println!("a || b = {}", a || b);   // true  (OR)
    println!("!a     = {}", !a);       // false (NOT)

    // ---- Booleans in conditions -----------------------------------------
    // `if` expects a `bool` — Rust does NOT implicitly convert other types.
    if is_adult {
        println!("You are an adult.");
    }
    if !can_drink {
        println!("But you cannot drink (in the US).");
    }

    // Contrast with C / JavaScript: this would NOT compile in Rust:
    //   let n = 5;
    //   if n { ... }     // ERROR: expected bool, found integer
    // You must be explicit:
    let n = 5;
    if n != 0 {
        println!("n is non-zero (n = {})", n);
    }

    // ---- Booleans as values (in expressions) ----------------------------
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("Status: {}", status);

    // ---- Short-circuit evaluation ---------------------------------------
    // `&&` returns early if the left side is false; `||` if the left is true.
    // Useful for safely checking before using a value:
    let maybe_name: Option<&str> = Some("Ada");
    let long_enough = maybe_name.is_some() && maybe_name.unwrap().len() > 2;
    println!("long_enough = {}", long_enough);

    // ---- `bool` is a single byte in memory ------------------------------
    // `std::mem::size_of::<bool>()` is 1.
    println!("size of bool = {} byte", std::mem::size_of::<bool>());

    // ---- Converting bool to integer -------------------------------------
    let truthy = true as i32;    // 1
    let falsy  = false as i32;   // 0
    println!("truthy = {}, falsy = {}", truthy, falsy);
}
