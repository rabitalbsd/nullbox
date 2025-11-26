use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing file"));
    }
    println!("nm: list symbols from {}", args[0]);
    Ok(())
}
