use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: touch <file>..."));
    }

    for file in args {
        if Path::new(file).exists() {
            OpenOptions::new().write(true).open(file)?;
        } else {
            File::create(file)?;
        }
    }
    Ok(())
}
