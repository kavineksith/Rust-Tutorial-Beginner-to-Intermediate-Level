// ============================================================================
//  CHAPTER 50 — RUST STANDARD LIBRARY TOUR
//  A quick reference to the most useful modules in std. Compile this file
//  to verify each import path; read it as a "table of contents" for std.
// ============================================================================

// ---- std::env — environment variables, args, current directory ---------
use std::env;

// ---- std::fs — file system operations ----------------------------------
use std::fs;

// ---- std::io — Read / Write / BufRead / stdin / stdout / stderr --------
use std::io::{self, BufRead, BufReader, Read, Write};

// ---- std::path — Path / PathBuf ----------------------------------------
use std::path::{Path, PathBuf};

// ---- std::process — spawning child processes, exit codes ---------------
use std::process;

// ---- std::collections — Vec, HashMap, HashSet, BTreeMap, VecDeque, etc.
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

// ---- std::sync — Mutex, RwLock, Arc, mpsc channels --------------------
use std::sync::{mpsc, Arc, Mutex, RwLock};

// ---- std::thread — spawning threads ------------------------------------
use std::thread;

// ---- std::time — Instant, Duration, SystemTime -------------------------
use std::time::{Duration, Instant, SystemTime};

// ---- std::fmt — Display, Debug, Formatter ------------------------------
use std::fmt;

// ---- std::mem — size_of, swap, replace, drop ---------------------------
use std::mem;

// ---- std::cmp — Ordering, min, max -------------------------------------
use std::cmp::Ordering;

// ---- std::convert — From, Into, TryFrom, AsRef -------------------------
use std::convert::{AsRef, From, TryFrom};

// ---- std::ffi — CString, CStr, OsString, OsStr -------------------------
use std::ffi::{CStr, CString, OsStr, OsString};

// ---- std::os — platform-specific extensions ----------------------------
// use std::os::unix::fs::PermissionsExt;     // Unix file modes
// use std::os::windows::process::CommandExt; // Windows-specific spawn options

// ---- std::ptr — null, null_mut, swap, replace --------------------------
// use std::ptr;

// ---- std::slice — slice operations -------------------------------------
// use std::slice;

// ---- std::str — string slice operations --------------------------------
// use std::str;

// ---- std::string — String (owned, growable) ----------------------------
// use std::string;

// ---- std::vec — Vec ----------------------------------------------------
// use std::vec;

// ---- std::result — Result<T, E> ----------------------------------------
// use std::result;

// ---- std::option — Option<T> -------------------------------------------
// use std::option;

// ---- std::iter — Iterator, iterator adaptors ---------------------------
// use std::iter;

// ---- std::ops — Range, Add, Sub, Deref, Index --------------------------
// use std::ops;

// ---- std::num — ParseIntError, ParseFloatError, IntErrorKind -----------
// use std::num;

// ---- std::error — Error trait ------------------------------------------
// use std::error;

// ---- std::cell — Cell, RefCell (interior mutability, single-thread) ----
// use std::cell;

// ---- std::rc — Rc, Weak (shared ownership, single-thread) --------------
// use std::rc;

// ---- std::marker — Send, Sync, Copy, PhantomData -----------------------
// use std::marker;

// ---- std::any — Any (type erasure via dyn Any) -------------------------
// use std::any;

// ---- std::pin — Pin (for self-referential futures) ---------------------
// use std::pin;

// ---- std::future — Future trait ----------------------------------------
// use std::future;

// ---- std::task — Poll, Context, Waker ----------------------------------
// use std::task;

// ---- std::net — TcpListener, TcpStream, UdpSocket ----------------------
// use std::net;

