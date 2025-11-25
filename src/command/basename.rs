use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: basename <path>"));
    }

    let path = Path::new(&args[0]);
    if let Some(name) = path.file_name() {
        println!("{}", name.to_string_lossy());
    }
    Ok(())
}
