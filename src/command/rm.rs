use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: rm <file>..."));
    }

    for file in args {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("rm: cannot remove '{}': {}", file, e);
        }
    }
    Ok(())
}
