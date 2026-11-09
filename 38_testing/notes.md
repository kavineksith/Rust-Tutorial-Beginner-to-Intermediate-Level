# Rust Testing

Rust has **built-in** testing support — no external test framework needed.
`cargo test` finds every `#[test]` function in your crate and runs it.

## The Three Kinds of Tests

| Kind | Where | Run with |
|------|------|----------|
| **Unit tests** | `#[cfg(test)] mod tests` inside source files | `cargo test` |
| **Integration tests** | `tests/` directory (one file per test binary) | `cargo test` |
| **Doctests** | Code blocks inside `///` doc comments | `cargo test --doc` |

## A Unit Test

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }
}
```

* `#[cfg(test)]` — compile this module only during `cargo test`.
* `use super::*` — bring the parent module's items into scope.
* `#[test]` — mark a function as a test.

Run with `cargo test`:

```
running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed
```

## Assertion Macros

| Macro | Meaning |
|-------|---------|
| `assert!(cond)` | pass if `cond` is true |
| `assert!(cond, "msg {}", arg)` | with a custom message |
| `assert_eq!(a, b)` | pass if `a == b` (shows diff on failure) |
| `assert_ne!(a, b)` | pass if `a != b` |
| `debug_assert!(...)` | like `assert!` but only in debug builds |
| `debug_assert_eq!(...)` | debug-only equality |

```rust
#[test]
fn test_something() {
    let result = compute();
    assert!(result > 0);
    assert_eq!(result, 42);
    assert_ne!(result, 0);
}
```

## Tests That Should Panic

```rust
#[test]
#[should_panic]
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_out_of_bounds_with_message() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

`expected = "..."` checks that the panic message contains the given string.

## Tests That Return `Result`

```rust
#[test]
fn test_div() -> Result<(), String> {
    let r = div(10, 2).ok_or("returned None")?;
    if r == 5 { Ok(()) } else { Err(format!("got {}", r)) }
}
```

Using `Result` lets you use the `?` operator inside tests — failure is
reported as a test failure rather than a panic.

## Ignored Tests

```rust
#[test]
#[ignore = "slow test — run explicitly"]
fn test_slow() {
    std::thread::sleep(std::time::Duration::from_secs(10));
    assert_eq!(2 + 2, 4);
}
```

Ignored tests don't run by default. Run them with:

```bash
cargo test -- --ignored
```

## Running Tests

```bash
cargo test                       # run all tests
cargo test test_add              # run tests whose name contains "test_add"
cargo test -- --nocapture        # show println! output
cargo test -- --test-threads=1   # run serially (default: parallel)
cargo test -- --ignored          # run only ignored tests
cargo test --doc                 # run only doctests
```

## Doctests

Code blocks inside `///` doc comments are compiled and run as tests:

```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

Run with `cargo test` (or `cargo test --doc` for doctests only).

## Integration Tests

Put files in `tests/` — each file becomes a separate test binary:

```
my_crate/
├── src/
│   └── lib.rs
└── tests/
    ├── api_test.rs        # tests the public API
    └── integration.rs     # end-to-end tests
```

```rust
// tests/api_test.rs
use my_crate::add;

#[test]
fn test_api() {
    assert_eq!(add(2, 3), 5);
}
```

Integration tests can only access the **public** API of your crate.

## Test Organization

```
src/
├── lib.rs             # library code + #[cfg(test)] mod tests
├── math.rs            # math module + its tests
└── network/
    ├── mod.rs
    └── client.rs      # client code + its tests
tests/
├── api.rs             # integration tests
└── e2e.rs
```

Convention: **unit tests live next to the code they test**, inside a
`#[cfg(test)] mod tests` block. Integration tests live in `tests/`.

## Common Patterns

### Setup / Teardown

Rust has no built-in setup/teardown. Use a helper function:

