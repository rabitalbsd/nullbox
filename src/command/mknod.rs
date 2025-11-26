use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: mknod name type"));
    }
    println!("mknod: create special file");
    Ok(())
}
