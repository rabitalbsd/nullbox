use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: rmdir <dir>..."));
    }

    for dir in args {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}
