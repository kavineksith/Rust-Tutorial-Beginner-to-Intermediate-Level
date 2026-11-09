# Rust Strings

Rust has **two** string types. This trips up beginners, but the distinction
is fundamental to how Rust manages memory.

## The Two Types

| Type | Owned? | Growable? | Stored |
|------|--------|-----------|--------|
| `String` | yes | yes | heap |
| `&str` | no (borrowed) | no | anywhere (binary, stack, or heap) |

```rust
let literal: &str = "I am a &str";            // baked into the binary
let owned: String = String::from("I am a String");  // heap allocated
```

## Creating a `String`

```rust
let s1 = String::from("from a literal");
let s2 = "literal".to_string();
let s3 = String::new();                       // empty
```

## Growing a `String`

```rust
let mut s = String::from("Hello");
s.push_str(", world!");   // append a &str
s.push('!');              // append a single char
```

## Length and Emptiness

```rust
s.len();          // byte length (not char count for UTF-8!)
s.is_empty();
```

## Concatenation

### Using `+`

```rust
let hello = String::from("Hello, ");
let name  = String::from("Ada");
let greeting = hello + &name;   // + takes &str on the right side
// Note: `hello` is now consumed (moved). `name` is borrowed and still usable.
```

### Using `format!` (most flexible)

```rust
let a = String::from("foo");
let b = String::from("bar");
let c = String::from("baz");
let combined = format!("{}-{}-{}", a, b, c);
// a, b, c are all still usable — format! borrows them.
```

## Indexing: NOT Supported!

```rust
let s = String::from("hello");
// let c = s[0];   // ERROR: Rust does not allow String indexing
```

Why? Because Rust strings are UTF-8, and one byte may not be one character.
Indexing by byte would crash on multi-byte characters. Instead:

```rust
let first_char = s.chars().next().unwrap();
let first_byte = s.as_bytes()[0];
let slice = &s[0..3];              // byte range — must be at char boundaries
```

## Iterating Characters

```rust
for c in "Rust🦀".chars() {
    print!("{} ", c);   // R u s t 🦀
}
```

## UTF-8

Rust strings are **always valid UTF-8**. You can store any Unicode text:

```rust
let s = String::from("日本語");
println!("{}", s);             // 日本語
println!("{}", s.len());       // 9 (bytes!)
println!("{}", s.chars().count()); // 3 (characters)
```

## Useful Methods

| Method | Description |
|--------|-------------|
| `s.trim()` | Remove leading/trailing whitespace |
| `s.replace("a", "b")` | Replace all occurrences |
| `s.split(',')` | Split into an iterator |
| `s.contains("x")` | Check substring |
| `s.starts_with("x")` / `s.ends_with("x")` | Prefix/suffix check |
| `s.to_uppercase()` / `s.to_lowercase()` | Case conversion |
| `s.chars()` | Iterate over chars |
| `s.as_bytes()` | Iterate over bytes |

## Converting Between `String` and `&str`

```rust
let owned: String = String::from("hello");
let borrowed: &str = &owned;            // String -> &str (cheap)
let back: String = borrowed.to_string(); // &str -> String (allocates)
```

Going from `String` to `&str` is free (just a view). Going back allocates.

## Which Should I Use?

| Use `String` when | Use `&str` when |
|---|---|
| You need to own and modify the text | You just need to read text |
| You need to grow / shrink it | A function takes any string-like argument |
| You need to return a new string | You store a borrowed view |

Function parameters usually take `&str` (more flexible — accepts both
`&str` and `&String`).

**Next:** [Ownership](../19_ownership) — Rust's signature feature.
