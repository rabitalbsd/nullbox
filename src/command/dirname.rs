use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: dirname <path>"));
    }

    let path = Path::new(&args[0]);
    if let Some(parent) = path.parent() {
        println!("{}", parent.display());
    } else {
        println!(".");
    }
    Ok(())
}
