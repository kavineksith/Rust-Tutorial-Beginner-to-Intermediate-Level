# Standard Library Tour

A quick reference to the most useful modules in `std`. Use this as a "table
of contents" — each module has many more functions than listed here.

## Top-Level Types (in the prelude)

These are always in scope — no `use` needed:

```rust
Vec, String, Option, Result, Box, ToString, ToOwned, Clone, Copy,
Debug, Display, Drop, Eq, Hash, Ord, PartialEq, PartialOrd, Iterator,
Send, Sync, Sized, Sync
```

## `std::env` — Environment

```rust
use std::env;

env::args()                          // iterator over CLI args
env::args_os()                       // same, but OsString (handles invalid UTF-8)
env::vars()                          // iterator over env vars
env::var("KEY")                      // Result<String, VarError>
env::var_os("KEY")                   // Option<OsString>
env::set_var("KEY", "value")
env::remove_var("KEY")
env::current_dir()                   // Result<PathBuf>
env::set_current_dir(path)
env::temp_dir()                      // PathBuf to system temp dir
env::home_dir()                      // (deprecated — use the `dirs` crate)
```

## `std::fs` — File System

```rust
use std::fs;

fs::read_to_string(path)?            // -> String
fs::read(path)?                      // -> Vec<u8>
fs::write(path, content)?
fs::copy(src, dst)?                  // -> u64 (bytes copied)
fs::rename(old, new)?
fs::remove_file(path)?
fs::create_dir(path)?
fs::create_dir_all(path)?            // recursive
fs::remove_dir(path)?                // empty dir only
fs::remove_dir_all(path)?            // recursive
fs::read_dir(path)?                  // iterator of DirEntry
fs::metadata(path)?                  // file info
fs::File::open(path)?                // file handle for reading
fs::File::create(path)?              // file handle for writing
fs::OpenOptions::new().append(true).open(path)?
```

## `std::io` — Input / Output

```rust
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};

io::stdin()                          // stdin handle
io::stdout()                         // stdout handle
io::stderr()                         // stderr handle

// Reading:
let mut s = String::new();
io::stdin().read_line(&mut s)?;

let mut buf = Vec::new();
io::stdin().read_to_end(&mut buf)?;

// Buffering:
let reader = BufReader::new(File::open("f")?);
for line in reader.lines() {
    let line = line?;
}

// Writing:
let mut writer = BufWriter::new(File::create("f")?);
writer.write_all(b"data")?;
writer.flush()?;

// Traits: Read, Write, BufRead — implement on your own types too.
```

## `std::path` — Paths

```rust
use std::path::{Path, PathBuf};

let p = Path::new("/tmp/foo.txt");
p.file_name()                        // Option<&OsStr>
p.file_stem()                        // Option<&OsStr> (without extension)
p.extension()                        // Option<&OsStr>
p.parent()                           // Option<&Path>
p.exists()                           // bool
p.is_file()                          // bool
p.is_dir()                           // bool

// Joining (cross-platform):
let full: PathBuf = Path::new("/tmp").join("foo").join("bar.txt");
let owned: PathBuf = PathBuf::from("/tmp");
```

## `std::process` — Processes

```rust
use std::process::{Command, Stdio};

let output = Command::new("echo")
    .arg("hello")
    .output()?;                       // captures stdout/stderr
println!("{}", String::from_utf8_lossy(&output.stdout));

let status = Command::new("ls")
    .arg("-l")
    .status()?;                       // inherits stdin/stdout/stderr

// Pipe stdin:
let mut child = Command::new("grep")
    .arg("rust")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
child.stdin.take().unwrap().write_all(b"rust is great\n")?;
let out = child.wait_with_output()?;

process::exit(0);                    // exit immediately
process::id()                        // current PID
```

## `std::collections` — Collections

```rust
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap, LinkedList};

HashMap::new()                       // O(1) lookup, unordered
BTreeMap::new()                      // O(log n), sorted
HashSet::new()                       // O(1) unique set
BTreeSet::new()                      // O(log n) sorted set
VecDeque::new()                      // O(1) push/pop both ends
BinaryHeap::new()                    // max-heap, O(log n) push/pop
LinkedList::new()                    // rarely useful — prefer Vec
```

