use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: zip archive files"));
    }
    println!("zip: creating {}", args[0]);
    Ok(())
}
