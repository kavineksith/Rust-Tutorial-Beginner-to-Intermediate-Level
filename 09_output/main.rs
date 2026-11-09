// ============================================================================
//  CHAPTER 04 — RUST OUTPUT
//  All the ways to print text to the terminal in Rust.
// ============================================================================

fn main() {
    // ---- Basic printing ---------------------------------------------------
    // `println!` — print + newline (the "ln" stands for "line")
    println!("Hello, world!");

    // `print!` — print WITHOUT a newline
    print!("Same ");
    print!("line.");
    println!(); // just a newline

    // ---- Placeholders `{}` ------------------------------------------------
    // `{}` is a placeholder filled by the arguments in order.
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);

    // Named placeholders (more readable):
    println!("{name} is {age} years old.");   // inline variable names
    println!("{name} again, age {age}", name = "Ada", age = 36);

    // ---- Positional placeholders ------------------------------------------
    println!("{0} vs {1} vs {0}", "first", "second");

    // ---- Debug formatting `{:?}` ------------------------------------------
    // `{:?}` prints a value using its `Debug` implementation.
    // Great for inspecting data structures during development.
    let fruits = vec!["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);

    // Pretty-printed debug `{:#?}` (multi-line, indented):
    println!("Pretty: {:#?}", fruits);

    // ---- Number formatting ------------------------------------------------
    let n = 3.14159;
    println!("{:.2}", n);          // 2 decimal places -> 3.14
    println!("{:08.2}", n);        // width 8, leading zeros -> 0003.14
    println!("{:>10}", "right");   // right-aligned, width 10
    println!("{:<10}|", "left");   // left-aligned, width 10
    println!("{:^10}", "center");  // centered

    // ---- Bases (binary / octal / hex) ------------------------------------
    let number = 255u32;
    println!("bin  : {:b}", number); // 11111111
    println!("oct  : {:o}", number); // 377
    println!("hex  : {:x}", number); // ff
    println!("HEX  : {:X}", number); // FF

    // ---- stderr with `eprintln!` / `eprint!` -----------------------------
    // These write to STANDARD ERROR instead of standard output.
    // Useful for log messages and errors that should not mix with normal
    // program output (e.g. when piping output to a file).
    eprintln!("This goes to stderr, not stdout.");

    // ---- `format!` builds a String instead of printing -------------------
    let s = format!("{}-{}-{}", 2024, 12, 25);
    println!("Formatted string: {}", s);

    // ---- Printing special characters -------------------------------------
    println!("Tab\tinside\ttext");        // \t = tab
    println!("Newline\ninside\ntext");     // \n = newline
    println!("Quote: \" and backslash: \\"); // \" \\ escapes
    println!("Unicode: \u{1F980}");        // 🦀 (crab emoji)
}
