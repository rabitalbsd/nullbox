use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("at: missing time specification");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing time"));
    }
    println!("at: schedule command at {}", args[0]);
    Ok(())
}
