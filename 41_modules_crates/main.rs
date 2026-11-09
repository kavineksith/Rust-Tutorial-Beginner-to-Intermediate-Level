// ============================================================================
//  CHAPTER 32 — RUST MODULES & CRATES
//  How to organize code as it grows: modules, paths, `use`, `pub`, crates.
// ============================================================================

// ============================================================================
//  Inline modules — declare a module right inside the file
// ============================================================================
mod geometry {
    // By default, items inside a module are PRIVATE — not visible outside.
    // Use `pub` to make them visible.
    pub const PI: f64 = 3.141592653589793;

    pub struct Circle {
        pub radius: f64,           // public field
        _label: String,            // private field (underscore = intentionally unused)
    }

    impl Circle {
        // Public constructor — users outside the module can call this.
        pub fn new(radius: f64) -> Self {
            Circle { radius, _label: String::from("circle") }
        }

        pub fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }

    // A private helper — only visible inside `geometry`:
    fn _degrees_to_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}

// ============================================================================
//  Nested modules — modules can contain modules
// ============================================================================
mod math {
    pub mod arithmetic {
        pub fn add(a: f64, b: f64) -> f64 { a + b }
        pub fn sub(a: f64, b: f64) -> f64 { a - b }
    }

    pub mod trigonometry {
        pub fn sin_deg(deg: f64) -> f64 {
            deg.to_radians().sin()
        }
    }
}

// ============================================================================
//  `use` — bring items into scope so you don't have to write full paths
// ============================================================================
// Without `use`:
//   let c = geometry::Circle::new(2.0);
//
// With `use`:
use geometry::Circle;
// Now we can just write Circle::new(2.0)

// Bring a module into scope (then access its items via `::`):
use math::arithmetic;

// Bring a deeply nested item directly:
use math::trigonometry::sin_deg;

// Re-export with `pub use` (used by library crates to expose a clean API):
pub mod re_exported {
    pub use crate::math::arithmetic::add as add_numbers;
}

// ============================================================================
//  Glob imports — bring everything from a module (use sparingly)
// ============================================================================
// use math::arithmetic::*;   // would import add, sub, ...

// ============================================================================
//  Modules can also be defined in SEPARATE FILES
// ============================================================================
//  If you write `mod network;` (no body) in main.rs, Rust looks for the
//  module's content in:
//    * `network.rs`
//    * `network/mod.rs`
//
//  Example file layout for a real project:
//
//    src/
//      main.rs              // contains `mod network;` and `mod utils;`
//      network.rs           // contains `pub fn connect(...) { }`
//      utils/
//        mod.rs             // contains `pub mod format;`
//        format.rs          // contains `pub fn pretty_print(...) { }`
//
//  In main.rs:
//    mod network;
//    mod utils;
//
//  Then access via: `network::connect()` and `utils::format::pretty_print()`
//
//  We keep everything inline here so a single `rustc` command compiles it.

// ============================================================================
//  The `crate` keyword refers to the root of the current crate
// ============================================================================
// Use `crate::` for absolute paths from the crate root:
//   crate::math::arithmetic::add(...)
//
// Use `self::` for paths relative to the current module.
// Use `super::` for paths relative to the parent module.

mod outer {
    pub fn hello() {
        println!("hello from outer");
    }

    pub mod inner {
        pub fn call_parent() {
            // `super::` reaches up to the parent module:
            super::hello();
        }
    }
}

// ============================================================================
//  A small "library" module with public API
// ============================================================================
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn mul(a: i32, b: i32) -> i32 { a * b }

    // Private function — only used internally:
    fn _validate(x: i32) -> bool { x >= 0 }

    /// Public re-export — keeps the API flat for users
    pub use self::ops::power;
    mod ops {
        pub fn power(base: i32, exp: u32) -> i32 {
            (1..=exp).fold(1, |acc, _| acc * base)
        }
    }
}

fn main() {
    // ---- Using items from the geometry module ---------------------------
    let c = Circle::new(2.0);
    println!("circle area = {:.2}", c.area());
    println!("PI = {}", geometry::PI);

    // ---- Using nested modules -------------------------------------------
    println!("add(2, 3) = {}", arithmetic::add(2.0, 3.0));
    println!("sub(2, 3) = {}", math::arithmetic::sub(2.0, 3.0));
    println!("sin(30deg) = {:.4}", sin_deg(30.0));
    println!("sin(90deg) = {:.4}", math::trigonometry::sin_deg(90.0));

    // ---- Re-exported item -----------------------------------------------
    println!("2^10 = {}", re_exported::add_numbers(2.0, 3.0));

    // ---- super:: navigation ---------------------------------------------
    outer::inner::call_parent();   // prints "hello from outer"

    // ---- Calculator library ---------------------------------------------
    println!("calc add(5, 7) = {}", calculator::add(5, 7));
    println!("calc mul(5, 7) = {}", calculator::mul(5, 7));
    println!("calc power(2, 10) = {}", calculator::power(2, 10));

    // ========================================================================
    //  CRATES — the compilation unit
    // ========================================================================
    //  * A CRATE is what `cargo` builds. It's either a binary or a library.
    //  * A binary crate has a `fn main()` (or `src/main.rs`).
    //  * A library crate has a `lib.rs` (no main).
    //  * Crates are made up of MODULES (what we just used above).
    //
    //  Cargo.toml example for a binary project:
    //
    //    [package]
    //    name = "my_app"
    //    version = "0.1.0"
    //    edition = "2024"
    //
    //    [dependencies]
    //    serde = { version = "1.0", features = ["derive"] }
    //    tokio = { version = "1", features = ["full"] }
    //
    //  Adding a dependency:
    //    cargo add serde           # adds the latest version
    //
    //  Using a dependency in code:
    //    use serde::Serialize;
    //
    //  The crate root is `src/main.rs` (binary) or `src/lib.rs` (library).
    // ========================================================================
}
