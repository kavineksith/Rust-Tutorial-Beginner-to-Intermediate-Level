// ============================================================================
//  CHAPTER 43 — RUST COMMON TRAITS (Copy, Clone, Drop, Default, ...)
//  A deep dive into the most-derivable standard traits — what they mean,
//  when to derive them, and when to implement them manually.
// ============================================================================

use std::fmt;

// ============================================================================
//  1. Clone — explicit deep copy
// ============================================================================
//  `Clone` says: "you can make a deep copy of me by calling .clone()."
//  Almost every type can derive it. The only types that CAN'T are those
//  that hold non-Clone fields (e.g. a type wrapping a `MutexGuard`).
#[derive(Clone, Debug)]
struct Point { x: i32, y: i32 }

// Manual Clone impl (equivalent to the derive):
impl Clone for PointManual {
    fn clone(&self) -> Self {
        PointManual { x: self.x, y: self.y }
    }
}
#[derive(Debug)]
struct PointManual { x: i32, y: i32 }

// ============================================================================
//  2. Copy — implicit bitwise copy on assignment
// ============================================================================
//  `Copy` is a marker trait: it has no methods. It says "this type is so
//  cheap to copy that the compiler will do it implicitly on every read."
//
//  Rules:
//    * The type must be small (no heap allocation, no shared resources).
//    * All fields must themselves be Copy.
//    * If you implement Copy, you MUST also implement Clone (the compiler
//      requires it). Usually you just `#[derive(Copy, Clone)]`.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Color { r: u8, g: u8, b: u8 }   // 3 bytes — totally fine to Copy

fn copy_demo() {
    let c1 = Color { r: 255, g: 128, b: 0 };
    let c2 = c1;    // IMPLICITLY COPIED (no .clone() needed)
    // Both c1 and c2 are usable:
    println!("  c1 = {:?}, c2 = {:?}", c1, c2);
}

// =====================================================================
//  When NOT to derive Copy:
//    * String  — owns heap memory; copying would duplicate the allocation.
//    * Vec<T>  — same reason.
//    * Box<T>  — represents unique ownership of heap memory.
//    * A type containing any of the above.
//  For these, derive only Clone — callers must explicitly `.clone()`.
// =====================================================================

// ============================================================================
//  3. Drop — the destructor (cleanup code)
// ============================================================================
//  `Drop` runs automatically when a value goes out of scope. It's how Rust
//  frees memory, closes files, releases locks, etc. — without `free()`.
//
//  You rarely implement Drop manually. Common reasons:
//    * Custom resource cleanup (FFI types, file handles).
//    * Logging when something is dropped (debugging).
//    * Invariants that must be checked at drop time.
struct LogOnDrop { name: &'static str }
impl Drop for LogOnDrop {
    fn drop(&mut self) {
        println!("  -> dropping {}", self.name);
    }
}

fn drop_demo() {
    let _a = LogOnDrop { name: "a" };
    {
        let _b = LogOnDrop { name: "b" };
        println!("  inside inner block");
    }   // <- b is dropped here
    println!("  after inner block");
}   // <- a is dropped here, after the function returns

// ---- Early drop with std::mem::drop ----------------------------------
// Sometimes you want to drop a value BEFORE it goes out of scope:
fn early_drop() {
    let file = LogOnDrop { name: "file" };
    println!("  before explicit drop");
    drop(file);                       // drop now
    println!("  after explicit drop");
}

// ============================================================================
//  4. Default — a sensible zero/empty value
// ============================================================================
//  `Default::default()` returns a "natural" empty value:
//    * Numbers -> 0
//    * Strings -> ""
//    * Vec / HashMap -> empty
//    * bool -> false
//    * Option<T> -> None
//  Useful for config structs, partial initialization, test fixtures.
#[derive(Debug, Default)]
struct Config {
    host: String,           // default: ""
    port: u16,              // default: 0
    retries: u32,           // default: 0
    debug: bool,            // default: false
}

fn default_demo() {
    let c = Config::default();
    println!("  default config = {:?}", c);

    // Struct update syntax with ..Default::default() — partial init:
    let custom = Config {
        host: String::from("localhost"),
        port: 8080,
        ..Default::default()
    };
    println!("  custom config = {:?}", custom);
}

// ============================================================================
//  5. PartialEq / Eq — equality comparison
// ============================================================================
//  PartialEq gives you `==` and `!=`. Eq is a marker trait that says
//  "equality is reflexive, symmetric, transitive" (i.e. true equality).
//
//  Floating point types implement PartialEq but NOT Eq, because NaN != NaN.
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

// ============================================================================
//  6. PartialOrd / Ord — ordering comparison
// ============================================================================
//  PartialOrd gives you `<`, `>`, `<=`, `>=`. Ord is a marker that says
//  "every two values are comparable" (total order).
//
//  Floating point types implement PartialOrd but NOT Ord (NaN breaks totality).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u32, u32, u32);   // major.minor.patch

fn ord_demo() {
    let v1 = Version(1, 0, 0);
    let v2 = Version(1, 2, 0);
    let v3 = Version(2, 0, 0);
    println!("  v1 < v2? {}", v1 < v2);
    println!("  v2 < v3? {}", v2 < v3);

    // Because Version: Ord, it can be used in BTreeMap, sorted, etc.
    let mut versions = vec![v3, v1, v2];
    versions.sort();
    println!("  sorted: {:?}", versions);
}

// ============================================================================
//  7. Hash — use as a HashMap key
// ============================================================================
//  Required for HashMap / HashSet keys. Just derive it for most types.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Email(String);

// ============================================================================
//  8. Display — user-facing string representation
// ============================================================================
//  `Display` lets your type print with `{}` (vs `{:?}` for Debug).
//  Cannot be derived — you must implement it manually.
struct Kelvin(f64);
impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The Formatter supports precision, width, alignment flags:
        if let Some(precision) = f.precision() {
            write!(f, "{:.*} K", precision, self.0)
        } else {
            write!(f, "{:.2} K", self.0)
        }
    }
}

// ============================================================================
//  9. Debug — developer-facing representation
// ============================================================================
//  Derived with `#[derive(Debug)]`. Prints with `{:?}` (single-line) or
//  `{:#?}` (pretty multi-line).
//  For types where Debug matters, you can also implement it manually for
//  a cleaner output:
impl fmt::Debug for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "K({})", self.0)
    }
}

fn main() {
    println!("=== 1. Copy ===");
    copy_demo();

    println!("\n=== 2. Drop ===");
    drop_demo();

    println!("\n=== 3. Early drop ===");
    early_drop();

    println!("\n=== 4. Default ===");
    default_demo();

    println!("\n=== 5. Ord ===");
    ord_demo();

    println!("\n=== 6. Display / Debug ===");
    let k = Kelvin(273.15);
    println!("  Display (default): {}", k);
    println!("  Display (precision 4): {:.4}", k);
    println!("  Debug: {:?}", k);

    // ========================================================================
    //  DERIVABLE TRAITS — what to derive by default
    // ========================================================================
    //  For a typical data struct:
    //
    //    #[derive(Debug, Clone, PartialEq)]
    //    struct MyType { /* ... */ }
    //
    //  Add Eq if your type is fully orderable (no NaN-like values).
    //  Add Hash if you want to use it as a HashMap key.
    //  Add Copy only if it's a small, all-Copy-able type.
    //  Add Default if there's a natural "empty" value.
    //  Add PartialOrd/Ord if you want to sort it.
    //
    //  Never derive Display — implement it manually.
    // ========================================================================
}
