// ============================================================================
//  CHAPTER 24 — RUST TUPLES
//  Tuples group a FIXED number of values, possibly of DIFFERENT types.
// ============================================================================

fn main() {
    // ---- Creating tuples -------------------------------------------------
    let pair: (i32, f64) = (1, 2.5);
    let triple = (1, "hello", 3.14);          // type inferred
    let person: (&str, u32, f64) = ("Ada", 36, 1.68);
    let unit: () = ();                        // the unit tuple
    println!("{:?} {:?} {:?} {:?}", pair, triple, person, unit);

    // ---- Accessing by index `.0`, `.1`, ... ------------------------------
    println!("name = {}", person.0);          // Ada
    println!("age = {}", person.1);           // 36
    println!("height = {}", person.2);        // 1.68

    // ---- Destructuring ---------------------------------------------------
    let (name, age, height) = person;
    println!("{} is {} years old and {}m tall", name, age, height);

    // Partial destructuring with `_` for values you don't need:
    let (name, _, _) = person;                // already moved above; just syntax
    println!("only need name: {}", name);

    // ---- Tuples in functions: returning multiple values -----------------
    let result = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={}, max={}", result.0, result.1);

    let (min, max) = min_max(&[10, 20, 5, 30]);
    println!("min={}, max={}", min, max);

    // ---- Nested tuples ---------------------------------------------------
    let nested: ((i32, i32), (i32, i32)) = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("a={}, b={}, c={}, d={}", a, b, c, d);

    // ---- Tuples with Vec / String / etc ---------------------------------
    let record = (String::from("Alice"), vec![90, 85, 88]);
    let (student, scores) = record;
    println!("{}: {:?}", student, scores);

    // ---- Tuple of length 1 (rare) ---------------------------------------
    // Single-element tuples need a trailing comma:
    let single: (i32,) = (5,);
    println!("single-element tuple: {:?}", single);

    // ---- The unit type `()` ---------------------------------------------
    // `()` is the empty tuple. It is the default return type of functions
    // that don't return anything. It is also the type of empty blocks `{}`.
    fn returns_nothing() -> () {
        println!("I return the unit value");
    }
    let u: () = returns_nothing();
    println!("u is the unit value: {:?}", u);

    // ---- Iterating over a tuple? Not directly ---------------------------
    // Tuples do NOT implement Iterator — they have a fixed, known structure.
    // You must destructure them to access each element.

    // ---- Pattern matching on tuples -------------------------------------
    let point = (3, -4);
    match point {
        (0, 0)         => println!("origin"),
        (x, 0)         => println!("on x-axis at {}", x),
        (0, y)         => println!("on y-axis at {}", y),
        (x, y) if x > 0 && y > 0 => println!("quadrant I ({}, {})", x, y),
        (x, y)         => println!("somewhere else ({}, {})", x, y),
    }

    // ---- Swapping values using tuples -----------------------------------
    let (mut a, mut b) = (1, 2);
    println!("before swap: a={}, b={}", a, b);
    (a, b) = (b, a);                          // destructuring assignment (Rust 1.59+)
    println!("after swap: a={}, b={}", a, b);
}

fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in numbers {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)                                // return a tuple
}
