# Rust While Loops

`while` runs a block **while** a condition is `true`. When the condition
becomes `false`, the loop stops.

## Basic `while`

```rust
let mut n = 1;
while n <= 5 {
    println!("n = {}", n);
    n += 1;
}
```

Output:

```
n = 1
n = 2
n = 3
n = 4
n = 5
```

## Counting Down

```rust
let mut countdown = 3;
while countdown > 0 {
    println!("{}...", countdown);
    countdown -= 1;
}
println!("Liftoff!");
```

## Using a Boolean Flag

```rust
let mut keep_going = true;
let mut i = 0;
while keep_going {
    i += 1;
    if i >= 3 {
        keep_going = false;
    }
}
```

## `break` and `continue`

```rust
let mut i = 0;
while i < 10 {
    i += 1;
    if i % 3 == 0 { continue; }   // skip multiples of 3
    if i == 8    { break; }       // stop at 8
    print!("{} ", i);             // 1 2 4 5 7
}
```

## `while let` (Pattern Match + Loop)

A superpower: keep looping **while** a pattern matches.

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("popped: {}", top);
}
// Output: 3, 2, 1
```

`stack.pop()` returns `Option<T>`. Each iteration tries to match `Some(top)`.
When the stack is empty, `pop()` returns `None`, the pattern fails, and the
loop ends.

## `while let` with Iterators

```rust
let mut iter = vec!["a", "b", "c"].into_iter();
while let Some(item) = iter.next() {
    println!("{}", item);
}
```

This is equivalent to a `for` loop — but useful when you need to do something
extra between iterations.

## When to Use `while`

| Use `while` | Use `for` |
|-------------|-----------|
| Number of iterations is unknown | Iterating over a known collection |
| Polling / retrying until success | Iterating over a range |
| Looping until a condition becomes false | Iterating over an iterator |

## Watch Out for Infinite Loops

If you forget to update the condition, `while` runs forever:

```rust
let mut x = 0;
while x < 10 {
    println!("{}", x);   // oops, x never changes -> infinite loop
}
```

`clippy` will warn you about obvious cases, but always double-check.

## `loop` vs `while`

```rust
// Equivalent:
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}

let mut i = 0;
loop {
    if i >= 5 { break; }
    println!("{}", i);
    i += 1;
}
```

`while` is clearer when you have a clear exit condition. `loop` is clearer
when the exit happens in the middle of the body.

**Next:** [For Loops](../16_for_loops) — iterating over collections.
