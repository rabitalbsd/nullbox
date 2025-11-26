use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing host"));
    }
    println!("Server:  dns.server.com");
    println!("Address: 8.8.8.8");
    Ok(())
}
