// ============================================================================
//  CHAPTER 30 — RUST GENERICS
//  Generics let you write code that works on many types without duplication.
//  Rust uses "monomorphization" — the compiler creates a separate copy of
//  your generic code for each concrete type used. Result: zero runtime cost.
// ============================================================================

// ============================================================================
//  Generic FUNCTIONS
// ============================================================================
// `T` is a type parameter. `T: PartialOrd + Copy` is a trait bound: T must
// support comparison and be copyable.
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T {
    let mut max = slice[0];
    for &item in slice.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

// Multiple type parameters:
fn pair_to_string<A: std::fmt::Display, B: std::fmt::Display>(a: A, b: B) -> String {
    format!("({}, {})", a, b)
}

// ============================================================================
//  Generic STRUCTS
// ============================================================================
struct Point<T> {
    x: T,
    y: T,
}

// Multiple type parameters:
struct Pair<A, B> {
    first: A,
    second: B,
}

// ============================================================================
//  Methods on generic structs
// ============================================================================
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Methods can be restricted to specific type parameters:
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============================================================================
//  Generic ENUMS — the standard library is full of these
// ============================================================================
// `Option<T>` and `Result<T, E>` are generic enums. Let's define our own:
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

// A generic stack using Vec<T>:
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
}

// ============================================================================
//  Where clauses — for complex bounds
// ============================================================================
fn sum_iter<T, I>(iter: I) -> T
where
    T: std::ops::Add<Output = T> + Default,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().fold(T::default(), |acc, x| acc + x)
}

// ============================================================================
//  const generics — array size as a parameter
// ============================================================================
fn sum_array<T: Default + std::ops::Add<Output = T> + Copy, const N: usize>(arr: [T; N]) -> T {
    let mut total = T::default();
    for x in arr {
        total = total + x;
    }
    total
}

// ============================================================================
//  Generic with lifetime + trait bound (combining everything)
// ============================================================================
fn longest_in<'a, T: PartialOrd + Ord>(slice: &'a [T]) -> Option<&'a T> {
    slice.iter().max()
}

fn main() {
    // ---- Generic function with different types --------------------------
    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let chars = ['a', 'z', 'm', 'b'];
    let floats = [1.5, 2.5, 0.5];
    println!("largest int    = {}", largest(&nums));
    println!("largest char   = {}", largest(&chars));
    println!("largest float  = {}", largest(&floats));

    // ---- Multiple type params -------------------------------------------
    println!("{}", pair_to_string(1, "hello"));
    println!("{}", pair_to_string(3.14, true));

    // ---- Generic struct -------------------------------------------------
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 2.0 };
    // let mixed = Point { x: 1, y: 2.0 };   // ERROR: T must be the same!
    println!("int point x = {}", int_point.x());
    println!("float distance = {}", float_point.distance_from_origin());
    // int_point.distance_from_origin();   // ERROR: only impl'd for f64

    // ---- Pair with different types --------------------------------------
    let p = Pair { first: 42, second: String::from("answer") };
    println!("Pair: {} / {}", p.first, p.second);

    // ---- Generic enum ---------------------------------------------------
    let e1: Either<i32, &str> = Either::Left(42);
    let e2: Either<i32, &str> = Either::Right("hello");
    println!("{:?} {:?}", e1, e2);

    // ---- Generic Stack --------------------------------------------------
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack len = {}, empty? {}", stack.len(), stack.is_empty());
    while let Some(n) = stack.pop() {
        print!("{} ", n);          // 3 2 1
    }
    println!();

    // A stack of strings also works — same code, different type:
    let mut str_stack: Stack<&str> = Stack::new();
    str_stack.push("hello");
    str_stack.push("world");
    println!("str_stack: {:?}", str_stack.items);

    // ---- Where clause ---------------------------------------------------
    let s1 = sum_iter(vec![1, 2, 3, 4, 5]);
    let s2 = sum_iter(vec![1.5, 2.5, 3.0]);
    println!("sum int = {}, sum float = {}", s1, s2);

    // ---- const generics — works for any array size ----------------------
    let a3 = [1, 2, 3];
    let a5 = [10, 20, 30, 40, 50];
    println!("sum a3 = {}, sum a5 = {}", sum_array(a3), sum_array(a5));

    // ---- Generic + lifetime ---------------------------------------------
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some(max) = longest_in(&data) {
        println!("longest = {}", max);
    }

    // ======================================================================
    //  Monomorphization in action
    // ======================================================================
    //  Behind the scenes, the compiler generated THREE versions of `largest`:
    //    largest_i32(&[i32]) -> i32
    //    largest_char(&[char]) -> char
    //    largest_f64(&[f64]) -> f64
    //  This is why generics are zero-cost: there is no runtime lookup.
    // ======================================================================
}
