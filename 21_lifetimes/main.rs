// ============================================================================
//  CHAPTER 33 — RUST LIFETIMES (in depth)
//  Lifetimes are the borrow checker's way of tracking HOW LONG references
//  are valid. Most are inferred; some you must annotate explicitly.
// ============================================================================

// ============================================================================
//  Why lifetimes?
// ============================================================================
//  The borrow checker must prove that no reference ever outlives the data
//  it points to. When the relationship is ambiguous (e.g. a function
//  returning a reference), YOU must annotate lifetimes to spell it out.

// ============================================================================
//  1. The classic example: a function returning a reference
// ============================================================================
//  Without lifetime annotations the compiler doesn't know which input the
//  output reference is tied to. We must tell it:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
//  `<'a>` declares a lifetime parameter.
//  `&'a str` is "a string slice that lives at least as long as 'a".
//  The signature says: "the returned reference lives at least as long as
//  the SHORTER of x and y" — because both inputs share lifetime 'a.

// ============================================================================
//  2. One-sided lifetime: output tied to only one input
// ============================================================================
fn first_word(s: &str) -> &str {       // elided form — see section 4
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
//  Here only `s` is an input reference, so the output is tied to `s`.

// ============================================================================
//  3. Structs that hold references
// ============================================================================
//  A struct that holds a reference MUST declare a lifetime parameter, and
//  the reference's lifetime must outlive the struct.
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // Methods can also use lifetimes. This one returns a reference tied
    // to the struct's lifetime.
    fn part(&self) -> &'a str {
        self.part
    }

    // This method's return lifetime is elided (tied to &self by rule 3).
    fn announce(&self, msg: &str) -> &str {
        println!("{}: {}", msg, self.part);
        self.part
    }
}

// ============================================================================
//  4. Lifetime elision rules — when you can omit annotations
// ============================================================================
//  The compiler applies three rules to infer lifetimes automatically:
//
//  RULE 1: Each input reference gets its own lifetime.
//          fn foo(x: &str)            -> fn foo<'a>(x: &'a str)
//
//  RULE 2: If there is exactly ONE input lifetime, all output references
//          get that lifetime.
//          fn foo(x: &str) -> &str    -> fn foo<'a>(x: &'a str) -> &'a str
//
//  RULE 3: If there are multiple inputs but one of them is &self or &mut
//          self, all output references get self's lifetime.
//          fn foo(&self, x: &str) -> &str
//          -> fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str
//
//  If the rules can't fully determine all output lifetimes, you must
//  annotate them yourself.

// Rule 1+2 — single input, output tied to it (no annotation needed):
fn _upper(s: &str) -> String { s.to_uppercase() }

// Rule 1+2 — single ref input/output (no annotation needed):
fn _first_char(s: &str) -> &str { &s[..1] }

// Multiple inputs, ambiguous — MUST annotate:
// fn _longest(x: &str, y: &str) -> &str { ... }   // ERROR without lifetimes

// ============================================================================
//  5. The 'static lifetime
// ============================================================================
//  `'static` means "lives for the entire program". All string literals
//  have it (they're baked into the binary).
static GREETING: &str = "Hello, world!";   // type: &'static str

fn takes_static(s: &'static str) {
    println!("static: {}", s);
}

// ============================================================================
//  6. Multiple lifetimes
// ============================================================================
//  When inputs have different lifetimes, you can name them separately.
fn split_at<'a, 'b>(text: &'a str, _marker: &'b str) -> &'a str {
    // The output is tied to `text` (lifetime 'a), not to `_marker` ('b).
    // The compiler accepts this because we don't return anything tied to 'b.
    &text[..text.len() / 2]
}

// ============================================================================
//  7. Lifetime bounds on generic types
// ============================================================================
//  Combine `T` with `'a` to say "T must live at least as long as 'a":
//  NOTE: lifetime parameters must come BEFORE type parameters.
fn _use_ref<'a, T: 'a>(r: &'a T) -> &'a T
where
    T: std::fmt::Debug,
{
    println!("{:?}", r);
    r
}

// ============================================================================
//  8. Higher-ranked lifetimes (HRTB) — `for<'a>`
// ============================================================================
//  `for<'a>` means "for any lifetime 'a". Used for closures/funcs that
//  accept a reference of any lifetime.
fn apply_to_str<F>(s: &str, f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    f(s)
}

fn main() {
    // ---- The classic longest example ------------------------------------
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(s1.as_str(), s2.as_str());
    println!("longest: {}", result);

    // ---- Lifetime error demonstration (commented out) -------------------
    //  let r;
    //  {
    //      let s = String::from("temporary");
    //      r = &s;                  // ERROR: s does not live long enough
    //  }
    //  println!("{}", r);          // r would point to freed memory

    // ---- first_word: lifetime inferred ---------------------------------
    let sentence = "hello world rust";
    let word = first_word(sentence);
    println!("first word: {}", word);

    // ---- Struct holding a reference -------------------------------------
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let sentences: Vec<&str> = novel.split('.').collect();
        first_sentence = Excerpt { part: sentences[0] };
    }   // `sentences` goes out of scope here, but `novel` still lives.
    println!("excerpt: {}", first_sentence.part());
    // Note: `first_sentence` is borrowing from `novel`, not from the
    // local `sentences` Vec — that would not compile.

    // ---- 'static lifetime -----------------------------------------------
    takes_static("I am a string literal");      // string literals are 'static
    takes_static(GREETING);
    // let owned = String::from("hi");
    // takes_static(&owned);     // ERROR: &owned does not live 'static

    // ---- Multiple lifetimes ---------------------------------------------
    let long_lived = String::from("data that lives a long time");
    let result = {
        let short_lived = String::from("temp");
        split_at(&long_lived, &short_lived)
    };   // short_lived dropped here, but result borrows long_lived — OK
    println!("split: {}", result);

    // ---- Higher-ranked lifetime -----------------------------------------
    let length = apply_to_str("hello", |s| s.len());
    println!("length via HRTB: {}", length);

    // ========================================================================
    //  Mental model
    // ========================================================================
    //  * Lifetimes are compile-time tags on references.
    //  * `'a` is a NAME for "the scope where this reference is valid".
    //  * The compiler uses lifetimes to PROVE no dangling references exist.
    //  * You annotate only when the compiler can't infer them.
    //  * Once compiled, lifetimes DISAPPEAR — they have zero runtime cost.
    // ========================================================================
}
