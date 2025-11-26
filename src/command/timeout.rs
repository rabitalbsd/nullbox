use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: timeout duration command"));
    }
    println!("timeout: run with time limit");
    Ok(())
}
