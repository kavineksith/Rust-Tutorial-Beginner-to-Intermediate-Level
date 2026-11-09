# Rust Data Types

Rust is **statically typed** — every value has a type known at compile time.
The compiler can usually infer types, but you must annotate them at function
boundaries.

## Scalar Types

A scalar type represents a single value.

### Integers

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | `i8`   | `u8`     |
| 16-bit | `i16`  | `u16`    |
| 32-bit | `i32`  | `u32`    |
| 64-bit | `i64`  | `u64`    |
| 128-bit | `i128` | `u128`  |
| arch  | `isize` | `usize`  |

`isize` / `usize` match the size of a pointer (64-bit on most modern machines).

```rust
let a: i32 = -42;
let b: u64 = 100;
let c: usize = 0;
```

Integer literals:

```rust
let dec = 1_000;          // decimal (underscores allowed)
let hex = 0xFF;           // hex
let oct = 0o777;          // octal
let bin = 0b1010_1010;    // binary
let byte = b'A';          // u8 byte literal
```

### Floating-Point

`f32` and `f64`. Default is `f64`:

```rust
let x: f32 = 3.14;
let y = 2.718;            // f64
```

### Boolean

```rust
let t: bool = true;
let f = false;
```

### Character

A `char` is a 4-byte **Unicode scalar value** (not 1 byte like C):

```rust
let c = 'z';
let emoji = '🦀';
```

## Compound Types

Group multiple values.

### Tuple

Fixed size, mixed types:

```rust
let t: (i32, f64, char) = (1, 2.5, 'x');
let (a, b, c) = t;        // destructuring
println!("{}", t.0);      // access by index
```

### Array

Fixed size, same type, stored on the stack:

```rust
let nums: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 3];       // [0, 0, 0]
println!("{}", nums[0]);
```

### Strings

| Type | Owned? | Growable? |
|------|--------|-----------|
| `&str` | borrowed | no |
| `String` | yes | yes |

```rust
let s1: &str = "hello";
let s2: String = String::from("world");
```

## Type Inference

```rust
let v = vec![1, 2, 3];   // Vec<i32> — inferred
let s = v.iter().sum::<i32>();
```

## Type Casting with `as`

```rust
let x = 3.99_f64;
let y = x as i32;        // 3 (truncates towards zero)

let big: i64 = 1_000_000_000_000;
let small = big as i32;  // may truncate!
```

## Integer Overflow

In debug mode, integer overflow panics. In release mode it wraps silently.
Use the explicit methods if you want a specific behaviour:

```rust
let x: u8 = 255;
let y = x.wrapping_add(1);  // wraps to 0, no panic
let z = x.checked_add(1);   // returns None on overflow
```

**Next:** [Constants](../08_constants) — values that never change.
