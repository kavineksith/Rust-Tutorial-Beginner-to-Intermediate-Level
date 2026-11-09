// ============================================================================
//  CHAPTER 28 — RUST PRACTICE PROBLEMS
//  Each problem is a function. Solve them yourself, then check the answer
//  by running:   rustc main.rs  &&  ./main
// ============================================================================

fn main() {
    println!("=== PRACTICE PROBLEMS ===\n");

    // ------------------------------------------------------------------
    //  Problem 1: FizzBuzz
    //  Print numbers 1..=20. Multiples of 3 -> "Fizz", of 5 -> "Buzz",
    //  of both -> "FizzBuzz", otherwise the number.
    // ------------------------------------------------------------------
    println!("Problem 1 — FizzBuzz:");
    fizzbuzz(20);
    println!();

    // ------------------------------------------------------------------
    //  Problem 2: Reverse a string
    // ------------------------------------------------------------------
    println!("Problem 2 — Reverse string:");
    println!("  reverse(\"hello\") = {}", reverse("hello"));
    println!("  reverse(\"Rust🦀\") = {}", reverse("Rust🦀"));
    println!();

    // ------------------------------------------------------------------
    //  Problem 3: Check if a number is prime
    // ------------------------------------------------------------------
    println!("Problem 3 — Is prime:");
    for n in [1, 2, 3, 4, 17, 25, 29, 100] {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }
    println!();

    // ------------------------------------------------------------------
    //  Problem 4: Find the largest element in a slice
    // ------------------------------------------------------------------
    println!("Problem 4 — Largest:");
    let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("  largest = {}", largest(&nums));
    println!();

    // ------------------------------------------------------------------
    //  Problem 5: Count words in a sentence
    // ------------------------------------------------------------------
    println!("Problem 5 — Word count:");
    println!("  \"hello world rust\" -> {}", word_count("hello world rust"));
    println!();

    // ------------------------------------------------------------------
    //  Problem 6: Sum of even numbers in a vector
    // ------------------------------------------------------------------
    println!("Problem 6 — Sum of evens:");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("  sum of evens = {}", sum_evens(&v));
    println!();

    // ------------------------------------------------------------------
    //  Problem 7: Convert Celsius to Fahrenheit
    // ------------------------------------------------------------------
    println!("Problem 7 — Celsius to Fahrenheit:");
    println!("  0 C = {} F", celsius_to_fahrenheit(0.0));
    println!("  100 C = {} F", celsius_to_fahrenheit(100.0));
    println!();

    // ------------------------------------------------------------------
    //  Problem 8: Fibonacci (iterative and recursive)
    // ------------------------------------------------------------------
    println!("Problem 8 — Fibonacci:");
    for i in 0..10 {
        print!("{} ", fib_iter(i));
    }
    println!();
    println!("  fib_recursive(10) = {}", fib_recursive(10));
    println!();

    // ------------------------------------------------------------------
    //  Problem 9: Palindrome check
    // ------------------------------------------------------------------
    println!("Problem 9 — Palindrome:");
    for s in ["racecar", "hello", "level", "rust"] {
        println!("  \"{}\" -> {}", s, is_palindrome(s));
    }
    println!();

    // ------------------------------------------------------------------
    //  Problem 10: Use Option safely
    //  Find the first even number in a slice, returning Option<i32>.
    // ------------------------------------------------------------------
    println!("Problem 10 — First even (Option):");
    let a = [1, 3, 5, 7, 8, 9];
    let b = [1, 3, 5, 7, 9];
    println!("  first_even({:?}) = {:?}", a, first_even(&a));
    println!("  first_even({:?}) = {:?}", b, first_even(&b));
    println!();

    // ------------------------------------------------------------------
    //  Problem 11: Use Result for error handling
    //  Divide two integers, returning Result<i32, &str>.
    // ------------------------------------------------------------------
    println!("Problem 11 — Safe divide (Result):");
    println!("  divide(10, 2) = {:?}", divide(10, 2));
    println!("  divide(10, 0) = {:?}", divide(10, 0));
    println!();

    // ------------------------------------------------------------------
    //  Problem 12: Build a struct with methods
    // ------------------------------------------------------------------
    println!("Problem 12 — BankAccount struct:");
    let mut acc = BankAccount::new("Ada".to_string(), 100.0);
    acc.deposit(50.0);
    println!("  after deposit: {:?}", acc);
    match acc.withdraw(30.0) {
        Ok(balance) => println!("  after withdraw 30: balance = {}", balance),
        Err(msg)    => println!("  withdraw failed: {}", msg),
    }
    match acc.withdraw(1000.0) {
        Ok(balance) => println!("  after withdraw 1000: balance = {}", balance),
        Err(msg)    => println!("  withdraw failed: {}", msg),
    }
}

// ------------------------------------------------------------------
//  SOLUTIONS — try each problem yourself first, then peek below.
// ------------------------------------------------------------------

// Problem 1
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        let out = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => i.to_string(),
        };
        println!("  {}", out);
    }
}

// Problem 2 — reverse by chars (handles Unicode correctly)
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// Problem 3 — primality test
fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

// Problem 4 — generic largest works on any ordered type
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T {
    let mut max = slice[0];
    for &item in slice.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

// Problem 5 — count whitespace-separated words
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

// Problem 6 — sum of evens using iterator combinators
fn sum_evens(v: &[i32]) -> i32 {
    v.iter().filter(|x| *x % 2 == 0).sum()
}

// Problem 7
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Problem 8 — iterative (fast, O(n))
fn fib_iter(n: u32) -> u32 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

// Problem 8 — recursive (elegant but exponential — not for large n)
fn fib_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

// Problem 9 — palindrome check (case-insensitive, ignores non-letters)
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    cleaned.chars().eq(cleaned.chars().rev())
}

// Problem 10 — Option-based safe lookup
fn first_even(slice: &[i32]) -> Option<i32> {
    slice.iter().copied().find(|x| x % 2 == 0)
}

// Problem 11 — Result-based error handling
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

// Problem 12 — struct with methods
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, initial: f64) -> Self {
        BankAccount { owner, balance: initial }
    }
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: f64) -> Result<f64, &'static str> {
        if amount > self.balance {
            Err("insufficient funds")
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }
}
