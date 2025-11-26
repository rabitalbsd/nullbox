use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing file"));
    }
    let metadata = fs::metadata(&args[0])?;
    let ftype = if metadata.is_dir() { "directory" } else { "file" };
    println!("{}: {}", args[0], ftype);
    Ok(())
}
