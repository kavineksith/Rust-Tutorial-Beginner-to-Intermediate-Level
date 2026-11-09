// ============================================================================
//  CHAPTER 07 — RUST DATA TYPES
//  Every value in Rust has a type. This file tours the scalar and compound
//  types built into the language.
// ============================================================================

fn main() {
    // ======================================================================
    //  SCALAR TYPES  (a single value)
    // ======================================================================

    // ---- Integers ---------------------------------------------------------
    // Signed:   i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    //
    // `isize` / `usize` are the size of a pointer on the target machine
    // (64-bit on most modern machines).
    let a: i32 = -42;          // signed 32-bit
    let b: u64 = 100;          // unsigned 64-bit
    let c: isize = -1;         // pointer-sized signed
    println!("a={}, b={}, c={}", a, b, c);

    // Integer literals can have a base prefix and underscores:
    let dec   = 1_000;         // decimal
    let hex   = 0xFF;          // hex
    let oct   = 0o777;         // octal
    let bin   = 0b1010_1010;   // binary
    let byte  = b'A';          // byte (u8)
    println!("dec={}, hex={}, oct={}, bin={}, byte={}", dec, hex, oct, bin, byte);

    // ---- Floating-point ---------------------------------------------------
    // f32 and f64. Default is f64 (similar speed to f32 on modern CPUs but
    // more precision).
    let x: f32 = 3.14;
    let y: f64 = 2.718281828459045;
    println!("x={}, y={}", x, y);

    // ---- Boolean ----------------------------------------------------------
    let t: bool = true;
    let f = false;             // type inferred as bool
    println!("t={}, f={}", t, f);

    // ---- Character --------------------------------------------------------
    // `char` is a 4-byte Unicode scalar value (NOT 1 byte like in C).
    let letter: char = 'R';
    let emoji = '🦀';          // yes, this is a single char
    let zwj = 'A';
    println!("letter={}, emoji={}, zwj={}", letter, emoji, zwj);

    // ======================================================================
    //  COMPOUND TYPES  (multiple values grouped)
    // ======================================================================

    // ---- Tuples -----------------------------------------------------------
    // Fixed-size, mixed types. (Covered in detail in chapter 24.)
    let pair: (i32, f64) = (1, 2.5);
    let (id, score) = pair;                 // destructuring
    println!("id={}, score={}", id, score);
    println!("access by index: {} {}", pair.0, pair.1);

    // ---- Arrays -----------------------------------------------------------
    // Fixed-size, same type. Stored on the stack. (Chapter 22.)
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3];                      // [0, 0, 0]
    println!("nums={:?}, zeros={:?}", nums, zeros);

    // ---- String slices and String ----------------------------------------
    // `&str` is a borrowed string slice (fixed, view into text).
    // `String` is an owned, growable string (chapter 18).
    let slice: &str = "hello";
    let owned: String = String::from("world");
    println!("{} {}", slice, owned);

    // ======================================================================
    //  TYPE INFERENCE
    // ======================================================================
    // Rust infers types wherever it can. The compiler walks the code and
    // figures out the most specific type that satisfies all uses.
    let v = vec![1, 2, 3];    // Vec<i32> — inferred from the literals
    let s = v.iter().sum::<i32>();  // sum as i32
    println!("sum = {}", s);

    // ======================================================================
    //  TYPE CASTING with `as`
    // ======================================================================
    let big: i64 = 1_000_000;
    let small: i32 = big as i32;     // may truncate!
    println!("big={}, small={}", big, small);

    let float: f64 = 3.99;
    let trunc: i32 = float as i32;   // truncates towards zero -> 3
    println!("float={}, trunc={}", float, trunc);
}
