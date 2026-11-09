// ============================================================================
//  CHAPTER 38 — RUST TESTING
//  Rust has built-in testing support: `cargo test` finds every #[test]
//  function and runs it. This file shows both the idiomatic cargo layout
//  AND a self-contained version that runs the same assertions at runtime
//  when compiled with plain `rustc main.rs`.
// ============================================================================

// ============================================================================
//  MODULE 1 — Code under test
// ============================================================================
//  In a real project, your library code lives in src/lib.rs (or submodules).
//  Here we inline it so a single `rustc main.rs` works.
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn sub(a: i32, b: i32) -> i32 { a - b }
    pub fn div(a: i32, b: i32) -> Option<i32> {
        if b == 0 { None } else { Some(a / b) }
    }

    /// Splits a string on whitespace and returns the first word.
    /// Returns `None` if the input is empty or all whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_tutorial::math::first_word;
    /// assert_eq!(first_word("hello world"), Some("hello"));
    /// assert_eq!(first_word(""), None);
    /// ```
    pub fn first_word(s: &str) -> Option<&str> {
        s.split_whitespace().next()
    }
}

// ============================================================================
//  MODULE 2 — The test module
// ============================================================================
//  Convention: put tests inside a `#[cfg(test)] mod tests` block. The
//  `#[cfg(test)]` attribute means "compile this module only when running
//  `cargo test`", so it doesn't bloat your release binary.
#[cfg(test)]
mod tests {
    // Bring the parent module's items into scope:
    use super::math::*;

    // ---- Each test is a function annotated with #[test] -----------------
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(0, 0), 0);
    }

    // ---- Tests can return Result<(), String> for nicer failure messages ----
    #[test]
    fn test_div_ok() -> Result<(), String> {
        let result = div(10, 2).ok_or("division returned None")?;
        if result == 5 {
            Ok(())
        } else {
            Err(format!("expected 5, got {}", result))
        }
    }

    // ---- Testing that something should panic: #[should_panic] ----------
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10];      // panics with "index out of bounds"
    }

    // ---- Testing Option / Result with the `?` operator ------------------
    #[test]
    fn test_first_word_some() {
        let word = first_word("hello world").unwrap();   // panics if None
        assert_eq!(word, "hello");
    }

    #[test]
    fn test_first_word_none() {
        assert!(first_word("").is_none());
    }

    // ---- Ignored tests: run only with `cargo test -- --ignored` --------
    #[test]
    #[ignore = "slow test — run explicitly"]
    fn test_slow_operation() {
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert_eq!(add(1, 1), 2);
    }
}

// ============================================================================
//  The `main` function — runs the same assertions at runtime so you can
//  compile this file with plain `rustc main.rs && ./main` (no cargo needed).
// ============================================================================
fn main() {
    println!("=== Running tests inline (same assertions as `cargo test`) ===\n");

    // ---- test_add ----
    assert_eq!(math::add(2, 3), 5);
    assert_eq!(math::add(-1, 1), 0);
    assert_eq!(math::add(0, 0), 0);
    println!("[PASS] test_add");

    // ---- test_sub ----
    assert_eq!(math::sub(5, 3), 2);
    assert_eq!(math::sub(0, 0), 0);
    println!("[PASS] test_sub");

    // ---- test_div_ok ----
    let result = math::div(10, 2).expect("division returned None");
    assert_eq!(result, 5);
    println!("[PASS] test_div_ok");

    // ---- test_div_by_zero ----
    assert!(math::div(10, 0).is_none());
    println!("[PASS] test_div_by_zero");

    // ---- test_first_word_some ----
    let word = math::first_word("hello world").unwrap();
    assert_eq!(word, "hello");
    println!("[PASS] test_first_word_some");

    // ---- test_first_word_none ----
    assert!(math::first_word("").is_none());
    println!("[PASS] test_first_word_none");

    println!("\n=== All inline tests passed ===\n");

    // ========================================================================
    //  HOW TO USE THIS FILE WITH CARGO TEST
    // ========================================================================
    //
    //  1. Create a new cargo project:
    //         cargo new my_project --lib
    //         cd my_project
    //
    //  2. Replace src/lib.rs with this file's contents (minus `main`), OR
    //     put the math module in src/lib.rs and the tests inside
    //     #[cfg(test)] mod tests { ... } at the bottom.
    //
    //  3. Run all tests:
    //         cargo test
    //
    //     You will see output like:
    //         running 7 tests
    //         test tests::test_add ... ok
    //         test tests::test_sub ... ok
    //         test tests::test_div_ok ... ok
    //         test tests::test_first_word_none ... ok
    //         test tests::test_first_word_some ... ok
    //         test tests::test_out_of_bounds_panics ... ok
    //         test tests::test_slow_operation ... ignored
    //
    //  4. Run only ignored (slow) tests:
    //         cargo test -- --ignored
    //
    //  5. Run a specific test by name:
    //         cargo test test_add
    //
    //  6. Show println! output (hidden by default):
    //         cargo test -- --nocapture
    //
    //  7. Run tests in parallel (default) or serially:
    //         cargo test                       # parallel
    //         cargo test -- --test-threads=1   # serial
    //
    //  8. Doctests — code samples inside `///` doc comments are run too:
    //         cargo test --doc
    //
    //     The `first_word` example above is a doctest. If it doesn't compile
    //     or the assertion fails, `cargo test` will tell you.
    //
    //  9. Benchmark tests (nightly only — or use the `criterion` crate):
    //         #[bench] fn my_bench(b: &mut test::Bencher) { ... }
    //
    // ========================================================================

    // ========================================================================
    //  ASSERTION MACROS CHEAT SHEET
    // ========================================================================
    //  assert!(cond)                       — pass if cond is true
    //  assert!(cond, "msg {}", arg)        — pass with custom message
    //  assert_eq!(a, b)                    — pass if a == b
    //  assert_ne!(a, b)                    — pass if a != b
    //  debug_assert!(...)                  — same as assert, but only in debug builds
    //  debug_assert_eq!(...)               — debug-only equality
    //  should_panic                        — attribute: test passes if the function panics
    //  should_panic(expected = "msg")      — pass only if panic message contains "msg"
    //
    //  For PartialEq types, `assert_eq!` shows a nice diff on failure:
    //       left:  5
    //      right: 6
    // ========================================================================
}