```rust
fn make_user() -> User {
    User { name: "Test".to_string(), age: 30 }
}

#[test]
fn test_user_name() {
    let u = make_user();
    assert_eq!(u.name, "Test");
}
```

### Parameterized Tests

No built-in support. Use a loop:

```rust
#[test]
fn test_add_many() {
    for (a, b, expected) in [(1, 1, 2), (2, 3, 5), (10, -5, 5)] {
        assert_eq!(add(a, b), expected, "add({},{})", a, b);
    }
}
```

### Testing Across Threads

```rust
#[test]
fn test_threaded() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    assert_eq!(*counter.lock().unwrap(), 10);
}
```

### Snapshot Testing (with `insta` crate)

```rust
#[test]
fn test_snapshot() {
    let output = format_output();
    insta::assert_snapshot!(output);
}
```

`insta` saves the first run as a "snapshot" and compares future runs
against it. Great for testing complex output.

## Useful Crates

| Crate | Purpose |
|-------|---------|
| `proptest` | property-based testing |
| `quickcheck` | property-based testing (simpler) |
| `insta` | snapshot testing |
| `mockall` | mocking |
| `criterion` | benchmarks (stable Rust) |
| `rstest` | parameterized / fixture-based tests |
| `fake` | generating random test data |
| `pretty_assertions` | nicer `assert_eq!` diffs |

## Test-Driven Development (TDD) in Rust

1. Write a failing test.
2. Run `cargo test` — confirm it fails for the right reason.
3. Write the minimum code to pass.
4. Run `cargo test` — confirm it passes.
5. Refactor; re-run tests.

```bash
cargo watch -x test    # auto-run tests on every file change
```

## Common Mistakes

### Forgetting `#[cfg(test)]`

```rust
mod tests { /* ... */ }   // always compiled — bloats release binary
```

Fix:

```rust
#[cfg(test)]
mod tests { /* ... */ }
```

### Using `unwrap()` Everywhere

Tests that `unwrap` panic on the wrong thing. Prefer `assert!` with a
message:

```rust
// Bad: panics with "called Option::unwrap() on a None value"
let v = option.unwrap();

// Better: clear failure message
let v = option.expect("should have a value");
```

### Tests That Depend on Order

Rust runs tests in **parallel** by default and in **arbitrary order**. A
test that depends on another test running first is fragile.

Fix: each test sets up its own state. Don't share global mutable state.

### Tests That Depend on Time / Network

Flaky tests are worse than no tests. Use:

* Mock the clock (`mockall`, `tokio::time::pause`).
* Mock the network (`wiremock`, `httpmock`).
* Skip network tests by default (`#[ignore]`).

## `cargo test` Output

```
running 3 tests
test tests::test_add ... ok
test tests::test_sub ... ok
test tests::test_div ... FAILED

failures:

---- tests::test_div stdout ----
thread 'tests::test_div' panicked at 'assertion failed: `(left == right)`
  left:  3,
  right: 5', src/lib.rs:12:5

test result: FAILED. 2 passed; 1 failed
```

* `.` = passed, `F` = failed.
* The panic location and assertion are shown.
* `cargo test` exits non-zero if any test failed — good for CI.

## Summary

| Want | Use |
|------|-----|
| Unit test | `#[test] fn foo()` in `#[cfg(test)] mod tests` |
| Should panic | `#[should_panic]` |
| Ignored (slow) | `#[ignore]` |
| Doc example | `///` with ``` ``` ``` block |
| Integration test | `tests/foo.rs` |
| Property-based | `proptest` / `quickcheck` |
| Snapshot | `insta` |
| Benchmark | `criterion` (stable) / `#[bench]` (nightly) |
| Run all | `cargo test` |
| Run by name | `cargo test name_pattern` |
| Show output | `cargo test -- --nocapture` |
| Serial | `cargo test -- --test-threads=1` |

**Next:** [Macros](../36_macros) — `macro_rules!` and procedural macros.
