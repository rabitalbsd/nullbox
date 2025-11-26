use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: chattr [+-=][attrs] files"));
    }
    println!("chattr: change file attributes");
    Ok(())
}
