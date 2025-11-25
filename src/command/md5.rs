use std::fs::File;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: md5 <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    let digest = md5::compute(&buffer);
    println!("{:x}  {}", digest, args[0]);
    Ok(())
}
