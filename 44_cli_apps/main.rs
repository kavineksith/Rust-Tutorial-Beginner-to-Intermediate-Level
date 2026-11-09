// ============================================================================
//  CHAPTER 46 — RUST COMMAND-LINE APPS
//  How to read CLI arguments, set exit codes, build polished CLIs.
//  This file uses only std (no clap dependency) so plain `rustc` works.
//  The notes show the idiomatic clap setup for real projects.
// ============================================================================

use std::env;
use std::process;

// ---- A simple argument parser using only std ----------------------------
//  For real projects, use `clap` (see notes.md). For small tools, std is
//  fine.
struct Args {
    program: String,
    name: String,
    count: usize,
    uppercase: bool,
    help: bool,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).cloned().unwrap_or_else(|| "greet".to_string());

    if args.len() < 2 {
        return Err(format!("Usage: {} [--uppercase] [--count N] <name>", program));
    }

    let mut name = String::new();
    let mut count: usize = 1;
    let mut uppercase = false;
    let mut help = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--uppercase" | "-u" => uppercase = true,
            "--help" | "-h" => help = true,
            "--count" | "-c" => {
                i += 1;
                if i >= args.len() {
                    return Err("--count requires a value".to_string());
                }
                count = args[i].parse().map_err(|_| "count must be a number")?;
            }
            arg if arg.starts_with("--") => {
                return Err(format!("unknown flag: {}", arg));
            }
            _ => {
                if name.is_empty() {
                    name = args[i].clone();
                } else {
                    return Err(format!("unexpected argument: {}", args[i]));
                }
            }
        }
        i += 1;
    }

    Ok(Args { program, name, count, uppercase, help })
}

const HELP_TEXT: &str = "\
greet 1.0 — A simple greeting tool

USAGE:
    greet [OPTIONS] <name>

OPTIONS:
    -u, --uppercase       Print the greeting in uppercase
    -c, --count <N>       Number of times to print (default: 1)
    -h, --help            Print this help message

EXAMPLES:
    greet Alice
    greet --uppercase --count 3 Bob
    greet -u -c 5 Carol
";

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {}", e);
            eprintln!("Run with --help for usage.");
            process::exit(2);   // convention: 2 = usage error
        }
    };

    if args.help {
        println!("{}", HELP_TEXT);
        process::exit(0);   // 0 = success
    }

    if args.name.is_empty() {
        eprintln!("error: name is required");
        process::exit(2);
    }

    let greeting = if args.uppercase {
        format!("HELLO, {}!", args.name.to_uppercase())
    } else {
        format!("Hello, {}!", args.name)
    };

    for _ in 0..args.count {
        println!("{}", greeting);
    }

    // ========================================================================
    //  EXIT CODES
    // ========================================================================
    //  * 0    -> success
    //  * 1    -> general failure
    //  * 2    -> usage error (wrong arguments)
    //  * 3..  -> application-specific failure codes
    //
    //  Returning Ok(()) from main sets exit code 0:
    //      fn main() -> Result<(), Box<dyn Error>> { ... Ok(()) }
    //
    //  Returning Err sets exit code 1 (Debug-prints the error).
    //
    //  For finer control, call process::exit(code) explicitly.
    // ========================================================================

    // ========================================================================
    //  ENVIRONMENT VARIABLES
    // ========================================================================
    //  Read with std::env::var:
    if let Ok(lang) = env::var("LANG") {
        eprintln!("(detected LANG={})", lang);
    }

    // Set an env var (for child processes):
    // NOTE: In edition 2024, env::set_var is unsafe because it can cause
    // data races in multi-threaded programs. We wrap it in unsafe {}.
    unsafe {
        env::set_var("GREET_VISITED", "1");
    }

    // Iterate over all env vars:
    // for (key, value) in env::vars() {
    //     println!("{}={}", key, value);
    // }
    //
    // ========================================================================
    //  SPAWNING CHILD PROCESSES (with std::process::Command)
    // ========================================================================
    //
    //  use std::process::Command;
    //
    //  let output = Command::new("echo")
    //      .arg("hello")
    //      .arg("world")
    //      .output()?;        // collects stdout/stderr
    //
    //  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    //  println!("exit code: {:?}", output.status.code());
    //
    //  // Stream output instead of collecting:
    //  let status = Command::new("ls")
    //      .arg("-l")
    //      .status()?;       // waits, returns ExitStatus
    //
    //  // Pipe stdin:
    //  use std::io::Write;
    //  let mut child = Command::new("grep")
    //      .arg("rust")
    //      .stdin(Stdio::piped())
    //      .stdout(Stdio::piped())
    //      .spawn()?;
    //  if let Some(mut stdin) = child.stdin.take() {
    //      stdin.write_all(b"rust is great\npython is also fine\n")?;
    //  }
    //  let output = child.wait_with_output()?;
    //  ========================================================================

    // ========================================================================
    //  EXITING FROM ANYWHERE
    // ========================================================================
    //  process::exit(code) terminates the program immediately. Drop handlers
    //  do NOT run (memory is reclaimed by the OS). For graceful shutdown,
    //  return from main instead.

    process::exit(0);
}

// ========================================================================
//  IDIOMATIC CLI WITH clap (NOT USED HERE — SEE notes.md)
// ========================================================================
//
//  For real projects, add clap as a dependency:
//
//    [dependencies]
//    clap = { version = "4", features = ["derive"] }
//
//  Then write:
//
//    use clap::Parser;
//
//    /// Greet a person by name.
//    #[derive(Parser, Debug)]
//    struct Args {
//        /// Name of the person to greet
//        name: String,
//
//        /// Number of times to greet
//        #[arg(short, long, default_value_t = 1)]
//        count: usize,
//
//        /// Print in uppercase
//        #[arg(short, long)]
//        uppercase: bool,
//    }
//
//    fn main() {
//        let args = Args::parse();
//        // ... use args.name, args.count, args.uppercase
//    }
//
//  clap gives you:
//    * Automatic --help and --version
//    * Type-checked arguments
//    * Subcommands (git-style)
//    * Shell completion generation
//    * Beautiful error messages
//    * Environment variable support
// ========================================================================
