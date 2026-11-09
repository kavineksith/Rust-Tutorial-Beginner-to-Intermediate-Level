# unsafe Rust & FFI

Rust's safety guarantees come from the compiler. `unsafe` lets you opt out
of those checks — to do things the compiler can't verify.

## The 5 `unsafe` Superpowers

1. **Dereference a raw pointer** (`*const T` or `*mut T`)
2. **Call an unsafe function or method** (including `extern` fns)
3. **Implement an unsafe trait** (`Send`, `Sync`, etc.)
4. **Access or modify a mutable `static` variable**
5. **Access a field of a `union`**

That's it. `unsafe` doesn't turn off the borrow checker, doesn't disable
overflow checks, doesn't allow UB. It just lets you do these 5 things.

## Raw Pointers

`*const T` and `*mut T` are like C pointers:

* Don't enforce borrow rules (multiple mutable pointers allowed).
* Can be null.
* Don't auto-deref — you need `unsafe` to read/write through them.
* Aren't `Send` / `Sync` by default.

```rust
let mut x = 42;
let r1: *const i32 = &x;        // safe: creating a raw pointer
let r2: *mut i32 = &mut x;

unsafe {
    println!("{}", *r1);   // unsafe: dereferencing
    *r2 = 99;
}
```

## Calling Unsafe Functions

```rust
unsafe fn dangerous() -> i32 { 42 }

let n = unsafe { dangerous() };
```

### Real Example: `slice::from_raw_parts`

```rust
fn split(slice: &[i32], mid: usize) -> (&[i32], &[i32]) {
    let ptr = slice.as_ptr();
    let len = slice.len();
    assert!(mid <= len);   // we must guarantee this
    unsafe {
        (
            std::slice::from_raw_parts(ptr, mid),
            std::slice::from_raw_parts(ptr.add(mid), len - mid),
        )
    }
}
```

## Implementing Unsafe Traits

Some traits are `unsafe` to implement because their correctness can't be
verified. Examples: `Send`, `Sync`.

```rust
struct MyType { ptr: *mut i32 }

// SAFETY: MyType owns its pointer uniquely and doesn't share it,
// so it's safe to send between threads.
unsafe impl Send for MyType {}
```

You almost never need to do this manually — the compiler auto-derives
`Send` / `Sync` for most types.

## Mutable Statics

```rust
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe { COUNTER += 1; }
}
```

Mutable statics are unsafe because there's no synchronization across
threads. **Prefer atomics** (`AtomicU32`, etc.) — they're safe and fast:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}
```

## Unions

Like C unions — multiple fields share the same memory:

```rust
#[repr(C)]
union MyUnion {
    as_int: i32,
    as_float: f32,
}

let u = MyUnion { as_int: 0x40490FDB };
unsafe {
    println!("{}", u.as_float);   // reinterprets the bits
}
```

Reading from a union is `unsafe` because the compiler can't know which
variant is currently valid.

## FFI — Calling C from Rust

Declare C functions with `extern "C"`:

```rust
use std::os::raw::c_int;

extern "C" {
    fn abs(x: c_int) -> c_int;
}

fn main() {
    let n = -42;
    let result = unsafe { abs(n) };
    println!("{}", result);   // 42
}
```

* `extern "C"` declares foreign functions following the C ABI.
* Calling them is `unsafe` — the compiler can't verify the contract.
* The linker resolves the symbols at link time.

### Common C Types

| C type | Rust type |
|--------|-----------|
| `int` | `std::os::raw::c_int` (`i32` on most platforms) |
| `char` | `std::os::raw::c_char` |
| `void*` | `*mut std::ffi::c_void` |
| `size_t` | `usize` |
| `const char*` | `*const c_char` |

### The `libc` Crate

For binding to the full C standard library, use the `libc` crate:

```toml
[dependencies]
libc = "0.2"
```

```rust
use libc::{c_int, getpid};

let pid = unsafe { getpid() };
```

## Calling Rust from C

Mark functions with `#[no_mangle]` and `extern "C"`:

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

* `#[no_mangle]` — don't mangle the symbol name (so C can find it).
* `extern "C"` — use the C calling convention.
* Build as a `cdylib` (Cargo.toml: `[lib] crate-type = ["cdylib"]`).

In C:

```c
// rust_lib.h
extern int rust_add(int a, int b);

// main.c
int main() {
    return rust_add(2, 3);
}
```

### Documenting Safety

```rust
/// # Safety
///
/// The caller must ensure that `ptr` points to a valid `i32` and that
/// the memory is not mutated for the duration of this call.
pub unsafe fn read_i32(ptr: *const i32) -> i32 {
    *ptr
}
```

## C Strings: `CString` and `CStr`

C strings are null-terminated byte arrays. Rust strings are NOT.

| Rust type | C equivalent | Use |
|-----------|--------------|-----|
| `CString` | `char*` (owned) | Rust → C (passing owned strings) |
| `CStr` | `const char*` (borrowed) | C → Rust (receiving strings) |

