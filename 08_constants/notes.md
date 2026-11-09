# Rust Constants

Constants are values that **never change** for the entire lifetime of the
program.

## Declaring a Constant: `const`

```rust
const MAX_USERS: u32 = 100;
const PI: f64 = 3.141592653589793;
const GREETING: &str = "Welcome to Rust";
```

Rules:

* Use the `const` keyword (not `let`).
* Type MUST be annotated explicitly.
* Value must be a **compile-time constant expression**.
* Naming convention: `UPPER_SNAKE_CASE`.
* Constants are **inlined** wherever they are used — they have no fixed
  memory address.
* Constants may be declared in any scope, including the global scope.

## `const` vs `let`

| | `let` | `const` |
|---|-------|---------|
| Evaluated at | runtime | compile time |
| Mutable | with `mut` | never |
| Type annotation | optional | required |
| Scope | block-scoped | any scope, including global |

```rust
let x = 5;            // runtime binding
const Y: i32 = 5;     // compile-time constant
```

## Compile-time Computation

```rust
const SECONDS_PER_HOUR: u32 = 60 * 60;
const SECONDS_PER_DAY: u32 = SECONDS_PER_HOUR * 24;
```

The compiler evaluates these at compile time.

## Using Constants in Array Sizes

```rust
const SIZE: usize = 10;
let arr = [0u8; SIZE];
```

## `static` (Related Concept)

```rust
static APP_NAME: &str = "MyApp";
```

* Has a **fixed memory address** for the whole program (unlike `const`).
* May be mutable with `static mut`, but reading/writing one requires `unsafe`.
* Use sparingly — `const` is usually what you want.

## `const` vs `static` Summary

| | `const` | `static` |
|---|---------|---------|
| Memory address | none (inlined) | fixed |
| Mutable | no | only with `static mut` (unsafe) |
| Lifetime | inline | entire program |
| Typical use | configuration values, math constants | global singletons |

## When to Use Constants

* Math constants (`PI`, `E`).
* Configuration values (`MAX_CONNECTIONS`, `DEFAULT_TIMEOUT`).
* Magic numbers — give them a name to make code self-documenting.

```rust
const MAX_RETRIES: u32 = 3;
for attempt in 1..=MAX_RETRIES {
    // ...
}
```

**Next:** [Operators](../06_operators) — arithmetic, comparison, logical.
