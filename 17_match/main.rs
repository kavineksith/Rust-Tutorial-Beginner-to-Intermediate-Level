// ============================================================================
//  CHAPTER 12 — RUST MATCH
//  Pattern matching — Rust's most powerful control-flow construct.
// ============================================================================

fn main() {
    // ---- Basic match on an integer --------------------------------------
    let number = 3;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),     // `|` combines patterns
        6..=9 => println!("between six and nine"), // range pattern
        _ => println!("something else"),       // `_` is the wildcard
    }

    // ---- Match is an expression -----------------------------------------
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("result = {}", result);

    // ---- Match on a bool -------------------------------------------------
    let boolean = true;
    let text = match boolean {
        false => "no",
        true  => "yes",
    };
    println!("text = {}", text);

    // ---- Match on Option<T> ---------------------------------------------
    let maybe_name: Option<&str> = Some("Ada");
    let name_length = match maybe_name {
        Some(name) => name.len(),
        None => 0,
    };
    println!("name_length = {}", name_length);

    // ---- Match on enum (preview of chapter 27) --------------------------
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let coin = Coin::Dime;
    let value = match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    };
    println!("coin value = {}", value);

    // ---- Binding values in patterns -------------------------------------
    // Useful when matching an enum that carries data:
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }

    // ---- Match guards (extra `if` condition) ----------------------------
    let pair = (2, -2);
    match pair {
        (x, y) if x == y     => println!("equal: ({}, {})", x, y),
        (x, y) if x == -y    => println!("opposites: ({}, {})", x, y),
        (x, _) if x > 0      => println!("positive first: {}", x),
        _                    => println!("other"),
    }

    // ---- Binding with `@` to capture a value while testing a range ------
    // We use u32 so the ranges are exhaustive (no negative ages).
    let age: u32 = 25;
    match age {
        n @ 0..=12   => println!("child ({} years)", n),
        n @ 13..=19  => println!("teen ({} years)", n),
        n @ 20..=64  => println!("adult ({} years)", n),
        n @ 65..     => println!("senior ({} years)", n),
    }
}
