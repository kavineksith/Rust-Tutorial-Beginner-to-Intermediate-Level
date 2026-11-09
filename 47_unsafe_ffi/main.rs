// ============================================================================
//  CHAPTER 49 — RUST unsafe & FFI
//  `unsafe` lets you do things the compiler can't verify: raw pointers,
//  calling C functions, mutable globals, etc. This chapter covers the
//  5 superpowers + how to call C from Rust (and vice versa).
// ============================================================================

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// ============================================================================
//  1. Dereferencing raw pointers
// ============================================================================
//  Raw pointers `*const T` and `*mut T`:
//    * Don't enforce borrow rules (you can have multiple mutable pointers).
//    * Can be null.
//    * Don't auto-deref — you need `unsafe` to read/write through them.
//    * Can be sent between threads without `Send`/`Sync` checks.
//
//  Creating a raw pointer is SAFE. USING one is `unsafe`.
fn raw_pointers() {
    let mut x = 42;
    let r1: *const i32 = &x;        // safe: creating a raw pointer
    let r2: *mut i32 = &mut x;      // safe: creating a mutable raw pointer

    unsafe {
        // unsafe block: now we can dereference
        println!("  *r1 = {}", *r1);
        *r2 = 99;
        println!("  *r2 after write = {}", *r2);
    }
}

// ============================================================================
//  2. Calling unsafe functions and methods
// ============================================================================
//  Some functions are marked `unsafe` because they have contracts the
//  compiler can't verify. Example: `std::slice::from_raw_parts`.
unsafe fn dangerous() -> i32 {
    42
}

fn call_unsafe_fn() {
    let n = unsafe { dangerous() };
    println!("  dangerous() = {}", n);
}

// A real example: split a slice into two without checking bounds.
// You must GUARANTEE that `mid <= len`.
fn split_unsafe(slice: &[i32], mid: usize) -> (&[i32], &[i32]) {
    let ptr = slice.as_ptr();
    let len = slice.len();
    assert!(mid <= len);
    unsafe {
        (
            std::slice::from_raw_parts(ptr, mid),
            std::slice::from_raw_parts(ptr.add(mid), len - mid),
        )
    }
}

// ============================================================================
//  3. Implementing unsafe traits
// ============================================================================
//  Some traits are `unsafe` to implement because their correctness can't
//  be verified by the compiler. Examples: `Send`, `Sync`, and any trait
//  marked with `unsafe` in its declaration.
//
//  You almost never implement `Send` / `Sync` manually — the compiler
//  auto-derives them. But you might for types wrapping raw pointers.
struct MyRawType {
    ptr: *mut i32,
}

// SAFETY: MyRawType is safe to send between threads because the pointer
// points to memory that is uniquely owned and not shared.
unsafe impl Send for MyRawType {}

// ============================================================================
//  4. Accessing mutable static variables
// ============================================================================
//  Mutable statics are inherently unsafe in multi-threaded code — there's
//  no synchronization. Reading/writing them requires `unsafe`.
static mut COUNTER: u32 = 0;

fn increment_counter() {
    // In edition 2024, both reading AND writing a `static mut` through a
    // reference is unsafe. We copy the value into a local first, then
    // print the local (which doesn't involve the static).
    unsafe {
        COUNTER += 1;
        let current = COUNTER;   // read into a local
        println!("  COUNTER = {}", current);
    }
}

// Better: use atomics for thread-safe counters (no `unsafe` needed):
use std::sync::atomic::{AtomicU32, Ordering};
static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);

fn increment_safe() {
    let old = SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("  SAFE_COUNTER: {} -> {}", old, old + 1);
}

// ============================================================================
//  5. Accessing union fields
// ============================================================================
//  Unions are like C unions — multiple fields share the same memory.
//  Reading from a union is `unsafe` because the compiler can't know which
//  variant is currently valid.
#[repr(C)]
union MyUnion {
    as_int: i32,
    as_float: f32,
}

fn union_demo() {
    let u = MyUnion { as_int: 0x40490FDB };   // bits of 3.14159...
    unsafe {
        // Reading as_float is unsafe: we're reinterpreting the bits.
        println!("  as_int = {}, as_float = {}", u.as_int, u.as_float);
    }
}

// ============================================================================
//  FFI — Calling C functions from Rust
// ============================================================================
//  Rust can call C functions via `extern "C"` blocks. We declare the
//  signature; the linker resolves the symbol.
//
//  This file declares a few std C functions so it links without external
//  dependencies on most systems.
//  NOTE: In edition 2024, `extern` blocks must be marked `unsafe` because
//  declaring an extern item is a claim about the external code's behavior
//  that the compiler cannot verify.
unsafe extern "C" {
    // The C `abs` function from stdlib.h
    fn abs(x: c_int) -> c_int;
}

