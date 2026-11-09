// ============================================================================
//  CHAPTER 45 — RUST FILE I/O & std::fs
//  Reading/writing files, walking directories, working with paths.
// ============================================================================

use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("=== File I/O & std::fs ===\n");

    // ======================================================================
    //  1. Reading a whole file into a String
    // ======================================================================
    //  Easiest API — but loads the entire file into memory.
    let path = "/tmp/rust_io_demo.txt";
    fs::write(path, "first line\nsecond line\nthird line\n")?;

    let contents = fs::read_to_string(path)?;
    println!("1. read_to_string:\n{}", contents);

    // ======================================================================
    //  2. Reading bytes (binary files)
    // ======================================================================
    let bytes: Vec<u8> = fs::read(path)?;
    println!("2. read {} bytes", bytes.len());

    // ======================================================================
    //  3. Reading line-by-line (memory-efficient for large files)
    // ======================================================================
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    println!("3. line-by-line:");
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("   [{}] {}", i, line);
    }

    // ======================================================================
    //  4. Writing files
    // ======================================================================
    //  4a. Write a String/bytes (overwrites):
    fs::write("/tmp/rust_io_demo2.txt", "overwritten content\n")?;

    //  4b. Open a File and Write to it (more control):
    let mut file = fs::File::create("/tmp/rust_io_demo3.txt")?;
    file.write_all(b"hello\n")?;
    file.write_all(b"world\n")?;

    // ======================================================================
    //  5. Appending to a file with OpenOptions
    // ======================================================================
    use std::fs::OpenOptions;
    let mut append_file = OpenOptions::new()
        .append(true)        // open for appending
        .open("/tmp/rust_io_demo3.txt")?;
    append_file.write_all(b"appended line\n")?;

    //  Read it back to verify:
    let final_content = fs::read_to_string("/tmp/rust_io_demo3.txt")?;
    println!("5. after append:\n{}", final_content);

    // ======================================================================
    //  6. Copying files
    // ======================================================================
    fs::copy("/tmp/rust_io_demo.txt", "/tmp/rust_io_demo_copy.txt")?;
    println!("6. copied file. copy exists? {}", Path::new("/tmp/rust_io_demo_copy.txt").exists());

    // ======================================================================
    //  7. Renaming / moving files
    // ======================================================================
    fs::rename("/tmp/rust_io_demo_copy.txt", "/tmp/rust_io_demo_renamed.txt")?;
    println!("7. renamed file. new name exists? {}",
             Path::new("/tmp/rust_io_demo_renamed.txt").exists());

    // ======================================================================
    //  8. Deleting files
    // ======================================================================
    fs::remove_file("/tmp/rust_io_demo_renamed.txt")?;
    println!("8. deleted file.");

    // ======================================================================
    //  9. Creating and removing directories
    // ======================================================================
    fs::create_dir("/tmp/rust_io_demo_dir")?;
    fs::create_dir_all("/tmp/rust_io_demo_dir/sub/deep")?;   // recursive
    println!("9. created dir tree.");

    // ======================================================================
    //  10. Listing directory contents
    // ======================================================================
    println!("10. entries in /tmp/rust_io_demo_dir:");
    for entry in fs::read_dir("/tmp/rust_io_demo_dir")? {
        let entry = entry?;
        let path = entry.path();
        println!("    {} (is_dir={})", path.display(), path.is_dir());
    }

    // ======================================================================
    //  11. Walking a directory tree (with the walkdir crate or manual recursion)
    // ======================================================================
    fn walk(dir: &Path, depth: usize) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let indent = "  ".repeat(depth);
                println!("   {}{}", indent, path.display());
                if path.is_dir() {
                    walk(&path, depth + 1)?;
                }
            }
        }
        Ok(())
    }
    println!("11. walking /tmp/rust_io_demo_dir:");
    walk(Path::new("/tmp/rust_io_demo_dir"), 0)?;

    // ======================================================================
    //  12. File metadata
    // ======================================================================
    let metadata = fs::metadata(path)?;
    println!("12. metadata for {}:", path);
    println!("     size: {} bytes", metadata.len());
    println!("     is_file: {}", metadata.is_file());
    println!("     is_dir: {}", metadata.is_dir());
    println!("     read-only: {}", metadata.permissions().readonly());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        println!("     mode: {:o}", metadata.permissions().mode());
    }

    // ======================================================================
    //  13. Reading file in chunks (for very large files)
    // ======================================================================
    let mut file = fs::File::open(path)?;
    let mut buffer = [0u8; 16];
    let mut total = 0;
    loop {
        let n = file.read(&mut buffer)?;   // returns bytes read
        if n == 0 { break; }               // EOF
        total += n;
    }
    println!("13. read {} bytes via chunks", total);

    // ======================================================================
    //  14. Using `?` for error propagation
    // ======================================================================
    //  main returns io::Result<()>, so we can use `?` throughout. If any
    //  operation fails, the error propagates up and the program exits with
    //  a non-zero status code.
    //
    //  For real projects, use anyhow::Result or a custom error type (see
    //  chapter 31).

    // ======================================================================
    //  15. Cleanup
    // ======================================================================
    let _ = fs::remove_dir_all("/tmp/rust_io_demo_dir");
    let _ = fs::remove_file("/tmp/rust_io_demo.txt");
    let _ = fs::remove_file("/tmp/rust_io_demo2.txt");
    let _ = fs::remove_file("/tmp/rust_io_demo3.txt");

    println!("\nAll file I/O operations succeeded.");
    Ok(())

    // ========================================================================
    //  CHEAT SHEET
    // ========================================================================
    //  Read whole file as String:       fs::read_to_string(path)?
    //  Read whole file as bytes:        fs::read(path)?
    //  Read line by line:               BufReader::new(File::open(path)?).lines()
    //  Read in chunks:                  File::open(path)?.read(&mut buf)?
    //  Write whole file (overwrite):    fs::write(path, content)?
    //  Create empty file:               File::create(path)?
    //  Append to file:                  OpenOptions::new().append(true).open(path)?
    //  Copy file:                       fs::copy(src, dst)?
    //  Rename / move:                   fs::rename(old, new)?
    //  Delete file:                     fs::remove_file(path)?
    //  Make directory:                  fs::create_dir(path)?
    //  Make directory tree (recursive): fs::create_dir_all(path)?
    //  Remove empty dir:                fs::remove_dir(path)?
    //  Remove dir tree:                 fs::remove_dir_all(path)?
    //  List dir entries:                fs::read_dir(path)?
    //  File metadata:                   fs::metadata(path)?
    //  Path operations:                 Path::new(s).exists(), .is_file(), .is_dir()
    // ========================================================================
}
