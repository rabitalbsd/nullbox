use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing host"));
    }
    println!("; <<>> DiG 9.18.0 <<>> {}", args[0]);
    Ok(())
}
