use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing file"));
    }
    fs::remove_file(&args[0])?;
    Ok(())
}
