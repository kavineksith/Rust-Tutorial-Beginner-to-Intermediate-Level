# File I/O & std::fs

Rust's `std::fs` and `std::io` modules provide everything you need for
file and directory operations.

## The Two Layers

| Module | Purpose |
|--------|---------|
| `std::fs` | file/directory operations (open, read, write, list, etc.) |
| `std::io` | low-level reading/writing (`Read`, `Write`, `BufRead` traits) |
| `std::path` | cross-platform path manipulation (`Path`, `PathBuf`) |

## Reading a Whole File

```rust
use std::fs;

// Read into a String (text file):
let contents = fs::read_to_string("file.txt")?;

// Read into Vec<u8> (binary file):
let bytes = fs::read("file.bin")?;
```

These load the entire file into memory — fine for small files, bad for
multi-GB files.

## Writing a Whole File

```rust
use std::fs;

fs::write("output.txt", "hello\n")?;   // overwrites if exists
fs::write("output.bin", &[0u8, 1, 2, 3])?;
```

## Reading Line-by-Line (Memory Efficient)

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let file = File::open("file.txt")?;
let reader = BufReader::new(file);

for (i, line) in reader.lines().enumerate() {
    let line = line?;
    println!("[{}] {}", i, line);
}
```

`BufReader` adds buffering so each `.lines()` call doesn't make a syscall.

## Reading in Chunks

For very large files, read into a fixed-size buffer:

```rust
use std::fs::File;
use std::io::Read;

let mut file = File::open("large.bin")?;
let mut buffer = [0u8; 4096];

loop {
    let n = file.read(&mut buffer)?;   // bytes read this call
    if n == 0 { break; }               // EOF
    // process &buffer[..n]
}
```

## Creating and Writing Files

```rust
use std::fs::File;
use std::io::Write;

let mut file = File::create("output.txt")?;   // creates or truncates
file.write_all(b"hello\n")?;
file.write_all(b"world\n")?;
```

## Appending with `OpenOptions`

```rust
use std::fs::OpenOptions;
use std::io::Write;

let mut file = OpenOptions::new()
    .append(true)
    .open("log.txt")?;

file.write_all(b"new log entry\n")?;
```

### Common `OpenOptions`

```rust
OpenOptions::new()
    .read(true)              // open for reading
    .write(true)             // open for writing
    .create(true)            // create if doesn't exist
    .truncate(true)          // truncate if exists
    .append(true)            // append to end
    .open("file.txt")?;
```

## Copying, Renaming, Deleting

```rust
use std::fs;

fs::copy("src.txt", "dst.txt")?;          // returns bytes copied
fs::rename("old.txt", "new.txt")?;        // move/rename
fs::remove_file("file.txt")?;             // delete
```

## Directories

```rust
use std::fs;

fs::create_dir("dir")?;                   // create one dir
fs::create_dir_all("a/b/c")?;             // create whole tree
fs::remove_dir("dir")?;                   // remove empty dir
fs::remove_dir_all("dir")?;               // remove dir tree
```

## Listing Directory Contents

```rust
use std::fs;

for entry in fs::read_dir("/tmp")? {
    let entry = entry?;
    let path = entry.path();
    println!("{}", path.display());
}
```

Each `entry` is a `DirEntry` — call `.path()`, `.file_name()`, `.metadata()`.

## Walking a Directory Tree

No built-in recursive walker, but easy to write:

```rust
use std::fs;
use std::path::Path;

fn walk(dir: &Path, depth: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}{}", "  ".repeat(depth), path.display());
            if path.is_dir() {
                walk(&path, depth + 1)?;
            }
        }
    }
    Ok(())
}
```

For real projects, use the `walkdir` or `ignore` crates (more features,
better error handling).

## File Metadata

```rust
use std::fs;

let metadata = fs::metadata("file.txt")?;
println!("size: {}", metadata.len());
println!("is_file: {}", metadata.is_file());
println!("is_dir: {}", metadata.is_dir());
println!("read-only: {}", metadata.permissions().readonly());
```

### Platform-Specific Extensions

On Unix:

```rust
use std::os::unix::fs::PermissionsExt;
println!("mode: {:o}", metadata.permissions().mode());

