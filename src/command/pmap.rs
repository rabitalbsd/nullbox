use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing pid"));
    }
    println!("pmap: report memory map of process");
    Ok(())
}
