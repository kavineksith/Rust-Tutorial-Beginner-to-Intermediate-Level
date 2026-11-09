// ============================================================================
//  CHAPTER 18 — RUST STRINGS
//  Rust has TWO string types: `String` (owned, growable) and `&str`
//  (borrowed, immutable view). Understanding the difference is essential.
// ============================================================================

fn main() {
    // ======================================================================
    //  &str — string slice (borrowed)
    // ======================================================================
    //  * Immutable view of some text.
    //  * Stored in the binary (string literals) or borrowed from a String.
    //  * Cheap to pass around (just a pointer + length).
    let literal: &str = "I am a string literal";   // type &'static str
    println!("{}", literal);

    // ======================================================================
    //  String — owned, growable, heap-allocated
    // ======================================================================
    let mut owned: String = String::from("Hello");
    owned.push_str(", world!");            // grow the string
    owned.push('!');                        // append one char
    println!("{}", owned);                  // Hello, world!!

    // ---- Creating Strings (many ways) -----------------------------------
    let s1 = String::from("from a &str");
    let s2: String = "literal".to_string();
    let s3 = String::new();                 // empty
    let s4: String = "abc".chars().collect();      // from iterator of chars
    println!("{} / {} / (empty) / {}", s1, s2, s4);

    // ---- Length and emptiness -------------------------------------------
    println!("len = {}", owned.len());
    println!("is_empty = {}", s3.is_empty());

    // ---- Concatenation --------------------------------------------------
    let hello = String::from("Hello, ");
    let name  = String::from("Ada");
    let greeting = hello + &name;           // + takes &str on the right
    println!("{}", greeting);               // Hello, Ada

    // `format!` is the most flexible way to combine strings:
    let a = String::from("foo");
    let b = String::from("bar");
    let c = String::from("baz");
    let combined = format!("{}-{}-{}", a, b, c);  // does NOT consume a, b, c
    println!("{}", combined);
    println!("a, b, c still usable: {} {} {}", a, b, c);

    // ---- Indexing: NOT supported! ---------------------------------------
    // You CANNOT do `s[0]`. Strings are UTF-8, and one byte may not be
    // one character. Instead, use:
    let s = String::from("Hello");
    let first_char = s.chars().next().unwrap();
    let first_byte = s.as_bytes()[0];
    println!("first char: {}, first byte: {}", first_char, first_byte);

    // Slicing by BYTE range (be careful with UTF-8!):
    let hello_bytes = &s[0..3];             // "Hel" — must be at char boundaries
    println!("slice: {}", hello_bytes);

    // ---- Iterating over characters --------------------------------------
    for ch in "Rust🦀".chars() {
        print!("{} ", ch);                  // R u s t 🦀
    }
    println!();

    // ---- UTF-8 support --------------------------------------------------
    let unicode = String::from("日本語");
    println!("Unicode string: {}, len_bytes={}", unicode, unicode.len());
    println!("char count: {}", unicode.chars().count());

    // ---- Replace, trim, split -------------------------------------------
    let dirty = "  hello world  ";
    println!("[{}]", dirty.trim());         // "hello world"

    let sentence = "hello,world,rust";
    for word in sentence.split(',') {
        print!("[{}] ", word);              // [hello] [world] [rust]
    }
    println!();

    let replaced = "I like cats".replace("cats", "dogs");
    println!("{}", replaced);               // I like dogs

    // ---- Converting between String and &str -----------------------------
    let owned_str: String = String::from("hello");
    let borrowed: &str = &owned_str;        // String -> &str (cheap)
    let back_to_owned: String = borrowed.to_string();   // &str -> String (allocates)
    println!("{} {} {}", owned_str, borrowed, back_to_owned);
}