fs::set_permissions("file.txt", fs::Permissions::from_mode(0o644))?;
```

On Windows:

```rust
use std::os::windows::fs::MetadataExt;
println!("attributes: {}", metadata.file_attributes());
```

## Paths: `Path` and `PathBuf`

`Path` is to paths what `str` is to strings — a borrowed slice. `PathBuf`
is the owned, growable version.

```rust
use std::path::{Path, PathBuf};

let p = Path::new("/tmp/foo/bar.txt");
println!("file_name: {:?}", p.file_name());        // Some("bar.txt")
println!("extension: {:?}", p.extension());        // Some("txt")
println!("parent: {:?}", p.parent());              // Some("/tmp/foo")
println!("exists: {}", p.exists());
println!("is_file: {}", p.is_file());
println!("is_dir: {}", p.is_dir());

// Joining paths:
let full: PathBuf = Path::new("/tmp").join("foo").join("bar.txt");

// Cross-platform:
let config = std::env::home_dir().unwrap().join(".config").join("myapp");
```

### `Path` vs `PathBuf`

| | `Path` | `PathBuf` |
|---|--------|-----------|
| Owned? | no (borrowed) | yes |
| Growable? | no | yes |
| Like | `&str` | `String` |
| Construct | `Path::new(s)` | `PathBuf::from(s)`, `s.to_path_buf()` |

## The `Read` and `Write` Traits

These traits abstract over anything you can read from / write to — files,
sockets, in-memory buffers, etc.

```rust
use std::io::{Read, Write};

fn copy_data<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 { break; }
        writer.write_all(&buf[..n])?;
    }
    Ok(())
}
```

## `BufReader` and `BufWriter`

Add buffering to any `Read`/`Write`:

```rust
use std::io::{BufReader, BufWriter};

let reader = BufReader::new(File::open("input.txt")?);
let writer = BufWriter::new(File::create("output.txt")?);
```

Always buffer when doing many small reads/writes — unbuffered I/O makes a
syscall per call.

## Standard Input / Output / Error

```rust
use std::io::{self, Read};

let mut input = String::new();
io::stdin().read_line(&mut input)?;
println!("you typed: {}", input.trim());

// Read entire stdin:
let mut buf = Vec::new();
io::stdin().read_to_end(&mut buf)?;
```

## Error Handling

All file operations return `io::Result<T>`. Use `?` to propagate:

```rust
fn read_config(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
```

For richer errors, see chapter 31 — wrap `io::Error` in a custom enum or
use `anyhow::Result`.

## Common Patterns

### Read a File or Use Default

```rust
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("default config"));
```

### Atomic Write (write to temp, rename)

```rust
use std::io::Write;

let temp_path = "output.tmp";
let final_path = "output.txt";

let mut f = File::create(temp_path)?;
f.write_all(content.as_bytes())?;
f.sync_all()?;            // ensure data is flushed to disk
fs::rename(temp_path, final_path)?;   // atomic on most filesystems
```

### Stream Process (line-by-line transform)

```rust
use std::io::{BufRead, BufReader, BufWriter, Write};

let input = File::open("in.txt")?;
let output = File::create("out.txt")?;
let reader = BufReader::new(input);
let mut writer = BufWriter::new(output);

for line in reader.lines() {
    let line = line?;
    let upper = line.to_uppercase();
    writeln!(writer, "{}", upper)?;
}
```

## `std::fs` Cheat Sheet

```rust
fs::read_to_string(path)?              // String
fs::read(path)?                        // Vec<u8>
fs::write(path, content)?              // overwrite
fs::copy(src, dst)?                    // copy file
fs::rename(old, new)?                  // move/rename
fs::remove_file(path)?                 // delete file
fs::create_dir(path)?                  // one dir
fs::create_dir_all(path)?              // recursive
fs::remove_dir(path)?                  // empty dir
fs::remove_dir_all(path)?              // dir tree
fs::read_dir(path)?                    // iterator of entries
fs::metadata(path)?                    // file info
```

## Useful Crates

| Crate | Purpose |
|-------|---------|
| `tempfile` | safe temp files and dirs |
| `walkdir` | recursive directory walking |
| `ignore` | like walkdir, respects .gitignore |
| `notify` | filesystem event watching |
| `memmap2` | memory-mapped files |
| `tokio::fs` | async file I/O |

**Next:** [Command-Line Apps](../44_cli_apps) — `clap`, `std::env::args`, exit codes.
