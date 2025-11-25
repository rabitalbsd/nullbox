use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: realpath <path>"));
    }

    let path = Path::new(&args[0]);
    let canonical = path.canonicalize()?;
    println!("{}", canonical.display());
    Ok(())
}
