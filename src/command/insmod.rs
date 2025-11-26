use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing module"));
    }
    println!("insmod: insert kernel module");
    Ok(())
}
