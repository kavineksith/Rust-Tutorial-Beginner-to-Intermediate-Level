# Rust Loops

The `loop` keyword creates an **infinite loop** — it runs forever until you
explicitly `break` out.

## Basic `loop`

```rust
let mut count = 0;
loop {
    count += 1;
    if count >= 5 {
        break;
    }
}
println!("count = {}", count);
```

* `break` exits the loop immediately.
* `continue` skips to the next iteration.

## `loop` Is an Expression

Like `if` and `match`, `loop` returns a value via `break value;`:

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;   // <-- value returned
    }
};
println!("result = {}", result);   // 20
```

## `continue`

```rust
loop {
    n += 1;
    if n > 10 { break; }
    if n % 2 == 0 { continue; }   // skip even numbers
    print!("{} ", n);              // 1 3 5 7 9
}
```

## Loop Labels

When you nest loops, you can name them with `'label:` and use
`break 'label` / `continue 'label` to control which loop is affected:

```rust
'outer: loop {
    loop {
        if some_condition {
            break 'outer;   // exits the OUTER loop
        }
    }
}
```

Labels start with a single quote `'` (a "lifetime" syntax in Rust).

## Common Patterns

### Retry Until Success

```rust
let mut attempts = 0;
let success = loop {
    attempts += 1;
    if try_thing() {
        break true;
    }
    if attempts >= 5 {
        break false;
    }
};
```

### Find First Match

```rust
let found = loop {
    let item = next_item();
    if matches(item) {
        break item;
    }
};
```

## `loop` vs `while` vs `for`

| Construct | When to use |
|-----------|-------------|
| `loop` | infinite loop, or you don't know the exit condition up front |
| `while` | loop while a condition is `true` |
| `for` | iterate over a collection or range |

## Infinite Loop With No Body? Use `loop`

```rust
loop {}   // valid, runs forever, does nothing
```

The compiler accepts this because `loop` cannot fall through — only `break`
can exit, so an empty body is fine (used for halting the program).

## Avoid Spinning (Use Sleep)

A bare `loop {}` with no `break` will pin a CPU core at 100%. In real code
you usually sleep or wait for I/O:

```rust
use std::thread;
use std::time::Duration;

loop {
    do_work();
    thread::sleep(Duration::from_millis(100));
}
```

**Next:** [While Loops](../15_while_loops) — condition-based repetition.
