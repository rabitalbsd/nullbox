use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing device"));
    }
    println!("badblocks: search for bad blocks");
    Ok(())
}
