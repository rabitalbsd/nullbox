use std::fs::File;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: od <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    for (i, chunk) in buffer.chunks(16).enumerate() {
        print!("{:07o} ", i * 16);
        for byte in chunk {
            print!("{:03o} ", byte);
        }
        println!();
    }
    Ok(())
}
