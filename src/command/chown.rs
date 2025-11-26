use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: chown owner file"));
    }
    println!("chown: change file owner");
    Ok(())
}
