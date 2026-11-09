// ============================================================================
//  CHAPTER 20 — RUST BORROWING
//  Borrowing = using a value via a reference WITHOUT taking ownership.
//  This is the everyday alternative to moving values around.
// ============================================================================

fn main() {
    // ======================================================================
    //  IMMUTABLE BORROWS: &T
    // ======================================================================
    //  You can have AS MANY immutable borrows as you want at the same time.
    let s = String::from("hello");
    let r1 = &s;                        // first immutable borrow
    let r2 = &s;                        // second immutable borrow
    println!("{} {} {}", s, r1, r2);    // all three usable together

    // ======================================================================
    //  MUTABLE BORROWS: &mut T
    // ======================================================================
    //  You can have ONLY ONE mutable borrow at a time. And you cannot mix
    //  mutable and immutable borrows in the same scope.
    let mut s2 = String::from("hello");
    let mr = &mut s2;                   // ONE mutable borrow
    mr.push_str(" world");
    println!("{}", mr);
    // let mr2 = &mut s2;               // ERROR: cannot borrow mutably twice

    // After mr goes out of use, we can mutably borrow again:
    let mr3 = &mut s2;
    mr3.push('!');
    println!("{}", mr3);

    // ======================================================================
    //  THE BORROW CHECKER RULE
    // ======================================================================
    //  At any point, a value can be borrowed EITHER:
    //    * by any number of IMMUTABLE references, OR
    //    * by exactly ONE MUTABLE reference.
    //  Never both. This prevents data races at compile time.

    // ======================================================================
    //  Borrowing in function calls
    // ======================================================================
    let greeting = String::from("Hello, world!");
    let length = calculate_length(&greeting);   // borrow, don't move
    println!("'{}' has length {}", greeting, length);

    let mut mutable_string = String::from("hi");
    append_exclamation(&mut mutable_string);    // mutably borrow
    println!("{}", mutable_string);

    // ======================================================================
    //  Slices — a special kind of borrow
    // ======================================================================
    //  A slice is a borrowed view into a CONTIGUOUS sequence.
    let data = String::from("hello world");
    let hello: &str = &data[0..5];      // "hello"
    let world: &str = &data[6..11];     // "world"
    println!("{} / {}", hello, world);

    // String slices are just &str. The borrow keeps the owner alive.
    let first_word = first_word(&data);
    println!("first word: {}", first_word);

    // ======================================================================
    //  Lifetimes in brief (advanced)
    // ======================================================================
    //  Every borrow has a LIFETIME — the region of code where the
    //  reference is valid. The compiler usually infers it.
    //  When it can't, you must annotate lifetimes explicitly with `'a`:
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let longest = longest(&s1, &s2);    // both inputs share a lifetime
    println!("longest: {}", longest);

    // ======================================================================
    //  Common borrow checker errors (commented out — uncomment to see)
    // ======================================================================
    // let r;
    // {
    //     let x = 5;
    //     r = &x;                        // ERROR: x does not live long enough
    // }
    // println!("{}", r);

    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(4);                         // ERROR: mutable borrow while
    // println!("{}", first);            //        immutable borrow is alive
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s is borrowed — it is NOT dropped when this function returns.
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

// A function that borrows a string and returns a slice of it.
// The returned slice's lifetime is tied to the input's lifetime (`'a`).
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Lifetime annotation: both inputs and the output share lifetime 'a.
// Means: the returned reference lives at least as long as the shorter input.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
