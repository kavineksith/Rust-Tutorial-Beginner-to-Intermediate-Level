// ============================================================================
//  CHAPTER 21 — RUST DATA STRUCTURES (Overview)
//  A tour of Rust's built-in collection types. Each is covered in depth in
//  its own chapter (22 - 27).
// ============================================================================

fn main() {
    // ======================================================================
    //  ARRAYS  (chapter 22)
    //  Fixed-size, same-type, stored on the stack.
    // ======================================================================
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("Array: {:?}, first = {}", arr, arr[0]);

    // ======================================================================
    //  VECTORS  (chapter 23)
    //  Growable, heap-allocated, same-type.
    // ======================================================================
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);

    // ======================================================================
    //  TUPLES  (chapter 24)
    //  Fixed-size, mixed types.
    // ======================================================================
    let person: (&str, i32, f64) = ("Ada", 36, 1.68);
    println!("Tuple: {:?}, name = {}", person, person.0);

    // ======================================================================
    //  HASHMAP  (chapter 25)
    //  Key-value pairs, hashed lookup.
    // ======================================================================
    use std::collections::HashMap;
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("apple", 5);
    map.insert("banana", 3);
    println!("HashMap: {:?}", map);

    // ======================================================================
    //  STRUCTS  (chapter 26)
    //  Custom record type — named fields, your own type.
    // ======================================================================
    struct User {
        name: String,
        age: u32,
    }
    let user = User { name: String::from("Ada"), age: 36 };
    println!("Struct user: {} ({} years)", user.name, user.age);

    // ======================================================================
    //  ENUMS  (chapter 27)
    //  A type that can be one of several variants. Optional and Result are
    //  famous built-in enums.
    // ======================================================================
    enum Shape {
        Circle(f64),
        Square(f64),
        Rectangle(f64, f64),
    }
    let s = Shape::Circle(2.0);
    let area = match s {
        Shape::Circle(r)         => std::f64::consts::PI * r * r,
        Shape::Square(side)      => side * side,
        Shape::Rectangle(w, h)   => w * h,
    };
    println!("Shape area: {:.2}", area);

    // ======================================================================
    //  Choosing the right structure
    // ======================================================================
    //  Need a FIXED number of values of the SAME type? -> Array
    //  Need a GROWABLE list of the same type?             -> Vec
    //  Need a few values of DIFFERENT types?              -> Tuple
    //  Need a key -> value mapping?                       -> HashMap
    //  Need a record with named fields?                   -> Struct
    //  Need a value that can be one of several shapes?    -> Enum
}
