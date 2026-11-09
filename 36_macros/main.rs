// ============================================================================
//  CHAPTER 39 — RUST MACROS
//  Macros generate code at COMPILE TIME. They are Rust's metaprogramming
//  tool — used for things functions can't do: variadic args, compile-time
//  string formatting, custom DSLs, derive implementations.
// ============================================================================

// ============================================================================
//  PART A — Declarative Macros (`macro_rules!`)
// ============================================================================
//  Pattern-matching over tokens. The most common kind — `println!`, `vec!`,
//  `format!` are all `macro_rules!` macros.

// ---- A simple macro: greet!(name) -> println!("Hello, {name}!") ------
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name)
    };
}

// ---- A variadic macro: sum!(1, 2, 3, 4) -> 1 + 2 + 3 + 4 -------------
// `$( ... ),*` matches zero or more comma-separated repetitions.
macro_rules! sum {
    // Base case: a single expression
    ($single:expr) => { $single };
    // Recursive case: first expr, comma, then recurse on the rest
    ($first:expr, $($rest:expr),+) => {
        $first + sum!($($rest),+)
    };
}

// ---- The classic vec![] macro (simplified) ---------------------------
// Matches three different syntaxes:
//   vec![]                 -> empty
//   vec![1, 2, 3]          -> list of values
//   vec![1; 5]             -> 5 copies of 1
macro_rules! my_vec {
    () => { Vec::new() };
    ( $first:expr $(, $rest:expr)* $(,)? ) => {{
        let mut v = Vec::new();
        v.push($first);
        $( v.push($rest); )*
        v
    }};
    ( $val:expr ; $n:expr ) => {{
        let mut v = Vec::with_capacity($n as usize);
        for _ in 0..$n { v.push($val); }
        v
    }};
}

// ---- A macro that creates a HashMap from key-value pairs -------------
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )*
        m
    }};
}

// ---- A debugging macro that prints variable name AND value ----------
macro_rules! dbg_var {
    ($var:ident) => {
        println!(concat!(stringify!($var), " = {:?}"), $var)
    };
}

// ============================================================================
//  Macro fragment specifiers — what each `$x:TYPE` matches
// ============================================================================
//  :expr      — any Rust expression (1 + 2, foo(), vec![])
//  :ident     — an identifier (variable name, function name)
//  :ty        — a type (i32, Vec<String>, &str)
//  :path      — a path (std::vec::Vec)
//  :stmt      — a statement
//  :pat       — a pattern (Some(x), (a, b))
//  :literal   — a literal (42, "hi", true)
//  :meta      — a meta item (used inside #[...] attributes)
//  :tt        — a single token tree (most flexible, used in complex macros)
//  :vis       — a visibility modifier (pub, pub(crate), nothing)
//  :lifetime  — a lifetime ('a, 'static)
//  :block     — a block { ... }

// ---- A macro using multiple fragment types ----------------------------
macro_rules! declare_static {
    ($name:ident, $ty:ty, $val:expr) => {
        static $name: $ty = $val;
    };
}
declare_static!(APP_VERSION, &str, "1.0.0");

// ---- The $(,)? trick: allow an optional trailing comma ----------------
// `$(,)?` matches an optional trailing comma, so users can write
// `foo!(1, 2, 3,)` as well as `foo!(1, 2, 3)`.

// ============================================================================
//  PART B — Procedural Macros (overview)
// ============================================================================
//  Procedural macros are Rust FUNCTIONS that take TokenStream as input
//  and return TokenStream as output. They are more powerful than
//  `macro_rules!` but require a separate crate of type `proc-macro`.
//
//  Four kinds:
//    1. Function-like (#[proc_macro])
//    2. Derive (#[proc_macro_derive])
//    3. Attribute (#[proc_macro_attribute])
//    4. proc_macro_derive with helper attributes

// ---- A simplified derive macro sketch (won't compile without the right
//      crate setup, shown for illustration):
//
//  // In a separate crate (Cargo.toml: [lib] proc-macro = true)
//  use proc_macro::TokenStream;
//
//  #[proc_macro_derive(HelloMacro)]
//  pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//      // Parse the input tokens into a syntax tree
//      let ast = syn::parse_macro_input!(input as DeriveInput);
//      // Build the output (impl block)
//      let name = &ast.ident;
//      let expanded = quote! {
//          impl HelloMacro for #name {
//              fn hello_macro() {
//                  println!("Hello, Macro! My name is {}", stringify!(#name));
//              }
//          }
//      };
//      expanded.into()
//  }
//
//  Then in your main crate:
//      #[derive(HelloMacro)]
//      struct Pancakes;
//      Pancakes::hello_macro();   // prints "Hello, Macro! My name is Pancakes"

