use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: mv <source> <dest>"));
    }

    let source = &args[0];
    let dest = &args[1];
    
    fs::rename(source, dest)?;
    Ok(())
}
