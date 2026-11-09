// ============================================================================
//  CHAPTER 51 — RUST IDIOMATIC PATTERNS & BEST PRACTICES
//  The capstone: how to write Rust like a Rustacean. Each example shows
//  the NON-idiomatic way followed by the IDIOMATIC way.
// ============================================================================

fn main() {
    println!("=== Idiomatic Rust & Best Practices ===\n");

    // ======================================================================
    //  1. Use iterators instead of explicit loops
    // ======================================================================
    let nums = vec![1, 2, 3, 4, 5];

    // ❌ Non-idiomatic: imperative loop
    let mut sum = 0;
    for n in &nums {
        if n % 2 == 0 {
            sum += n * n;
        }
    }
    println!("1. (imperative) sum of even squares = {}", sum);

    // ✅ Idiomatic: iterator chain
    let sum: i32 = nums.iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("1. (iterator)  sum of even squares = {}", sum);

    // ======================================================================
    //  2. Use `?` instead of match-on-Result
    // ======================================================================
    // ❌ Non-idiomatic
    fn parse_doubling_bad(s: &str) -> Result<i32, String> {
        let n = match s.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e.to_string()),
        };
        Ok(n * 2)
    }

    // ✅ Idiomatic
    fn parse_doubling(s: &str) -> Result<i32, String> {
        let n: i32 = s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok(n * 2)
    }

    println!("\n2. parse_doubling(\"21\") = {:?}", parse_doubling("21"));

    // ======================================================================
    //  3. Use `if let` for single-arm matches
    // ======================================================================
    let maybe: Option<i32> = Some(5);

    // ❌ Non-idiomatic
    match maybe {
        Some(n) => println!("\n3. (match) got {}", n),
        _ => {}
    }

    // ✅ Idiomatic
    if let Some(n) = maybe {
        println!("3. (if let) got {}", n);
    }

    // ======================================================================
    //  4. Builder pattern for complex construction
    // ======================================================================
    // ✅ Idiomatic: a builder with chainable methods and a `build()` method.
    let request = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .header("Accept", "application/json")
        .timeout_ms(5000)
        .build();
    println!("\n4. request: {:?}", request);

    // ======================================================================
    //  5. Newtype pattern: wrap a primitive to give it a distinct type
    // ======================================================================
    // ✅ Idiomatic: prevents mixing up two values with the same primitive type.
    let d = Meters(5.0);
    let t = Seconds(2.0);
    println!("\n5. speed = {} m/s", d.0 / t.0);
    // let _bad = d + t;   // ERROR: can't add Meters and Seconds — type-safe!

    // ======================================================================
    //  6. Use `String` and `&str` correctly
    // ======================================================================
    // ❌ Non-idiomatic: taking String when you only need to read
    fn greet_owned(name: String) -> String {
        format!("Hello, {}!", name)
    }

    // ✅ Idiomatic: take &str (more flexible — accepts &String, &str, &&str, ...)
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    println!("\n6. greet(\"Alice\") = {}", greet("Alice"));
    let owned = String::from("Bob");
    println!("6. greet(owned) = {}", greet(&owned));
    let _also_works = greet_owned(owned);   // consumes owned

    // ======================================================================
    //  7. Don't unwrap in production code
    // ======================================================================
    // ❌ Non-idiomatic: panics on Err / None
    // let n: i32 = "abc".parse().unwrap();

    // ✅ Idiomatic: handle the error, or use `?` to propagate
    let n: i32 = "42".parse().unwrap_or(0);
    let n2: Result<i32, _> = "abc".parse();
    println!("\n7. parsed with fallback: {}, parse result: {:?}", n, n2);

    // ======================================================================
    //  8. Use `derive` for boilerplate traits
    // ======================================================================
    // ✅ Idiomatic
    // #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    // struct User { /* ... */ }

    // ======================================================================
    //  9. Return `impl Trait` for opaque return types
    // ======================================================================
    // ✅ Idiomatic: hide the concrete iterator type
    fn evens(up_to: u32) -> impl Iterator<Item = u32> {
        (0..up_to).filter(|x| x % 2 == 0)
    }
    println!("\n9. evens(10) = {:?}", evens(10).collect::<Vec<_>>());

    // ======================================================================
    //  10. Use `clippy` — it's the Rust linter
    // ======================================================================
    // Run `cargo clippy` regularly. It catches non-idiomatic code:
    //
    // ❌ Non-idiomatic:
    //   if x == true { ... }
    // ✅ Idiomatic:
    //   if x { ... }
    //
    // ❌ Non-idiomatic:
    //   let v = vec![];
    //   for i in 0..v.len() { println!("{}", v[i]); }
    // ✅ Idiomatic:
    //   for x in &v { println!("{}", x); }

    // ======================================================================
    //  11. Module organization: one concept per file
    // ======================================================================
    //  src/
    //    main.rs         # binary entry point
    //    lib.rs          # library entry point (if any)
    //    config.rs       # config loading
    //    network/
    //      mod.rs        # `pub mod client; pub mod server;`
    //      client.rs
    //      server.rs
    //    storage/
    //      mod.rs
    //      sqlite.rs

    // ======================================================================
    //  12. Error handling: libraries return errors, applications log them
    // ======================================================================
    // ✅ Library code: return Result, don't log
    // ✅ Application code: log the error at the top, then exit

    // ======================================================================
    //  13. Test in the same file as the code
    // ======================================================================
    // ✅ Idiomatic
    //   pub fn add(a: i32, b: i32) -> i32 { a + b }
    //
    //   #[cfg(test)]
    //   mod tests {
    //       use super::*;
    //       #[test]
    //       fn test_add() { assert_eq!(add(2, 3), 5); }
    //   }

    // ======================================================================
    //  14. Use `cargo fmt` to format your code
    // ======================================================================
    // ✅ Idiomatic: don't argue about style — let `rustfmt` do it.

    // ======================================================================
    //  15. Document public APIs with `///`
    // ======================================================================
    // ✅ Idiomatic:
    //   /// Adds two integers.
    //   ///
    //   /// # Examples
    //   ///
    //   /// ```
    //   /// let n = my_crate::add(2, 3);
    //   /// assert_eq!(n, 5);
    //   /// ```
    //   pub fn add(a: i32, b: i32) -> i32 { a + b }

    // ======================================================================
    //  16. Avoid `clone` when you can borrow
    // ======================================================================
    let names = vec![String::from("Alice"), String::from("Bob")];

    // ❌ Non-idiomatic: clones everything
    let _cloned: Vec<String> = names.iter().cloned().collect();

    // ✅ Idiomatic: borrow when you only need to read
    let borrowed: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    println!("\n16. borrowed: {:?}", borrowed);

    // ======================================================================
    //  17. Use `Box<dyn Trait>` only when necessary
    // ======================================================================
    // Prefer generics `<T: Trait>` (static dispatch, faster) over
    // `Box<dyn Trait>` (dynamic dispatch, slight overhead), UNLESS you
    // need heterogeneous collections.

    // ======================================================================
    //  18. The `?` operator — but only inside functions returning Result
    // ======================================================================

    // ======================================================================
    //  19. Use `anyhow` for application errors, `thiserror` for library errors
    // ======================================================================
    // ✅ Application (binary crate):
    //   use anyhow::{Context, Result};
    //   fn main() -> Result<()> {
    //       let content = std::fs::read_to_string("config")
    //           .context("failed to read config")?;
    //       Ok(())
    //   }
    //
    // ✅ Library:
    //   use thiserror::Error;
    //   #[derive(Error, Debug)]
    //   pub enum MyError {
    //       #[error("io error: {0}")]
    //       Io(#[from] std::io::Error),
    //       #[error("parse error: {0}")]
    //       Parse(#[from] std::num::ParseIntError),
    //   }

    // ======================================================================
    //  20. Common crates you should know
    // ======================================================================
    println!("\n20. Common crates to learn:");
    println!("   serde       - serialization");
    println!("   tokio       - async runtime");
    println!("   clap        - CLI argument parsing");
    println!("   anyhow      - application error handling");
    println!("   thiserror   - library error types");
    println!("   log         - logging facade");
    println!("   tracing     - structured logging");
    println!("   rand        - random numbers");
    println!("   chrono      - date/time");
    println!("   regex       - regular expressions");
    println!("   reqwest     - HTTP client");
    println!("   axum/actix  - web frameworks");
    println!("   sqlx        - async SQL");
    println!("   rayon       - data parallelism");
    println!("   crossbeam   - low-level concurrency");
    println!("   once_cell   - global singletons");
    println!("   itertools   - extra iterator helpers");
    println!("   lazy_static - lazy globals (older)");

    // ========================================================================
    //  FINAL CHECKLIST — write idiomatic Rust
    // ========================================================================
    //
    //  □ Run `cargo fmt` before every commit.
    //  □ Run `cargo clippy` and fix all warnings.
    //  □ No `unwrap()` / `expect()` in production paths (except where
    //    failure is truly impossible).
    //  □ Use `?` for error propagation.
    //  □ Use iterators instead of explicit loops.
    //  □ Use `&str` instead of `&String` in function parameters.
    //  □ Use `&[T]` instead of `&Vec<T>` in function parameters.
    //  □ Derive `Debug` for all public types.
    //  □ Document public items with `///`.
    //  □ Write at least one unit test per public function.
    //  □ Write doctests for examples.
    //  □ Use `anyhow` for apps, `thiserror` for libraries.
    //  □ Prefer generics over `Box<dyn Trait>` when possible.
    //  □ Don't clone when you can borrow.
    //  □ Keep `unsafe` blocks small and well-documented.
    //  □ Use `#[cfg(test)] mod tests` for unit tests.
    //  □ Put integration tests in `tests/`.
    //  □ Keep public API small — most things should be private.
    //  □ Use the newtype pattern to give primitives distinct types.
    //  □ Don't put logging in libraries — return errors instead.
    //  □ Use `Arc<Mutex<T>>` for multi-thread shared mutable state.
    //  □ Use `tokio` for async I/O; threads for CPU-bound work.
    // ========================================================================
    println!("\nYou are now equipped to write idiomatic Rust! 🦀");
}

// ========================================================================
//  Helper types for the examples above
// ========================================================================

#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    timeout_ms: u64,
}

#[derive(Default)]
struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    timeout_ms: Option<u64>,
}

impl RequestBuilder {
    fn new() -> Self { Self::default() }

    fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    fn build(self) -> Request {
        Request {
            url: self.url.unwrap_or_default(),
            method: self.method.unwrap_or_else(|| "GET".to_string()),
            headers: self.headers,
            timeout_ms: self.timeout_ms.unwrap_or(30000),
        }
    }
}

// Newtype pattern — distinct types for distinct quantities:
#[derive(Debug, Clone, Copy)]
struct Meters(f64);

#[derive(Debug, Clone, Copy)]
struct Seconds(f64);

// Adding Meters + Seconds would be a type error:
// impl std::ops::Add<Seconds> for Meters { ... }  // you'd have to write it explicitly
