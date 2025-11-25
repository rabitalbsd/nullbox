use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: stat <file>"));
    }

    let metadata = fs::metadata(&args[0])?;
    println!("  File: {}", args[0]);
    println!("  Size: {}", metadata.len());
    println!("  Type: {}", if metadata.is_dir() { "directory" } else { "file" });
    Ok(())
}
