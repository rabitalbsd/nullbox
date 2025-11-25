use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: mkdir <dir>..."));
    }

    for dir in args {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}
