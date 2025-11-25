use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: readlink <link>"));
    }

    let target = fs::read_link(&args[0])?;
    println!("{}", target.display());
    Ok(())
}
