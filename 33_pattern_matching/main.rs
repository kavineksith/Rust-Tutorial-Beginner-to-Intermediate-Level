// ============================================================================
//  CHAPTER 44 — RUST PATTERN MATCHING (in depth)
//  Patterns are Rust's most expressive tool. This chapter covers every
//  kind of pattern, where you can use them, and the rules they follow.
// ============================================================================

// Some types we'll match against:
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
enum OptionalInt {
    Some(i32),
    None,
}

fn main() {
    println!("=== Pattern Matching (in depth) ===\n");

    // ======================================================================
    //  1. Literal patterns
    // ======================================================================
    let n = 3;
    match n {
        1 => println!("  one"),
        2 | 3 => println!("  two or three (using `|`)"),
        _ => println!("  other"),
    }

    // ======================================================================
    //  2. Variable bindings
    // ======================================================================
    let x = 5;
    match x {
        n => println!("  matched: {}", n),   // binds n to x
    }

    // ======================================================================
    //  3. The wildcard `_`
    // ======================================================================
    // `_` matches anything and discards the value.
    let _anything: i32 = 42;        // `_` prefix silences unused warning
    match x {
        1 => println!("  one"),
        _ => println!("  not one"),  // catch-all
    }

    // ======================================================================
    //  4. Range patterns
    // ======================================================================
    let age = 25;
    match age {
        0..=12   => println!("  child"),
        13..=19  => println!("  teen"),
        20..=64  => println!("  adult"),
        65..=120 => println!("  senior"),
        _        => println!("  invalid age"),
    }

    // Exclusive range (Rust 1.80+):
    // match n { 1..5 => ..., _ => ... }

    // ======================================================================
    //  5. Destructuring structs
    // ======================================================================
    let p = Point { x: 1, y: 2 };
    match p {
        Point { x, y } => println!("  point at ({}, {})", x, y),
    }
    // Partial struct destructuring:
    match p {
        Point { x, .. } => println!("  only x matters: {}", x),
    }
    // Match specific field values:
    match p {
        Point { x: 0, y } => println!("  on y-axis at {}", y),
        Point { x, y: 0 } => println!("  on x-axis at {}", x),
        Point { x, y }   => println!("  at ({}, {})", x, y),
    }

    // ======================================================================
    //  6. Destructuring tuples
    // ======================================================================
    let pair = (1, "hello", 3.14);
    match pair {
        (1, s, _)    => println!("  first=1, second={}", s),
        (n, _, _)    => println!("  first={}", n),
    }
    // Skip elements with `..`:
    match pair {
        (first, .., last) => println!("  first={:?}, last={:?}", first, last),
    }

    // ======================================================================
    //  7. Destructuring enums
    // ======================================================================
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("  quit"),
        Message::Move { x, y } => println!("  move to ({}, {})", x, y),
        Message::Write(text)   => println!("  write: {}", text),
        Message::ChangeColor(r, g, b) => println!("  rgb({},{},{})", r, g, b),
    }

    // ======================================================================
    //  8. Binding with `@` — capture a value while testing a pattern
    // ======================================================================
    let age = 25;
    match age {
        n @ 0..=12   => println!("  child (age={})", n),
        n @ 13..=19  => println!("  teen (age={})", n),
        n @ 20..=64  => println!("  adult (age={})", n),
        n @ 65..     => println!("  senior (age={})", n),
        _ => unreachable!(),
    }

    // ======================================================================
    //  9. Match guards — extra `if` condition on a pattern
    // ======================================================================
    let pair = (2, -2);
    match pair {
        (x, y) if x == y  => println!("  equal: ({}, {})", x, y),
        (x, y) if x == -y => println!("  opposites: ({}, {})", x, y),
        (x, _) if x > 0   => println!("  positive first: {}", x),
        _                 => println!("  other"),
    }

    // ======================================================================
    //  10. Bindings with `ref` and `ref mut` (rarely needed in modern Rust)
    // ======================================================================
    // In modern Rust (edition 2018+), match ergonomics usually let you
    // write `Some(x)` instead of `Some(ref x)`. The explicit form is still
    // useful when you want to be precise.
    let name = String::from("Alice");
    match name {
        ref s => println!("  borrowed: {}", s),   // s: &String
    }
    println!("  still own name: {}", name);

    // ======================================================================
    //  11. `if let` — match a single pattern
    // ======================================================================
    let maybe: Option<i32> = Some(5);
    if let Some(n) = maybe {
        println!("  if let got: {}", n);
    } else {
        println!("  if let got None");
    }

    // ======================================================================
    //  12. `let else` (Rust 1.65+) — early return on no match
    // ======================================================================
    fn parse_positive(s: &str) -> i32 {
        let n: i32 = match s.parse() {
            Ok(n) if n > 0 => n,
            _ => panic!("not a positive integer"),
        };
        n
    }
    println!("  parse_positive(\"42\") = {}", parse_positive("42"));

    // With `let else`, the above is cleaner:
    fn parse_positive_v2(s: &str) -> i32 {
        let n: i32 = s.parse().ok().filter(|&n| n > 0).expect("not positive");
        n
    }
    println!("  parse_positive_v2(\"42\") = {}", parse_positive_v2("42"));

    // `let else` example with a real early return:
    // Note: `let ... else` cannot appear inside a block expression as a
    // value — it must be a statement. We use it here as a statement:
    let config: Option<&str> = Some("localhost");
    let hostname: &str = match config {
        Some(h) => h,
        None => panic!("hostname is required"),
    };
    println!("  hostname = {}", hostname);

    // In a real function returning Result, you'd write:
    //   let hostname: &str = let Some(h) = config else {
    //       return Err("hostname is required".into());
    //   };
    // (The `let ... else` form is a statement, not an expression.)

    // ======================================================================
    //  13. `while let` — loop while a pattern matches
    // ======================================================================
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);   // 3 2 1
    }
    println!();

    // ======================================================================
    //  14. Patterns in function parameters
    // ======================================================================
    fn print_point(Point { x, y }: &Point) {
        println!("  point: ({}, {})", x, y);
    }
    print_point(&p);

    // ======================================================================
    //  15. Patterns in `for` loops
    // ======================================================================
    let pairs = [(1, "a"), (2, "b"), (3, "c")];
    for (n, letter) in pairs {
        println!("  {} = {}", n, letter);
    }

    // ======================================================================
    //  16. Patterns in `let` statements
    // ======================================================================
    let (a, b, c) = (1, 2, 3);
    println!("  destructured let: a={}, b={}, c={}", a, b, c);

    // Partial destructuring with `..`:
    let (first, .., last) = (1, 2, 3, 4, 5);
    println!("  first={}, last={}", first, last);

    // ======================================================================
    //  17. Subpatterns — nested patterns
    // ======================================================================
    let nested: Option<Option<i32>> = Some(Some(42));
    match nested {
        Some(Some(n)) => println!("  doubly wrapped: {}", n),
        Some(None)    => println!("  inner None"),
        None          => println!("  outer None"),
    }

    // ======================================================================
    //  18. Match ergonomics — modern Rust infers `ref` automatically
    // ======================================================================
    let v: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    for opt in &v {
        match opt {
            Some(n) => println!("  some: {}", n),    // n: &i32 (auto)
            None    => println!("  none"),
        }
    }

    // ========================================================================
    //  PATTERN CHEAT SHEET
    // ========================================================================
    //
    //  Literal                | 1, "hello", 'a', true
    //  Variable binding       | x   (binds the value to x)
    //  Wildcard               | _   (matches anything, discards)
    //  Range                  | 1..=10, 1..10, 'a'..='z'
    //  Or                     | 1 | 2 | 3
    //  Struct destructuring   | Point { x, y }
    //  Struct partial         | Point { x, .. }
    //  Struct literal match   | Point { x: 0, y }
    //  Tuple destructuring    | (a, b, c)
    //  Tuple partial          | (first, .., last)
    //  Enum destructuring     | Some(x), Ok(v), Message::Move { x, y }
    //  Binding with @         | n @ 1..=10   (binds n to matched value)
    //  Guard                  | pattern if condition
    //  Reference              | &x, ref x, ref mut x
    //  Subpattern             | Some(Some(n))
    //
    //  Where patterns are allowed:
    //    * match arms
    //    * if let
    //    * while let
    //    * let statements (single-pattern, irrefutable)
    //    * let else
    //    * function parameters
    //    * for loop variables
    //    * closure parameters
    // ========================================================================
}