```rust
use std::ffi::{CStr, CString};

// Rust -> C
let rust_string = String::from("hello");
let c_string = CString::new(rust_string).unwrap();   // adds null terminator
let c_ptr = c_string.as_ptr();   // *const c_char
// Pass c_ptr to a C function.

// C -> Rust
unsafe {
    let cstr = CStr::from_ptr(c_ptr);
    let rust_str = cstr.to_str().unwrap();   // validates UTF-8
    println!("{}", rust_str);
}
```

`CString::new` returns an error if the string contains an internal null
byte (because C strings can't have those).

## Safe Wrappers — The Idiomatic Pattern

Keep all `unsafe` private, expose a safe API:

```rust
pub struct SafeFile {
    // private: raw pointer or other unsafe state
    raw: *mut std::ffi::c_void,
}

impl SafeFile {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        // ... internally uses unsafe to call C library ...
        Ok(SafeFile { raw: /* ... */ })
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        // ... internally uses unsafe ...
    }
}

impl Drop for SafeFile {
    fn drop(&mut self) {
        // ... close the file, free memory ...
    }
}
```

The invariant: a `SafeFile` value can NEVER be in an invalid state. The
unsafe code is hidden behind safe methods.

## Documenting Safety

Every `unsafe` block should have a `// SAFETY:` comment explaining why
it's safe:

```rust
unsafe {
    // SAFETY: `ptr` was obtained from a valid &T, so it's non-null and
    // properly aligned. We hold a shared borrow for the entire duration
    // of this call, so no mutation can happen concurrently.
    let value = *ptr;
}
```

For `unsafe fn`, document the contract in a `# Safety` section:

```rust
/// # Safety
///
/// - `ptr` must point to a valid `i32`.
/// - `ptr` must be properly aligned.
/// - The memory must not be mutated during this call.
pub unsafe fn read_i32(ptr: *const i32) -> i32 {
    *ptr
}
```

## When to Use `unsafe`

| Reason | When |
|--------|------|
| FFI | calling C libraries |
| Low-level data structures | implementing `Vec`, `HashMap` internals |
| OS interaction | syscalls, hardware registers |
| Performance | rare — profile first |
| Wrapping raw APIs | safe wrapper around unsafe code |

## When NOT to Use `unsafe`

* "I think the borrow checker is wrong" — usually it isn't.
* "I want it to be faster" — profile first; safe Rust is usually fast.
* To silence a borrow error — there's almost always a safe refactor.

## Tools for unsafe Code

| Tool | Purpose |
|------|---------|
| `cargo miri` | interpreter that detects UB in unsafe code |
| `loom` | testing concurrent unsafe code |
| AddressSanitizer / valgrind | runtime UB detection |
| `cargo-fuzz` | fuzz testing for unsafe parsers |

**Run Miri on your unsafe code.** It catches a LOT of bugs.

## FFI Best Practices

* Use the `libc` crate for standard C types.
* Wrap `unsafe` in a safe API immediately.
* Don't expose raw pointers in your public API.
* Document `# Safety` on every `unsafe fn`.
* Use `Drop` to free C-allocated resources.
* Be careful with exception/panic safety — `extern "C"` functions must
  not unwind across the FFI boundary. Use `catch_unwind` or
  `panic = "abort"`.

## Common Pitfalls

### Use-After-Free

```rust
let dangling;
{
    let x = 5;
    dangling = &x as *const i32;
}   // x is dropped here
unsafe {
    // println!("{}", *dangling);   // UB: use-after-free
}
```

### Null Pointer Dereference

```rust
let ptr: *const i32 = std::ptr::null();
unsafe {
    // println!("{}", *ptr);   // UB: null dereference
}
```

Always check `ptr.is_null()` before deref in unsafe code that might
receive nulls.

### Data Race on Mutable Static

```rust
static mut COUNTER: u32 = 0;
// Thread A: unsafe { COUNTER += 1; }
// Thread B: unsafe { COUNTER += 1; }
// UB: data race (no synchronization).
```

Use `AtomicU32` instead.

### Unwinding Through FFI

```rust
extern "C" fn callback() {
    panic!("oops");   // UB if called from C — panic crosses the FFI boundary
}
```

Use `catch_unwind` or set `panic = "abort"` in `Cargo.toml`.

## Cheat Sheet

```rust
// Raw pointer:
let ptr: *const i32 = &x;
unsafe { println!("{}", *ptr); }

// Unsafe fn:
unsafe fn foo() { /* ... */ }
unsafe { foo(); }

// extern "C":
extern "C" { fn abs(x: i32) -> i32; }
let n = unsafe { abs(-42) };

// Exporting to C:
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 { a + b }

// C strings:
let c = CString::new("hi").unwrap();
let r = unsafe { CStr::from_ptr(c.as_ptr()).to_str().unwrap() };
```

**Next:** [Standard Library Tour](../49_std_tour) — a quick reference to important std modules.
