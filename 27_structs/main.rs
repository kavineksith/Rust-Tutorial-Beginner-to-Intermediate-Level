// ============================================================================
//  CHAPTER 26 — RUST STRUCTS
//  Structs let you create your own record types with named fields.
// ============================================================================

// ---- Classic named-field struct ----------------------------------------
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

// ---- Tuple struct (fields are unnamed, accessed by index) --------------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ---- Unit struct (no fields — useful as a marker type) -----------------
struct AlwaysEnabled;

// ---- Struct with derived traits ----------------------------------------
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Methods on a struct go in an `impl` block ------------------------------
impl Rectangle {
    // An ASSOCIATED FUNCTION (no `self`) — like a static method.
    // Called as `Rectangle::square(5.0)`.
    fn square(size: f64) -> Self {
        Rectangle { width: size, height: size }
    }

    // A METHOD takes `&self` (immutable borrow) — called as `r.area()`.
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // A method that takes `&mut self` (mutable borrow).
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // A method that takes `self` (consumes) — called as `r.into_square()`.
    fn into_square(self) -> Rectangle {
        let side = (self.width + self.height) / 2.0;
        Rectangle { width: side, height: side }
    }

    // A method that borrows another Rectangle:
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // ---- Creating a named-field struct (mutable so we can change fields) -
    let mut user1 = User {
        name: String::from("Ada"),
        age: 36,
        email: String::from("ada@example.com"),
        active: true,
    };
    println!("{} is {} years old", user1.name, user1.age);

    // Mutating a field:
    user1.age = 37;
    println!("now {} years old", user1.age);

    // ---- Struct update syntax (copy fields from another instance) -------
    let user2 = User {
        email: String::from("lovelace@example.com"),
        ..user1            // remaining fields come from user1
    };
    // Note: user1.name is a String — moved into user2. user1 is partially
    // moved, so we can only use its Copy fields (age, active) afterwards.
    println!("user2 = {} <{}>", user2.name, user2.email);
    println!("user1.age still works (Copy): {}", user1.age);

    // ---- Tuple structs ---------------------------------------------------
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
    println!("red = ({}, {}, {})", red.0, red.1, red.2);
    println!("origin = ({}, {}, {})", origin.0, origin.1, origin.2);
    // Color and Point are different types even though they have the same
    // field types — this prevents accidentally passing a Color where a
    // Point is expected.

    // ---- Unit struct -----------------------------------------------------
    let _marker = AlwaysEnabled;
    println!("AlwaysEnabled created (no data)");

    // ---- Using the Rectangle methods ------------------------------------
    let mut r1 = Rectangle { width: 30.0, height: 50.0 };
    println!("r1 area = {}", r1.area());      // 1500

    r1.scale(2.0);
    println!("after scale(2.0): {:?}", r1);   // 60 x 100

    let r2 = Rectangle::square(10.0);          // associated function call
    println!("r2 = {:?}, area = {}", r2, r2.area());

    let r3 = Rectangle { width: 5.0, height: 5.0 };
    println!("r1 can hold r3? {}", r1.can_hold(&r3));

    let square = r1.into_square();             // consumes r1
    println!("into_square: {:?}", square);
    // println!("{:?}", r1);                  // ERROR: r1 was moved

    // ---- Tuple-struct destructuring -------------------------------------
    let Color(r, g, b) = red;
    println!("destructured color: ({}, {}, {})", r, g, b);

    // ---- Printing structs ------------------------------------------------
    // `{:?}` requires the Debug trait — we derived it above. Without
    // `#[derive(Debug)]`, `println!("{:?}", r)` would not compile.
    let r4 = Rectangle { width: 1.0, height: 2.0 };
    println!("Debug: {:?}", r4);
    println!("Pretty: {:#?}", r4);
}
