use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: chgrp group file"));
    }
    println!("chgrp: change group ownership");
    Ok(())
}