## `std::sync` — Synchronization

```rust
use std::sync::{Arc, Mutex, RwLock, mpsc};

Arc::new(value)                      // atomic Rc (thread-safe shared ownership)
Mutex::new(value)                    // exclusive access
RwLock::new(value)                   // many readers OR one writer
mpsc::channel()                      // (tx, rx) — multiple producer, single consumer
mpsc::sync_channel(buf)              // bounded channel
Arc::clone(&arc)                     // bump refcount
```

## `std::thread` — Threads

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| { /* ... */ });
handle.join().unwrap();

thread::spawn(move || { /* capture by value */ });
thread::sleep(Duration::from_secs(1));
thread::current();
thread::yield_now();
```

## `std::time` — Time

```rust
use std::time::{Instant, Duration, SystemTime};

let start = Instant::now();
// ... do work ...
let elapsed: Duration = start.elapsed();

let now: SystemTime = SystemTime::now();
let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;

Duration::from_secs(60)
Duration::from_millis(500)
Duration::from_micros(10)
```

## `std::fmt` — Formatting

```rust
use std::fmt;

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyType({})", self.0)
    }
}

// Formatter supports precision, width, alignment, fill, sign:
write!(f, "{:>10}", "right")?;       // right-aligned, width 10
write!(f, "{:0>5}", 42)?;            // "00042"
write!(f, "{:.*}", 2, 3.14159)?;     // "3.14" (precision from arg)
```

## `std::mem` — Memory Utilities

```rust
use std::mem;

mem::size_of::<T>()                  // size in bytes
mem::size_of_val(&t)
mem::align_of::<T>()
mem::swap(&mut a, &mut b)            // swap two values
mem::replace(&mut dest, src)         // replace, return old
mem::take(&mut dest)                 // replace with Default::default(), return old
mem::drop(x)                         // force drop now
```

## `std::cmp` — Comparison

```rust
use std::cmp::Ordering;

1.cmp(&2)                            // Ordering::Less
std::cmp::min(1, 2)                  // 1
std::cmp::max(1, 2)                  // 2
std::cmp::min_by(a, b, |x, y| ...)
std::cmp::min_by_key(items, |x| x.cost)
```

## `std::convert` — Conversions

```rust
use std::convert::{From, Into, TryFrom, TryInto, AsRef, AsMut};

String::from("hi")                   // From
let s: String = "hi".into();         // Into
i32::try_from(big_i64)               // TryFrom
let n: Result<i32, _> = big_i64.try_into();
s.as_ref()                           // AsRef
```

## `std::ffi` — Foreign Function Interface

```rust
use std::ffi::{CStr, CString, OsStr, OsString};

CString::new("hi").unwrap()          // Rust String -> C string (null-terminated)
CStr::from_ptr(ptr)                  // C string -> &CStr
OsString::from("hi")                 // OS-native owned string
OsStr::new("hi")                     // OS-native borrowed string
```

## `std::cell` — Interior Mutability (Single-Thread)

```rust
use std::cell::{Cell, RefCell};

Cell::new(0)                         // Copy types only, no borrow check
RefCell::new(vec![])                 // runtime borrow check, any type

let c = Cell::new(5);
c.set(10);                           // &Cell can mutate
let v = c.get();

let r = RefCell::new(vec![1, 2, 3]);
r.borrow_mut().push(4);              // panics if already borrowed
```

## `std::rc` — Shared Ownership (Single-Thread)

```rust
use std::rc::{Rc, Weak};

let a = Rc::new(42);
let b = Rc::clone(&a);               // bump refcount
Rc::strong_count(&a);                // 2
let weak: Weak<i32> = Rc::downgrade(&a);   // non-owning reference
weak.upgrade()                       // Option<Rc<i32>>
```

## `std::net` — Networking

```rust
use std::net::{TcpListener, TcpStream, UdpSocket};

let listener = TcpListener::bind("127.0.0.1:8080")?;
for stream in listener.incoming() {
    let stream = stream?;
    // handle connection
}

