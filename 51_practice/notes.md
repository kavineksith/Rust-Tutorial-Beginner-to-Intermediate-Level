# Rust Practice Problems

Test yourself on everything covered in this tutorial. Each problem comes
with a worked solution in `main.rs` — but **try them yourself first** before
peeking.

## How to Use This Chapter

1. Read the problem statement.
2. Write your solution in a separate file (or comment out the provided one).
3. Compile: `rustc main.rs && ./main`.
4. Compare your output with the expected output.
5. Compare your code with the provided solution.

---

## Problem 1 — FizzBuzz

Print numbers from 1 to 20. For each:

* multiples of 3 → print `Fizz`
* multiples of 5 → print `Buzz`
* multiples of both → print `FizzBuzz`
* otherwise → print the number

**Concepts practiced:** `for`, `match`, ranges, `%` operator.

---

## Problem 2 — Reverse a String

Write a function `fn reverse(s: &str) -> String` that returns the reversed
string. It must handle Unicode correctly (e.g. `reverse("Rust🦀")` should
return `🦀tsuR`).

**Concepts practiced:** `&str` vs `String`, `.chars()`, `.rev()`,
`.collect()`.

---

## Problem 3 — Is Prime?

Write `fn is_prime(n: u32) -> bool`. Handle `n < 2` correctly.

**Concepts practiced:** early returns, `while` loops, arithmetic.

---

## Problem 4 — Largest in a Slice

Write a **generic** `fn largest<T: PartialOrd + Copy>(slice: &[T]) -> T`.
It should work on `[i32]`, `[f64]`, `[char]`, ...

**Concepts practiced:** generics, traits, slices, `for` loops.

---

## Problem 5 — Word Count

Write `fn word_count(s: &str) -> usize` that counts whitespace-separated
words.

**Concepts practiced:** string methods, iterators, `.count()`.

---

## Problem 6 — Sum of Evens

Given `&[i32]`, return the sum of all even numbers. Use **iterator
combinators** (`filter`, `sum`).

**Concepts practiced:** iterator adaptors, closures.

---

## Problem 7 — Celsius to Fahrenheit

`fn celsius_to_fahrenheit(c: f64) -> f64` — formula: `F = C × 9/5 + 32`.

**Concepts practiced:** functions, floating point.

---

## Problem 8 — Fibonacci (Two Ways)

Implement Fibonacci **iteratively** (`fib_iter`) and **recursively**
(`fib_recursive`). Compare their performance for large `n`.

**Concepts practiced:** `loop` vs recursion, `match`, performance tradeoffs.

---

## Problem 9 — Palindrome

`fn is_palindrome(s: &str) -> bool`. Make it case-insensitive and ignore
non-alphanumeric characters (so `"A man, a plan, a canal: Panama"` returns
`true`).

**Concepts practiced:** iterator chains, char methods.

---

## Problem 10 — First Even (Option)

`fn first_even(slice: &[i32]) -> Option<i32>` — return the first even
number, or `None` if there isn't one.

**Concepts practiced:** `Option`, `.find()`, iterator adaptors.

---

## Problem 11 — Safe Divide (Result)

`fn divide(a: i32, b: i32) -> Result<i32, &'static str>` — return
`Err("cannot divide by zero")` when `b == 0`.

**Concepts practiced:** `Result`, error handling, `match` on results.

---

## Problem 12 — BankAccount Struct

Design a `BankAccount` struct with:

* fields: `owner: String`, `balance: f64`
* `new(owner, initial)` — constructor
* `deposit(amount)` — adds to balance
* `withdraw(amount) -> Result<f64, &'static str>` — returns new balance, or
  `Err("insufficient funds")`

Derive `Debug` so you can print it.

**Concepts practiced:** structs, `impl` blocks, `&mut self`, `Result`,
`#[derive(Debug)]`.

---

## Bonus Challenges

Once you finish the basic problems, try these:

### Bonus A — Word Frequency Counter

Read a sentence and print a `HashMap` of word → count, sorted by count
descending.

### Bonus B — Linked List (the hard way)

Implement a singly linked list of `i32` using `Box<Node>`. Implement `push`,
`pop`, and `len`. (This is famously tricky in Rust — a great learning
exercise.)

### Bonus C — CLI Tool

Write a small command-line tool that takes a filename argument and prints
the number of lines, words, and characters (like a tiny `wc`).

### Bonus D — Generic Min/Max Stack

Design a `MinStack<T>` that supports `push`, `pop`, and `min` (all O(1))
using two vectors.

---

## Where to Go Next

Now that you have finished the beginner-friendly style tutorial, dive deeper:

| Topic | Resource |
|-------|----------|
| The official book | [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Book") |
| Interactive exercises | [Rustlings](https://github.com/rust-lang/rustlings) |
| Deeper ownership / lifetimes | [Rust By Example](https://doc.rust-lang.org/rust-by-example/) |
| Async programming | [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) |
| Idiomatic patterns | [Rust Patterns](https://rust-unofficial.github.io/patterns/) |
| Practice problems | [Exercism Rust track](https://exercism.org/tracks/rust) |

Keep building, keep experimenting, and **welcome to the Rust community!** 🦀
