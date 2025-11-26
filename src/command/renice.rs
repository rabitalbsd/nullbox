use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: renice priority pid"));
    }
    println!("renice: alter priority of running process");
    Ok(())
}