let mut stream = TcpStream::connect("example.com:80")?;
stream.write_all(b"GET / HTTP/1.1\r\n\r\n")?;

let socket = UdpSocket::bind("0.0.0.0:0")?;
socket.send_to(b"hi", "127.0.0.1:5000")?;
```

## `std::os` — Platform-Specific

```rust
// Unix-only:
use std::os::unix::fs::PermissionsExt;
metadata.permissions().mode();       // file mode bits

use std::os::unix::process::CommandExt;
Command::new("ls").process_group(0);

// Windows-only:
use std::os::windows::fs::MetadataExt;
metadata.file_attributes();
```

## `std::marker` — Marker Traits

```rust
use std::marker::{Send, Sync, Copy, Sized, PhantomData};

// Auto-derived for most types:
fn send_if_safe<T: Send>(x: T) { /* ... */ }

// PhantomData: declare a type parameter without storing it
struct MyType<T> { _phantom: PhantomData<T> }
```

## `std::any` — Type Reflection

```rust
use std::any::{Any, TypeId};

let x: Box<dyn Any> = Box::new(42);
if let Some(n) = x.downcast_ref::<i32>() {
    println!("{}", n);   // 42
}

let id = TypeId::of::<i32>();
```

## `std::pin` — Pinning

```rust
use std::pin::Pin;

// Used for self-referential futures. You rarely write Pin yourself —
// Box::pin and async/await handle it.
let pinned = Box::pin(future);
```

## `std::future` / `std::task` — Async

```rust
use std::future::Future;
use std::task::{Context, Poll, Waker};

// Implement Future manually (rare — usually use async fn):
impl Future for MyFuture {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        Poll::Ready(42)
    }
}
```

## `std::ops` — Operator Traits

```rust
use std::ops::{Add, Sub, Mul, Div, Deref, Index, Range};

// Range:
for i in 1..5 { /* 1, 2, 3, 4 */ }
for i in 1..=5 { /* 1, 2, 3, 4, 5 */ }

// Implement Add on your type:
impl Add for Point { type Output = Point; fn add(self, other: Self) -> Self { ... } }

// Deref coercion:
impl Deref for Wrapper { type Target = Inner; fn deref(&self) -> &Self::Target { ... } }
```

## `std::result` / `std::option` — Result / Option

```rust
// Result:
let ok: Result<i32, &str> = Ok(5);
let err: Result<i32, &str> = Err("oops");
ok.unwrap();
ok.unwrap_or(0);
ok.map(|x| x * 2);
ok.and_then(|x| Ok(x + 1));

// Option:
let some = Some(5);
let none: Option<i32> = None;
some.unwrap();
some.map(|x| x * 2);
some.and_then(|x| Some(x + 1));
```

## `std::iter` — Iterators

```rust
use std::iter::{from_fn, once, repeat, successors, zip};

once(42)                                       // iterator yielding one value
repeat(0)                                      // infinite iterator of 0
zip([1, 2, 3].iter(), [4, 5, 6].iter())        // pairs of elements

// Generate from a function:
let mut n = 0;
let mut iter = from_fn(move || {
    n += 1;
    if n <= 5 { Some(n) } else { None }
});

// Successors:
let powers_of_2 = successors(Some(1), |&x| Some(x * 2)).take(10);
```

## Cheat Sheet

```rust
// The 90% you'll use daily:
use std::env;
use std::fs;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Instant, Duration};
use std::mem;
use std::convert::{From, Into, TryFrom};

// Format strings:
println!("{:?}", x);          // Debug
println!("{}", x);            // Display
println!("{:?}", vec);        // pretty-printed: {:#?}
println!("{:>10}", "hi");     // right-align
println!("{:.2}", 3.14159);   // 2 decimals

// Result / Option:
ok.map(|x| x * 2)?;
opt.and_then(|x| some_func(x))?;
opt.unwrap_or(default);
```

**Next:** [Idiomatic Rust & Best Practices](../50_idiomatic_rust) — style, pitfalls, ecosystem.
