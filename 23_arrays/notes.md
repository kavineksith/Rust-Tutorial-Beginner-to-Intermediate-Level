# Rust Arrays

An array is a **fixed-size** collection of values of the **same type**,
stored on the stack.

## Creating an Array

```rust
let nums: [i32; 5] = [1, 2, 3, 4, 5];   // 5 i32s
let zeros = [0; 10];                     // 10 zeros (shorthand)
let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
              "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
```

`[value; count]` creates an array of `count` copies of `value`.

## Accessing Elements

```rust
let first = nums[0];   // 1
let last  = nums[4];   // 5
```

## Bounds Checking

Rust checks array indices **at runtime**. Out-of-bounds access **panics**:

```rust
let nums = [1, 2, 3];
// let x = nums[10];   // PANIC at runtime: index out of bounds
```

This prevents the buffer overflows that plague C/C++.

## Length

```rust
nums.len();      // 5
nums.is_empty(); // false
```

## Iterating

```rust
for n in nums.iter() {
    println!("{}", n);
}

for (i, n) in nums.iter().enumerate() {
    println!("[{}] = {}", i, n);
}
```

## Mutable Arrays

```rust
let mut scores = [0; 3];
scores[0] = 10;
scores[1] = 20;
scores[2] = 30;
```

## Slices — A Borrowed View

A slice `&[T]` borrows a contiguous chunk of an array:

```rust
let nums = [1, 2, 3, 4, 5];
let slice: &[i32] = &nums[1..4];   // [2, 3, 4]
```

Slices work for arrays of **any size** — this is the standard way to write
functions that take array-like input:

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

sum(&nums);   // works for any size array
```

## Array Length Is Part of the Type

`[i32; 5]` and `[i32; 6]` are **different types**:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b: [i32; 6] = [1, 2, 3, 4, 5, 6];
// They cannot be passed to a function expecting the other size.
```

## Multi-dimensional Arrays

```rust
let matrix: [[i32; 3]; 2] = [
    [1, 2, 3],
    [4, 5, 6],
];
println!("{}", matrix[1][2]);   // 6
```

## Arrays vs Vectors

| | Array `[T; N]` | Vector `Vec<T>` |
|---|----------------|------------------|
| Size | fixed at compile time | growable at runtime |
| Storage | stack | heap |
| Use when | size is known & small | size is dynamic |

## Converting to `Vec`

```rust
let arr = [1, 2, 3];
let v: Vec<i32> = arr.to_vec();
```

## Const Generics — Functions Over Any Array Size

```rust
fn sum<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}
```

This works for arrays of any size — modern Rust feature.

## When to Use Arrays

* A small, fixed-size group of related values (coordinates, RGB color).
* Compile-time-known sizes (12 months, 7 weekdays).
* Stack-allocated buffers for performance.

For almost everything else, prefer `Vec<T>`.

**Next:** [Vectors](../24_vectors) — growable lists.
