// ============================================================================
//  CHAPTER 48 — RUST LOGGING & TRACING
//  Logging in Rust: the `log` crate (facade) + a backend like `env_logger`
//  or `tracing`. This file shows a tiny CUSTOM logger (compiles with std)
//  plus extensive notes on the real ecosystem.
// ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};

// ============================================================================
//  PART A — A minimal custom logger using only std
// ============================================================================
//  We define our own LogLevel and Logger to demonstrate the concept.
//  In real code, use the `log` crate (see notes.md) — but this shows what
//  happens under the hood.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Error = 0,
    Warn  = 1,
    Info  = 2,
    Debug = 3,
    Trace = 4,
}

impl LogLevel {
    fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn  => "WARN",
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

// A simple global logger. In real Rust, you'd use the `log` crate's
// `log::set_logger` machinery. Here we just use a function.
static MAX_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::Info as usize);

fn set_max_level(level: LogLevel) {
    MAX_LEVEL.store(level as usize, Ordering::Relaxed);
}

fn log(level: LogLevel, module: &str, message: &str) {
    let max = MAX_LEVEL.load(Ordering::Relaxed);
    if (level as usize) <= max {
        eprintln!("[{}] {}: {}", level.as_str(), module, message);
    }
}

// Convenience macros (simplified — real `log` crate uses macro_rules!)
macro_rules! log_error { ($($arg:tt)*) => { log(LogLevel::Error, module_path!(), &format!($($arg)*)) } }
macro_rules! log_warn  { ($($arg:tt)*) => { log(LogLevel::Warn,  module_path!(), &format!($($arg)*)) } }
macro_rules! log_info  { ($($arg:tt)*) => { log(LogLevel::Info,  module_path!(), &format!($($arg)*)) } }
macro_rules! log_debug { ($($arg:tt)*) => { log(LogLevel::Debug, module_path!(), &format!($($arg)*)) } }
macro_rules! log_trace { ($($arg:tt)*) => { log(LogLevel::Trace, module_path!(), &format!($($arg)*)) } }

// A pretend "business logic" function that uses logging:
fn process_data(input: &str) -> Result<usize, String> {
    log_debug!("starting to process {} bytes", input.len());

    if input.is_empty() {
        log_warn!("empty input received");
        return Err("input is empty".to_string());
    }

    let mut count = 0;
    for word in input.split_whitespace() {
        log_trace!("processing word: {}", word);
        count += 1;
    }

    log_info!("processed {} words", count);
    Ok(count)
}

fn main() {
    println!("=== Logging & Tracing (custom logger demo) ===\n");

    // Default level is Info — Debug and Trace are filtered out.
    println!("--- Default level (Info) ---");
    let _ = process_data("hello world from rust");
    let _ = process_data("");

    // Bump level to Debug to see more detail:
    println!("\n--- After set_max_level(Debug) ---");
    set_max_level(LogLevel::Debug);
    let _ = process_data("debug mode is on");

    // Bump to Trace to see EVERYTHING:
    println!("\n--- After set_max_level(Trace) ---");
    set_max_level(LogLevel::Trace);
    let _ = process_data("a b c");

    // Quiet down to Error only:
    println!("\n--- After set_max_level(Error) ---");
    set_max_level(LogLevel::Error);
    let _ = process_data("this won't be logged at info level");

    // ========================================================================
    //  PART B — REAL LOGGING SETUP (with the `log` + `env_logger` crates)
    // ========================================================================
    //
    //  Cargo.toml:
    //    [dependencies]
    //    log = "0.4"
    //    env_logger = "0.11"
    //
    //  src/main.rs:
    //
    //    use log::{info, warn, error, debug, trace};
    //
    //    fn main() {
    //        // Initialize the logger from the RUST_LOG env var.
    //        // Examples:
    //        //   RUST_LOG=info        -> all info+ logs
    //        //   RUST_LOG=debug       -> all debug+ logs
    //        //   RUST_LOG=my_crate=debug  -> only this crate at debug
    //        //   RUST_LOG=my_crate::network=trace  -> only one module
    //        env_logger::init();
    //
    //        info!("starting application");
    //        debug!("debug info: x = {}", 42);
    //        warn!("something suspicious");
    //        error!("something failed");
    //    }
    //
    //  Run with:
    //    RUST_LOG=info cargo run
    //    RUST_LOG=debug,hyper=warn cargo run   // this crate at debug, hyper at warn
    //
    //  ========================================================================
    //  STRUCTURED LOGGING (with `tracing`)
    //  ========================================================================
    //
    //  For async / structured / distributed tracing, use the `tracing` crate.
    //
    //  Cargo.toml:
    //    [dependencies]
    //    tracing = "0.1"
    //    tracing-subscriber = "0.3"
    //
    //  src/main.rs:
    //
    //    use tracing::{info, debug, span, Level};
    //
    //    fn main() {
    //        tracing_subscriber::fmt::init();
    //
    //        let span = span!(Level::INFO, "request", request_id = "abc123");
    //        let _enter = span.enter();
    //
    //        info!("handling request");
    //        debug!(?config, "loaded config");
    //    }
    //
    //  Output includes the span context:
    //    INFO request{request_id="abc123"}: handling request
    //    DEBUG request{request_id="abc123"}: loaded config config=...
    //
    //  ========================================================================
    //  WHEN TO USE `log` vs `tracing`
    //  ========================================================================
    //
    //  Use `log` (+ `env_logger`) when:
    //    * You're writing a small CLI or library.
    //    * You just need basic text logging.
    //    * You want zero overhead when logging is disabled.
    //
    //  Use `tracing` when:
    //    * You're writing a server or async app.
    //    * You need to trace a request across multiple functions/tasks.
    //    * You want structured output (JSON) for log aggregation.
    //    * You want spans with timing info (how long each function took).
    //
    //  The `tracing` crate is compatible with `log` — you can mix them.
    //
    //  ========================================================================
    //  LOG LEVELS — conventional meanings
    //  ========================================================================
    //
    //  ERROR  | something went wrong; the operation failed
    //  WARN   | something unexpected, but the operation continued
    //  INFO   | high-level progress (server started, request handled)
    //  DEBUG  | detailed info useful for diagnosing problems
    //  TRACE  | very detailed (every function entry/exit, every iteration)
    //
    //  Production typically runs at INFO. Debug builds at DEBUG. Deep
    //  investigations at TRACE.
    //
    //  ========================================================================
    //  LOGGING BEST PRACTICES
    //  ========================================================================
    //
    //  * Don't log and return an error — pick one. Logging inside a library
    //    forces the caller to deal with your logs; better to return the
    //    error and let them decide.
    //  * Don't log sensitive data (passwords, tokens, PII).
    //  * Use structured fields when possible: `info!(?user, "logged in")`.
    //  * Avoid logging in hot loops — it can dominate runtime.
    //  * Use `error!` for things you want to alert on; `warn!` for things
    //    to investigate; `info!` for normal operation.
    //  ========================================================================
}