fn main() {
    println!("=== Standard Library Tour ===\n");

    // ======================================================================
    //  std::env
    // ======================================================================
    println!("std::env:");
    println!("  current dir: {:?}", env::current_dir().unwrap_or(PathBuf::from(".")));
    println!("  HOME env: {:?}", env::var("HOME").ok());
    let _args: Vec<String> = env::args().collect();
    println!("  args count: {}", _args.len());

    // ======================================================================
    //  std::path
    // ======================================================================
    println!("\nstd::path:");
    let p = Path::new("/tmp/foo.txt");
    println!("  file_name: {:?}", p.file_name());
    println!("  extension: {:?}", p.extension());
    println!("  parent: {:?}", p.parent());
    let joined: PathBuf = Path::new("/tmp").join("a").join("b.txt");
    println!("  joined: {}", joined.display());

    // ======================================================================
    //  std::collections (just a tiny demo of each)
    // ======================================================================
    println!("\nstd::collections:");
    let _v: Vec<i32>           = vec![1, 2, 3];
    let _vd: VecDeque<i32>     = VecDeque::new();
    let _ll: LinkedList<i32>   = LinkedList::new();
    let _hm: HashMap<&str, i32> = HashMap::new();
    let _bm: BTreeMap<&str, i32> = BTreeMap::new();
    let _hs: HashSet<i32>      = HashSet::new();
    let _bs: BTreeSet<i32>     = BTreeSet::new();
    let _bh: BinaryHeap<i32>   = BinaryHeap::new();
    println!("  all collections constructed");

    // ======================================================================
    //  std::io — stdin / stdout / stderr
    // ======================================================================
    println!("\nstd::io:");
    print!("  (write to stdout)"); io::stdout().flush().unwrap(); println!();
    eprintln!("  (write to stderr)");
    // Read a line:
    // let mut s = String::new();
    // io::stdin().read_line(&mut s).unwrap();

    // ======================================================================
    //  std::fs — file metadata
    // ======================================================================
    println!("\nstd::fs:");
    println!("  /tmp exists? {}", Path::new("/tmp").exists());

    // ======================================================================
    //  std::time — Instant / Duration / SystemTime
    // ======================================================================
    println!("\nstd::time:");
    let start = Instant::now();
    let _sum: u64 = (0..1000).sum();
    println!("  elapsed: {:?}", start.elapsed());
    let now = SystemTime::now();
    println!("  system time: {:?}", now.duration_since(SystemTime::UNIX_EPOCH));
    let _five_secs = Duration::from_secs(5);

    // ======================================================================
    //  std::thread + std::sync
    // ======================================================================
    println!("\nstd::thread + std::sync:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("  shared counter = {}", *counter.lock().unwrap());

    // mpsc channel:
    let (tx, rx) = mpsc::channel::<i32>();
    thread::spawn(move || { tx.send(42).unwrap(); });
    println!("  channel received: {:?}", rx.recv());

    // RwLock demo:
    let data = Arc::new(RwLock::new(0));
    {
        let _r1 = data.read().unwrap();
        let _r2 = data.read().unwrap();   // multiple readers OK
        println!("  RwLock: 2 concurrent readers");
    }
    {
        let mut _w = data.write().unwrap();   // exclusive writer
        *_w = 42;
    }

    // ======================================================================
    //  std::process — exit code
    // ======================================================================
    println!("\nstd::process:");
    let code = process::id();
    println!("  this process PID: {}", code);
    // To exit early: process::exit(0);
    // To run a child: Command::new("ls").status()

    // ======================================================================
    //  std::mem — size_of, swap, replace, drop
    // ======================================================================
    println!("\nstd::mem:");
    println!("  size_of::<i32>() = {}", mem::size_of::<i32>());
    println!("  size_of::<String>() = {}", mem::size_of::<String>());
    let mut a = 1; let mut b = 2;
    mem::swap(&mut a, &mut b);
    println!("  after swap: a={}, b={}", a, b);

    // ======================================================================
    //  std::cmp — Ordering
    // ======================================================================
    println!("\nstd::cmp:");
    println!("  1.cmp(&2) = {:?} (Less)", 1.cmp(&2));
    println!("  2.cmp(&2) = {:?} (Equal)", 2.cmp(&2));
    println!("  3.cmp(&2) = {:?} (Greater)", 3.cmp(&2));
    println!("  std::cmp::min(1, 2) = {}", std::cmp::min(1, 2));
    println!("  std::cmp::max(1, 2) = {}", std::cmp::max(1, 2));

    // ======================================================================
    //  std::convert — From / Into / AsRef / TryFrom
    // ======================================================================
    println!("\nstd::convert:");
    let s: String = String::from("hello");
    let borrowed: &str = s.as_ref();   // String -> &str via AsRef
    println!("  AsRef: {}", borrowed);
    let n: i32 = 42;
    let big: i64 = i64::from(n);
    println!("  From: i32 {} -> i64 {}", n, big);

    // ======================================================================
    //  std::ffi — CString / CStr / OsString / OsStr
    // ======================================================================
    println!("\nstd::ffi:");
    let c = CString::new("hello C").unwrap();
    println!("  CString: {:?}", c);
    let os: OsString = OsString::from("hello OS");
    let os_str: &OsStr = os.as_os_str();
    println!("  OsString: {:?}", os_str);

    // ======================================================================
    //  std::net — TCP / UDP (just declare, don't actually connect)
    // ======================================================================
    // use std::net::{TcpListener, TcpStream, UdpSocket};
    // let listener = TcpListener::bind("127.0.0.1:8080")?;
    // for stream in listener.incoming() {
    //     let stream = stream?;
    //     // handle connection
    // }

    // ========================================================================
    //  CHEAT SHEET — most-used std modules
    // ========================================================================
    //
    //  std::env           env vars, args, current_dir
    //  std::fs            file/dir operations
    //  std::io            Read/Write/BufRead, stdin/stdout/stderr
    //  std::path          Path, PathBuf
    //  std::process       spawn child processes, exit codes
    //  std::collections   Vec, HashMap, HashSet, BTreeMap, VecDeque, ...
    //  std::sync          Mutex, RwLock, Arc, mpsc channels
    //  std::thread        spawn OS threads
    //  std::time          Instant, Duration, SystemTime
    //  std::fmt           Display, Debug, Formatter
    //  std::mem           size_of, swap, replace, drop, take
    //  std::cmp           Ordering, min, max
    //  std::convert       From, Into, TryFrom, AsRef, AsMut
    //  std::ffi           CString, CStr, OsString, OsStr
    //  std::cell          Cell, RefCell (single-thread interior mutability)
    //  std::rc            Rc, Weak (single-thread shared ownership)
    //  std::marker        Send, Sync, Copy, PhantomData, Sized
    //  std::any           Any (type erasure)
    //  std::pin           Pin (for self-referential futures)
    //  std::future        Future trait
    //  std::task          Poll, Context, Waker
    //  std::net           TcpListener, TcpStream, UdpSocket
    //  std::os            platform-specific extensions
    //  std::ptr           raw pointer utilities
    //  std::slice         slice operations
    //  std::str           &str operations
    //  std::string        String (owned, growable)
    //  std::vec           Vec
    //  std::result        Result<T, E>
    //  std::option        Option<T>
    //  std::iter          Iterator, iterator adaptors
    //  std::ops           Range, Add, Sub, Mul, Deref, Index
    //  std::num           ParseIntError, NonZero*, Saturating
    //  std::error         Error trait
    // ========================================================================
}
