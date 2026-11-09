# Rust Output

Rust prints to the terminal through **macros** (not functions):

| Macro | Newline? | Stream |
|-------|----------|--------|
| `print!` | no | stdout |
| `println!` | yes | stdout |
| `eprint!` | no | stderr |
| `eprintln!` | yes | stderr |
| `format!` | — | returns a `String`, does not print |

## Basic Usage

```rust
println!("Hello, world!");
print!("No newline. ");
println!("Now we have a newline.");
```

## Placeholders `{}`

```rust
let name = "John";
let age = 30;
println!("{} is {} years old.", name, age);
```

Rust uses `{}` as a placeholder. Arguments fill them in order.

## Named Arguments

```rust
println!("{name} is {age}", name = "Ada", age = 36);
// or, with variables already in scope:
let name = "Ada";
let age = 36;
println!("{name} is {age}");
```

## Positional Arguments

```rust
println!("{0} and {1} and {0}", "a", "b"); // a and b and a
```

## Debug Formatting

```rust
let v = vec![1, 2, 3];
println!("{:?}", v);    // [1, 2, 3]
println!("{:#?}", v);   // pretty-printed, multi-line
```

Use `{:?}` for quick inspection. The type must implement the `Debug` trait
(most standard types do; for your own types, add `#[derive(Debug)]`).

## Number Formatting

```rust
let pi = 3.14159;
println!("{:.2}", pi);    // 3.14        (2 decimals)
println!("{:08.2}", pi);  // 0003.14     (width 8, zero-padded)
println!("{:>10}", "hi"); //         hi  (right aligned)
println!("{:<10}|", "hi");// hi         | (left aligned)
println!("{:^10}", "hi"); //    hi       (centered)
```

## Number Bases

```rust
let n = 255u32;
println!("{:b}", n); // 11111111  binary
println!("{:o}", n); // 377       octal
println!("{:x}", n); // ff        hex lowercase
println!("{:X}", n); // FF        hex uppercase
```

## Escape Sequences

| Sequence | Meaning |
|----------|---------|
| `\n` | newline |
| `\t` | tab |
| `\r` | carriage return |
| `\\` | backslash |
| `\"` | double quote |
| `\u{XXXX}` | unicode codepoint (e.g. `\u{1F980}` = 🦀) |

## stderr

```rust
eprintln!("Something went wrong");
```

Use `eprintln!` for errors and logs — they will not pollute normal output
when the user pipes your program with `|` or `>`.

**Next:** [Comments](../10_comments) — documenting your code.