// ---- The `quote`, `syn`, `proc-macro2` crates are the standard toolkit:
//  * `syn`         — parse TokenStream into a syntax tree
//  * `quote`       — turn Rust code back into TokenStream
//  * `proc-macro2` — work with tokens outside a proc-macro context (testing)

// ============================================================================
//  PART C — Built-in macros you'll use every day
// ============================================================================
//  println! / print! / eprintln! / eprint! — printing
//  format!                                — build a String
//  vec!                                   — build a Vec
//  dbg!                                   — debug-print an expression
//  panic!                                 — abort the program
//  assert! / assert_eq! / assert_ne!      — runtime assertions
//  todo!() / unimplemented!()             — placeholder that panics
//  unreachable!()                         — mark impossible code
//  matches!(val, pattern)                 — pattern match to bool
//  cfg!(feature = "x")                    — compile-time feature check
//  env!("VAR")                            — read an env var at compile time
//  file!() / line!() / column!()          — current source location
//  include_str!("path") / include_bytes!  — embed file contents
//  stringify!(ident)                      — turn ident into a string literal

fn main() {
    // ---- Using our custom greet! macro ----
    greet!("world");
    greet!("Rust");

    // ---- Using our sum! macro (variadic) ----
    println!("sum!(1) = {}", sum!(1));
    println!("sum!(1, 2) = {}", sum!(1, 2));
    println!("sum!(1, 2, 3, 4, 5) = {}", sum!(1, 2, 3, 4, 5));

    // ---- Using my_vec! ----
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3, 4];
    let v3 = my_vec![0; 5];
    println!("v1 = {:?}, v2 = {:?}, v3 = {:?}", v1, v2, v3);

    // ---- Using hashmap! ----
    let scores = hashmap! {
        "Alice" => 10,
        "Bob" => 20,
        "Carol" => 30,    // trailing comma is allowed
    };
    println!("scores = {:?}", scores);

    // ---- Using dbg_var! ----
    let x = 42;
    let name = "Ada";
    dbg_var!(x);          // prints "x = 42"
    dbg_var!(name);       // prints "name = \"Ada\""

    // ---- Built-in macros in action ----
    println!("file = {}, line = {}", file!(), line!());
    println!("APP_VERSION from macro = {}", APP_VERSION);
    // env! takes only one argument (the variable name). It panics at
    // compile time if the variable is unset. For optional env vars, use
    // option_env! which returns Option<&str>.
    println!(
        "env CARGO_PKG_NAME at compile time = {:?}",
        option_env!("CARGO_PKG_NAME").unwrap_or("(not set — run via cargo)")
    );

    // ---- matches! is super handy ----
    let opt: Option<i32> = Some(5);
    if matches!(opt, Some(x) if x > 3) {
        println!("matches! says: Some(x) where x > 3");
    }

    // ---- todo! / unimplemented! / unreachable! ----
    // All three panic at runtime when reached. They're useful as placeholders
    // while you're building out a function:
    //
    //   fn not_done_yet() -> i32 {
    //       todo!("compute this later")
    //   }
    //
    //   match direction {
    //       Direction::Up    => go_up(),
    //       Direction::Down  => go_down(),
    //       Direction::Left  => go_left(),
    //       Direction::Right => go_right(),
    //       _ => unreachable!("we covered all 4 directions above"),
    //   }
    //
    // We don't call them here so the program exits cleanly, but you can
    // try them yourself by uncommenting the next line:
    // let _placeholder: i32 = todo!("compute this later");
}

// ========================================================================
//  Why macros exist (when to use a macro vs a function)
// ========================================================================
//
//  Use a FUNCTION when:
//    * You have a fixed number of arguments.
//    * The arguments have known types (or you can use generics).
//    * You want the function to be a normal first-class value.
//
//  Use a MACRO when:
//    * You need a VARIABLE number of arguments (println!, vec!, format!).
//    * You need to operate on the SYNTAX itself (e.g. stringify!, file!).
//    * You need to generate new identifiers / types / impls at compile time.
//    * You need to evaluate arguments lazily or only once.
//    * You need custom DSLs (e.g. sqlx::query!, axum routes).
//
//  Macros are MORE powerful than functions, but they:
//    * Are harder to read and debug.
//    * Don't have first-class types (they generate code).
//    * Have worse error messages.
//    * Cannot be stored in variables or passed around.
//
//  Rule of thumb: prefer functions. Reach for macros only when functions
//  genuinely cannot do what you need.
// ========================================================================
