// ============================================================================
//  CHAPTER 31 — RUST ERROR HANDLING
//  Rust has no exceptions. Errors are VALUES — represented by the Result
//  type. Recoverable errors use Result; unrecoverable errors use panic!.
// ============================================================================

use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

// ============================================================================
//  1. UNRECOVERABLE ERRORS: panic!
// ============================================================================
//  Use panic! when the program is in an unrecoverable state — a bug, a
//  violated invariant, an impossible situation.
fn demonstrate_panic() {
    // panic!("something went terribly wrong");   // uncomment to see the stack unwind
    let _v = vec![1, 2, 3];
    // let _x = _v[10];   // PANIC at runtime: index out of bounds
    println!("no panic here — keep going");
}

// ============================================================================
//  2. RECOVERABLE ERRORS: Result<T, E>
// ============================================================================
//  Use Result for operations that can fail in expected ways (file I/O,
//  parsing, network requests).
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// ============================================================================
//  3. The `?` operator — propagates errors up the call stack
// ============================================================================
//  Inside a function returning Result, `?` returns early on Err.
//  It is sugar for:
//      match expr {
//          Ok(v) => v,
//          Err(e) => return Err(e.into()),
//      }
fn read_first_number(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;   // may fail
    let trimmed = content.trim();
    let n: i32 = trimmed.parse()?;                  // may fail
    Ok(n)
}

// ============================================================================
//  4. Custom error type
// ============================================================================
//  For libraries, define your own error enum and implement Display + Error.
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
    Empty,
}

// Implement Display so users can print the error nicely:
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e)    => write!(f, "I/O error: {}", e),
            AppError::Parse(e) => write!(f, "parse error: {}", e),
            AppError::Empty    => write!(f, "input was empty"),
        }
    }
}

// Implement the Error trait. The default impls are usually enough.
impl Error for AppError {
    // `source` lets users walk the cause chain:
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e)    => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::Empty    => None,
        }
    }
}

// Convert from underlying errors with From impls — this lets `?` work
// automatically when the function returns AppError:
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

// Now `?` works for both io and parse errors:
fn read_number_strict(filename: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(filename)?;       // io::Error -> AppError
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(AppError::Empty);
    }
    let n: i32 = trimmed.parse()?;                     // ParseIntError -> AppError
    Ok(n)
}

// ============================================================================
//  5. Convenient helpers on Result
// ============================================================================
fn use_helpers() {
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("nope");

    println!("ok.unwrap() = {}", ok.unwrap());              // panics if Err
    println!("err.unwrap_or(0) = {}", err.unwrap_or(0));    // 0
    println!("err.unwrap_or_default() = {}", err.unwrap_or_default()); // 0 (i32::default)
    println!("err.unwrap_or_else(|_| 99) = {}", err.unwrap_or_else(|_| 99)); // 99

    // map and map_err transform Ok or Err values:
    let doubled = ok.map(|x| x * 2);             // Ok(10)
    let mapped_err = err.map_err(|e| e.to_uppercase()); // Err("NOPE")

    // is_ok / is_err — boolean checks
    println!("ok.is_ok() = {}, err.is_err() = {}", ok.is_ok(), err.is_err());

    // Convert Result -> Option (drops the error):
    let opt = ok.ok();        // Some(5)
    let none = err.ok();      // None
    println!("opt = {:?}, none = {:?}", opt, none);

    let _ = (doubled, mapped_err);
}

// ============================================================================
//  MAIN
// ============================================================================
fn main() {
    println!("=== 1. panic! ===");
    demonstrate_panic();

    println!("\n=== 2. Result ===");
    match parse_number("42") {
        Ok(n)  => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }
    match parse_number("not a number") {
        Ok(n)  => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }

    println!("\n=== 3. The `?` operator ===");
    // Use a file that doesn't exist — see the error propagate.
    match read_first_number("/tmp/does_not_exist_12345.txt") {
        Ok(n)  => println!("read number: {}", n),
        Err(e) => println!("error reading file: {}", e),
    }
    // And a file we create on the fly:
    std::fs::write("/tmp/rust_demo_12345.txt", "123").unwrap();
    match read_first_number("/tmp/rust_demo_12345.txt") {
        Ok(n)  => println!("read number: {}", n),
        Err(e) => println!("error: {}", e),
    }

    println!("\n=== 4. Custom error type ===");
    match read_number_strict("/tmp/rust_demo_12345.txt") {
        Ok(n)  => println!("strict: parsed {}", n),
        Err(e) => println!("strict error: {} (source: {:?})", e, e.source()),
    }
    std::fs::write("/tmp/rust_demo_empty.txt", "   ").unwrap();
    match read_number_strict("/tmp/rust_demo_empty.txt") {
        Ok(_)  => println!("should not happen"),
        Err(e) => println!("strict error: {}", e),
    }

    println!("\n=== 5. Result helpers ===");
    use_helpers();

    // ========================================================================
    //  SUMMARY — when to use what
    // ========================================================================
    //  * panic!              : bugs, impossible states, violated invariants
    //  * Result<T, E>        : expected failures (I/O, parsing, network)
    //  * `?` operator        : propagate errors up the call chain
    //  * Option<T>           : "maybe nothing" (not really an error)
    //  * Custom Error enum   : libraries with multiple failure modes
    //  * Box<dyn Error>      : quick prototyping, any-error-goes
    //  * thiserror crate     : ergonomic custom errors (real projects)
    //  * anyhow crate        : application-level error chaining
    // ========================================================================
}
