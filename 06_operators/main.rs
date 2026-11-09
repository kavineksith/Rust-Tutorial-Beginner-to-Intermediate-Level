// ============================================================================
//  CHAPTER 09 — RUST OPERATORS
//  Arithmetic, comparison, logical, bitwise, assignment, and range.
// ============================================================================

fn main() {
    // ---- Arithmetic operators --------------------------------------------
    let a = 10;
    let b = 3;
    println!("a + b = {}", a + b);    // 13
    println!("a - b = {}", a - b);    // 7
    println!("a * b = {}", a * b);    // 30
    println!("a / b = {}", a / b);    // 3 (integer division truncates)
    println!("a % b = {}", a % b);    // 1 (remainder)

    let f1 = 10.0;
    let f2 = 3.0;
    println!("f1 / f2 = {}", f1 / f2); // 3.3333... (floats keep precision)

    // ---- Unary minus -----------------------------------------------------
    let positive = 5;
    let negative = -positive;
    println!("negative = {}", negative); // -5

    // ---- Compound assignment --------------------------------------------
    let mut x = 10;
    x += 5;  println!("x += 5  -> {}", x);   // 15
    x -= 3;  println!("x -= 3  -> {}", x);   // 12
    x *= 2;  println!("x *= 2  -> {}", x);   // 24
    x /= 4;  println!("x /= 4  -> {}", x);   // 6
    x %= 4;  println!("x %= 4  -> {}", x);   // 2

    // ---- Comparison operators (return bool) -----------------------------
    println!("a == b: {}", a == b);   // false
    println!("a != b: {}", a != b);   // true
    println!("a >  b: {}", a > b);    // true
    println!("a <  b: {}", a < b);    // false
    println!("a >= b: {}", a >= b);   // true
    println!("a <= b: {}", a <= b);   // false

    // ---- Logical operators ----------------------------------------------
    let t = true;
    let f = false;
    println!("t && f: {}", t && f);   // false  (AND)
    println!("t || f: {}", t || f);   // true   (OR)
    println!("!t    : {}", !t);       // false  (NOT)

    // Short-circuit evaluation: && and || only evaluate their right side
    // when needed. This is safe:
    //   let n = Some(5);
    //   if n.is_some() && n.unwrap() > 3 { ... }   // unwrap only runs if Some

    // ---- Bitwise operators ----------------------------------------------
    let m = 0b1100;  // 12
    let n = 0b1010;  // 10
    println!("m & n  = {:b}", m & n);    // 1000 (AND)
    println!("m | n  = {:b}", m | n);    // 1110 (OR)
    println!("m ^ n  = {:b}", m ^ n);    // 0110 (XOR)
    println!("!m     = {:b}", !m);       // all bits flipped
    println!("m << 2 = {:b}", m << 2);   // 110000 (left shift)
    println!("m >> 1 = {:b}", m >> 1);   // 110 (right shift)

    // Bitwise compound assignment also works:
    let mut bits = 0b1100u8;
    bits &= 0b1010;
    println!("bits &= 0b1010 -> {:b}", bits); // 1000

    // ---- Range operators ------------------------------------------------
    // `..`  : exclusive range (start, end]
    // `..=` : inclusive range [start, end]
    let exclusive = 1..5;        // 1, 2, 3, 4
    let inclusive = 1..=5;       // 1, 2, 3, 4, 5
    println!("exclusive: {:?}", exclusive.collect::<Vec<_>>());
    println!("inclusive: {:?}", inclusive.collect::<Vec<_>>());

    // ---- Type cast operator `as` ----------------------------------------
    let big: i64 = 1_000_000;
    let small: i32 = big as i32;
    println!("as cast: {}", small);

    // ---- Reference and dereference operators ----------------------------
    let value = 42;
    let ref_to_value = &value;        // `&` creates a reference
    let deref_value = *ref_to_value;  // `*` dereferences
    println!("ref: {}, deref: {}", ref_to_value, deref_value);
}
