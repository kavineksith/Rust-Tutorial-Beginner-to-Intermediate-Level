// ============================================================================
//  CHAPTER 42 — RUST TYPE CONVERSIONS
//  Rust is strict about types — no implicit conversions. Use:
//    * `as`             — primitive numeric casts
//    * `From` / `Into`  — infallible conversions between types
//    * `TryFrom`/TryInto— fallible conversions
//    * `AsRef` / `AsMut`— cheap reference-to-reference
//    * `Deref` / `DerefMut` — deref coercion
// ============================================================================

use std::convert::{TryFrom, TryInto};
use std::fmt;

// ============================================================================
//  1. `as` — primitive numeric casts
// ============================================================================
//  Only works on primitive numeric types. May truncate or lose precision.
fn primitive_casts() {
    let big: i64 = 1_000_000;
    let small: i32 = big as i32;          // may truncate on overflow
    let float: f64 = 3.99;
    let int_part: i32 = float as i32;     // truncates toward zero -> 3

    // Pointer casts (rare; usually you want From/Into):
    let bytes: [u8; 4] = [0, 0, 0, 1];
    let _num: u32 = u32::from_be_bytes(bytes);   // preferred way

    println!("  big as i32 = {}, float as i32 = {}", small, int_part);
}

// ============================================================================
//  2. `From` — infallible conversion (you implement From, get Into for free)
// ============================================================================
#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// `Into` is auto-implemented: anything that implements `From<T>` also
// implements `Into<T>`. You rarely implement `Into` directly.
fn from_into_demo() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.into();           // uses Into (auto from From)
    let f2 = Fahrenheit::from(Celsius(0.0)); // explicit From
    println!("  {:?} = {:?}, {:?}", Celsius(100.0), f, f2);
}

// From<String> for a custom type:
#[derive(Debug)]
struct UserId(String);

impl From<String> for UserId {
    fn from(s: String) -> Self { UserId(s) }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self { UserId(s.to_string()) }
}

// ============================================================================
//  3. `TryFrom` — fallible conversion (returns Result)
// ============================================================================
#[derive(Debug, PartialEq)]
struct PositiveInt(u32);

impl TryFrom<i32> for PositiveInt {
    type Error = &'static str;       // the error type

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err("value must be non-negative")
        } else {
            Ok(PositiveInt(value as u32))
        }
    }
}

fn try_from_demo() {
    let good: Result<PositiveInt, _> = PositiveInt::try_from(5);
    let bad: Result<PositiveInt, _> = PositiveInt::try_from(-1);
    println!("  try_from(5)  = {:?}", good);
    println!("  try_from(-1) = {:?}", bad);

    // `try_into()` is the auto-derived counterpart (note the turbofish):
    let n: Result<PositiveInt, _> = 10i32.try_into();
    println!("  10i32.try_into() = {:?}", n);
}

// ============================================================================
//  4. `AsRef` — cheap reference-to-reference conversion
// ============================================================================
//  When a function accepts `impl AsRef<str>`, it works with `&str`,
//  `String`, `&String`, `Path`, `&Path`, etc. Very flexible.
fn print_length<T: AsRef<str>>(s: T) {
    println!("  length of {:?} = {}", s.as_ref(), s.as_ref().len());
}

// AsRef for our own type:
struct Wrapper(String);
impl AsRef<str> for Wrapper {
    fn as_ref(&self) -> &str { &self.0 }
}

// ============================================================================
//  5. `AsMut` — mutable reference-to-reference
// ============================================================================
//  Same idea as AsRef but for `&mut T`. Lets you abstract over types that
//  expose a mutable view.

// ============================================================================
//  6. `Deref` / `DerefMut` — deref coercion
// ============================================================================
//  Lets you call methods on `T` through a smart pointer that wraps it.
//  Used by Box, Rc, Arc, String, Vec — and you can implement it for your
//  own wrapper types ("newtype" pattern).
use std::ops::Deref;

struct MyString(String);

impl Deref for MyString {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

// Because of Deref, &MyString coerces to &String, which coerces to &str.
// So you can call any &str method on a MyString.

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
//  7. `String::from`, `to_string`, `parse` — common string conversions
// ============================================================================
fn string_conversions() {
    // &str -> String (four equivalent ways)
    let s1: String = "hello".to_string();
    let _s2: String = String::from("hello");
    let _s3: String = "hello".to_owned();
    let _s4: String = format!("{} {}", "hello", "world");

    // String -> &str (free — just take a reference)
    let borrowed: &str = &s1;

    // &str -> integer (fallible)
    let n: i32 = "42".parse().unwrap();
    let n2: Result<i32, _> = "abc".parse();

    // integer -> String
    let n_str: String = 42.to_string();

    // String -> bytes (borrowed view)
    let bytes: &[u8] = s1.as_bytes();

    println!("  s1={}, borrowed={}, n={}, n_str={}, bytes={:?}",
             s1, borrowed, n, n_str, bytes);
    println!("  parse 'abc': {:?}", n2);

    // String -> Vec<u8> (consumes the String):
    let owned_bytes: Vec<u8> = s1.into_bytes();
    println!("  owned_bytes length: {}", owned_bytes.len());
}

fn main() {
    println!("=== 1. Primitive casts (`as`) ===");
    primitive_casts();

    println!("\n=== 2. From / Into ===");
    from_into_demo();

    let uid1: UserId = "ada".into();           // via From<&str>
    let uid2: UserId = String::from("lovelace").into();  // via From<String>
    println!("  uid1 = {:?}, uid2 = {:?}", uid1, uid2);

    println!("\n=== 3. TryFrom / TryInto ===");
    try_from_demo();

    println!("\n=== 4. AsRef ===");
    print_length("literal");                    // &str
    print_length(String::from("owned"));        // String
    print_length(&String::from("borrowed"));    // &String
    print_length(Wrapper("wrapped".to_string())); // Wrapper (AsRef<str>)

    println!("\n=== 5. Deref coercion ===");
    let m = MyString(String::from("via deref"));
    // .len() works because MyString derefs to String, which derefs to str:
    println!("  m.len() = {}, m = {}", m.len(), m);
    // Methods that need &str also work:
    let upper = m.to_uppercase();
    println!("  upper = {}", upper);

    println!("\n=== 6. String conversions ===");
    string_conversions();

    // ========================================================================
    //  CONVERSION CHEAT SHEET
    // ========================================================================
    //  Goal                          | Use
    //  ------------------------------+--------------------------------------
    //  i32 -> u8 etc.                | `as` (may truncate)
    //  T -> U (never fails)          | impl From<T> for U; then `.into()`
    //  T -> U (may fail)             | impl TryFrom<T>; then `.try_into()`
    //  &T -> &U (cheap view)         | impl AsRef<U>; then `.as_ref()`
    //  &mut T -> &mut U              | impl AsMut<U>; then `.as_mut()`
    //  Smart ptr -> &inner           | impl Deref (auto for Box/Rc/Arc/...)
    //  &str -> String                | `.to_string()`, `String::from(...)`, `.to_owned()`
    //  String -> &str                | `&s` (free)
    //  &str -> i32 / f64 / etc.      | `.parse::<T>()` (returns Result)
    //  i32 / f64 -> String           | `.to_string()`, `format!(...)`
    //  String -> Vec<u8>             | `.into_bytes()`
    //  &[u8] -> String               | `String::from_utf8(b)?` (fallible)
    //  T -> String (custom)          | impl ToString for T (or Display + `.to_string()`)
    // ========================================================================
}
