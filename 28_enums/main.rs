// ============================================================================
//  CHAPTER 27 — RUST ENUMS
//  Enums let a value be ONE of several VARIANTS. They can carry data and
//  are pattern-matched exhaustively. `Option<T>` and `Result<T, E>` are
//  famous built-in enums.
// ============================================================================

// ---- A simple enum (no data) -------------------------------------------
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// ---- An enum where each variant carries data ---------------------------
enum Shape {
    Circle(f64),                       // single value
    Square(f64),
    Rectangle(f64, f64),               // tuple-like
    Triangle { a: f64, b: f64, c: f64 },   // struct-like
}

// ---- Deriving traits so we can print / compare -------------------------
#[derive(Debug, Clone, PartialEq)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// ---- Methods on enums (just like structs) ------------------------------
impl IpAddr {
    fn new_v4(a: u8, b: u8, c: u8, d: u8) -> Self {
        IpAddr::V4(a, b, c, d)
    }
    fn new_v6(addr: String) -> Self {
        IpAddr::V6(addr)
    }
    fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(127, _, _, _) => true,
            IpAddr::V6(s) if s == "::1" => true,
            _ => false,
        }
    }
}

// ---- The Option enum (built-in, no import needed) ----------------------
// enum Option<T> { Some(T), None }
// Used everywhere instead of null.

// ---- The Result enum (built-in) ----------------------------------------
// enum Result<T, E> { Ok(T), Err(E) }
// Used for fallible operations.

fn main() {
    // ---- Simple enum ----------------------------------------------------
    let dir = Direction::Up;
    describe_direction(dir);

    // ---- Enum carrying data --------------------------------------------
    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Square(3.0),
        Shape::Rectangle(2.0, 4.0),
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
    ];
    for s in &shapes {
        println!("area = {:.2}", area(s));
    }

    // ---- IpAddr methods -------------------------------------------------
    let home = IpAddr::new_v4(127, 0, 0, 1);
    let loopback_v6 = IpAddr::new_v6(String::from("::1"));
    let external = IpAddr::new_v4(192, 168, 1, 1);
    println!("{:?} loopback? {}", home, home.is_loopback());                 // true
    println!("{:?} loopback? {}", loopback_v6, loopback_v6.is_loopback());   // true
    println!("{:?} loopback? {}", external, external.is_loopback());         // false

    // ---- Option<T> — Rust's null replacement ---------------------------
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    println!("{:?} {:?}", some_number, no_number);

    // The compiler forces you to handle both Some and None.
    let result = match some_number {
        Some(n) => n * 2,
        None    => 0,
    };
    println!("result = {}", result);

    // `if let` — handle just one variant:
    if let Some(n) = some_number {
        println!("got a number: {}", n);
    }

    // Common Option methods:
    let unwrapped = some_number.unwrap();           // panics if None
    let safe = no_number.unwrap_or(0);              // 0
    let computed = no_number.unwrap_or_else(|| 1 + 1);  // 2
    let mapped = some_number.map(|n| n + 1);        // Some(43)
    let filtered = some_number.filter(|n| *n > 10); // Some(42)
    println!("unwrapped={} safe={} mapped={:?}", unwrapped, safe, mapped);
    println!("filtered={:?}", filtered);

    // ---- Result<T, E> — Rust's error type ------------------------------
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("something went wrong");
    println!("{:?} {:?}", ok, err);

    // A function that can fail:
    match parse_number("42") {
        Ok(n)  => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }
    match parse_number("not a number") {
        Ok(n)  => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }

    // The `?` operator — early return on Err (very common in functions):
    // fn read_file() -> Result<String, io::Error> {
    //     let mut f = File::open("path")?;   // returns Err if open fails
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;          // returns Err if read fails
    //     Ok(s)
    // }

    // `unwrap_or` for Results too:
    let n = parse_number("abc").unwrap_or(0);
    println!("fallback to 0: {}", n);

    // ---- Enums replace "null" -------------------------------------------
    // In other languages you might write `String name = null;`
    // In Rust: Option<String> forces you to handle the missing case.
    let maybe_name: Option<String> = None;
    let name = maybe_name.unwrap_or_else(|| String::from("anonymous"));
    println!("name = {}", name);

    // ---- Enums as state machines ----------------------------------------
    enum OrderState {
        Pending,
        Shipped { tracking: String },
        Delivered { signed_by: String },
        Cancelled(String),   // reason
    }
    let state = OrderState::Shipped { tracking: String::from("TRK123") };
    match state {
        OrderState::Pending => println!("still pending"),
        OrderState::Shipped { tracking } => println!("shipped, tracking: {}", tracking),
        OrderState::Delivered { signed_by } => println!("delivered to {}", signed_by),
        OrderState::Cancelled(reason) => println!("cancelled: {}", reason),
    }
}

fn describe_direction(dir: Direction) {
    match dir {
        Direction::Up    => println!("going up"),
        Direction::Down  => println!("going down"),
        Direction::Left  => println!("going left"),
        Direction::Right => println!("going right"),
    }
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r)              => std::f64::consts::PI * r * r,
        Shape::Square(side)           => side * side,
        Shape::Rectangle(w, h)        => w * h,
        Shape::Triangle { a, b, c }   => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("parse error: {}", e))
}
