// ============================================================================
//  CHAPTER 11 — RUST IF..ELSE
//  Conditional branching. `if` is an EXPRESSION in Rust, not just a statement.
// ============================================================================

fn main() {
    // ---- Basic if --------------------------------------------------------
    let number = 6;
    if number > 5 {
        println!("number is greater than 5");
    }

    // ---- if / else -------------------------------------------------------
    let age = 17;
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // ---- if / else if / else --------------------------------------------
    let score = 78;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else if score >= 60 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }

    // ---- `if` is an expression ------------------------------------------
    // You can use the result of an `if` directly:
    let n = 7;
    let even_or_odd = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, even_or_odd);

    // IMPORTANT: both arms must have the SAME type. This would not compile:
    // let bad = if true { 1 } else { "two" }; // ERROR: mismatched types

    // ---- Condition must be a bool ---------------------------------------
    // Rust will NOT implicitly convert other types to bool:
    let count = 0;
    // if count { ... }     // ERROR
    if count != 0 {
        println!("non-zero");
    } else {
        println!("zero");
    }

    // ---- Multi-line expressions in arms ---------------------------------
    let temperature = 25;
    let advice = if temperature > 30 {
        let diff = temperature - 30;
        format!("Very hot — {} degrees above 30", diff)
    } else if temperature < 10 {
        String::from("Cold — wear a jacket")
    } else {
        format!("Pleasant: {} degrees", temperature)
    };
    println!("{}", advice);

    // ---- `if let` (preview of pattern matching) -------------------------
    // A more concise way to handle a single match arm.
    let maybe_name: Option<&str> = Some("Ada");
    if let Some(name) = maybe_name {
        println!("Hello, {}!", name);
    } else {
        println!("No name provided.");
    }
}
