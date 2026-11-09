// ============================================================================
//  CHAPTER 29 — RUST TRAITS
//  Traits describe a set of methods a type must implement. They are Rust's
//  answer to interfaces / type classes — the cornerstone of abstraction.
// ============================================================================

// ---- Defining a trait ---------------------------------------------------
// A trait is a contract: any type that implements it must provide these methods.
trait Summary {
    // Required method — implementing types MUST provide it:
    fn summarize(&self) -> String;

    // Default method — implementing types get this for free, but can override:
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..self.summarize().len().min(20)])
    }
}

// ---- Two types that will implement Summary ------------------------------
#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    text: String,
}

// ---- Implementing the trait for Article ---------------------------------
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// ---- Implementing the trait for Tweet (with override of preview) --------
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }

    fn preview(&self) -> String {
        // Tweets are short — just show the whole thing
        self.summarize()
    }
}

// ============================================================================
//  Trait bounds — generics that require certain capabilities
// ============================================================================
// "For any type T that implements Summary, return its summary."
fn announce<T: Summary>(item: &T) -> String {
    format!("BREAKING: {}", item.summarize())
}

// Multiple bounds with `+`:
fn announce_with_preview<T: Summary + std::fmt::Debug>(item: &T) -> String {
    format!("{:?} -> {}", item, item.preview())
}

// `impl Trait` syntax — sugar for a generic with a trait bound:
fn announce_short(item: &impl Summary) -> String {
    item.summarize()
}

// ============================================================================
//  Trait objects — dynamic dispatch via `dyn Trait`
// ============================================================================
// When you don't know the concrete type at compile time, you can use a
// `Box<dyn Trait>`, `&dyn Trait`, or `Rc<dyn Trait>`. This uses vtable
// dispatch (like virtual methods in C++).
fn print_all(items: &[Box<dyn Summary>]) {
    for item in items {
        println!("- {}", item.summarize());
    }
}

// ============================================================================
//  Display trait — let your type print nicely with `{}`
// ============================================================================
struct Kelvin(f64);

impl std::fmt::Display for Kelvin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The `+` flag means show the + sign for positives, etc.
        write!(f, "{:.2} K", self.0)
    }
}

// ============================================================================
//  Derivable traits — the compiler writes the impl for you
// ============================================================================
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

// ============================================================================
//  Associated types — a trait can declare a placeholder type
// ============================================================================
trait Container {
    type Item;                          // associated type
    fn first(&self) -> Option<&Self::Item>;
}

struct NumberList(Vec<i32>);

impl Container for NumberList {
    type Item = i32;                    // concrete choice
    fn first(&self) -> Option<&i32> {
        self.0.first()
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust 2.0 Released"),
        author: String::from("Ada"),
        content: String::from("Long content..."),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        text: String::from("Hello, world!"),
    };

    // ---- Trait methods are called just like regular methods --------------
    println!("{}", article.summarize());
    println!("{}", article.preview());     // uses default impl
    println!("{}", tweet.summarize());
    println!("{}", tweet.preview());       // uses overridden impl

    // ---- Trait bounds in action -----------------------------------------
    println!("{}", announce(&article));
    println!("{}", announce(&tweet));
    println!("{}", announce_with_preview(&article));
    println!("{}", announce_short(&tweet));

    // ---- Trait objects: a heterogeneous collection -----------------------
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Article {
            title: String::from("News 1"),
            author: String::from("Bob"),
            content: String::from("..."),
        }),
        Box::new(Tweet {
            username: String::from("user"),
            text: String::from("hi"),
        }),
    ];
    print_all(&items);

    // ---- Display trait in action ----------------------------------------
    let temp = Kelvin(273.15);
    println!("Absolute zero is {}", temp);   // uses Display

    // ---- Derivable traits in action -------------------------------------
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();                     // Clone
    println!("{:?}", p1);                    // Debug
    println!("p1 == p2? {}", p1 == p2);      // PartialEq

    // ---- Associated type in action --------------------------------------
    let list = NumberList(vec![10, 20, 30]);
    if let Some(first) = list.first() {
        println!("first = {}", first);
    }
}
