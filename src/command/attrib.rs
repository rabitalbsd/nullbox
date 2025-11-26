use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing file"));
    }
    let metadata = fs::metadata(&args[0])?;
    let readonly = metadata.permissions().readonly();
    println!("{} {}", if readonly { "R" } else { " " }, args[0]);
    Ok(())
}