fn ffi_demo() {
    let n: c_int = -42;
    let absolute;
    unsafe {
        // Calling extern functions is `unsafe` — the compiler can't verify
        // the function's contract or that the symbol exists.
        absolute = abs(n);
    }
    println!("  abs({}) = {}", n, absolute);
}

// ============================================================================
//  Reversing: making a Rust function callable from C
// ============================================================================
//  Mark your function with `#[no_mangle]` and `extern "C"` so C can find
//  and call it. This is how Rust libraries expose APIs to C.
//
//  (Commented out to avoid linker warnings about no_mangle in a binary;
//  uncomment in a `cdylib` crate to actually export.)
//
//  /// # Safety
//  /// Returns the square of `x`. Has no invariants.
//  #[no_mangle]
//  pub extern "C" fn rust_square(x: c_int) -> c_int {
//      x * x
//  }

// ============================================================================
//  Passing strings between Rust and C
// ============================================================================
//  C strings are null-terminated byte arrays. Rust strings are NOT.
//  Use `CString` (owned, null-terminated) and `CStr` (borrowed view).
fn c_strings() {
    // Rust -> C: CString
    let rust_string = String::from("hello, C!");
    let c_string = CString::new(rust_string).unwrap();   // adds null terminator
    let c_ptr: *const c_char = c_string.as_ptr();         // raw *const c_char

    // C -> Rust: CStr
    unsafe {
        let cstr = CStr::from_ptr(c_ptr);
        // Convert to Rust &str (validates UTF-8):
        let rust_str = cstr.to_str().unwrap();
        println!("  round-trip: {}", rust_str);
    }

    // CString ensures no internal null bytes:
    let bad = CString::new("hello\0world");
    println!("  CString with embedded null: {:?}", bad.err().map(|e| e.to_string()));
}

fn main() {
    println!("=== 1. Raw pointers ===");
    raw_pointers();

    println!("\n=== 2. Calling unsafe functions ===");
    call_unsafe_fn();
    let (left, right) = split_unsafe(&[1, 2, 3, 4, 5], 3);
    println!("  split: {:?} | {:?}", left, right);

    println!("\n=== 3. Mutable statics ===");
    increment_counter();
    increment_counter();

    println!("\n=== 4. Atomic counter (no unsafe needed) ===");
    increment_safe();
    increment_safe();

    println!("\n=== 5. Unions ===");
    union_demo();

    println!("\n=== 6. FFI — calling C functions ===");
    ffi_demo();

    println!("\n=== 7. C strings ===");
    c_strings();

    // ========================================================================
    //  THE 5 unsafe SUPERPOWERS
    // ========================================================================
    //
    //  1. Dereference a raw pointer (`*const T` or `*mut T`)
    //  2. Call an unsafe function or method (including extern fns)
    //  3. Implement an unsafe trait (Send, Sync, etc.)
    //  4. Access or modify a mutable static variable
    //  5. Access a field of a union
    //
    //  ========================================================================
    //  WHEN TO USE unsafe
    //  ========================================================================
    //
    //  * Calling C libraries (FFI).
    //  * Implementing low-level data structures (Vec, HashMap internals).
    //  * Talking to the OS (syscalls, hardware registers).
    //  * Performance (rarely needed — Rust is already fast).
    //  * Writing a safe wrapper around a raw API.
    //
    //  ========================================================================
    //  WHEN NOT TO USE unsafe
    //  ========================================================================
    //
    //  * "I think the borrow checker is wrong" — usually it isn't.
    //  * "I want it to be faster" — profile first; the safe version is
    //    usually already optimal.
    //  * To silence a borrow error — there's almost always a safe refactor.
    //
    //  ========================================================================
    //  RULES FOR WRITING SAFE WRAPPERS
    //  ========================================================================
    //
    //  The standard pattern: keep all `unsafe` private, expose a safe API.
    //
    //  pub struct MySafeWrapper { /* private: */ raw: *mut Something }
    //
    //  impl MySafeWrapper {
    //      pub fn new() -> Self { /* internally uses unsafe */ }
    //      pub fn get(&self) -> i32 { /* internally uses unsafe */ }
    //  }
    //
    //  The invariant: a value of type MySafeWrapper can NEVER be in an
    //  invalid state. The unsafe code is hidden behind safe methods.
    //
    //  Document the SAFETY contracts of every unsafe block with a comment:
    //
    //  unsafe {
    //      // SAFETY: `ptr` was obtained from a valid &T, so it's non-null
    //      // and properly aligned. We hold a shared borrow for the entire
    //      // duration of this call.
    //      *ptr
    //  }
    //  ========================================================================
}
