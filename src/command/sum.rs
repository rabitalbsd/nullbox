use std::fs::File;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: sum <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    let sum: u32 = buffer.iter().map(|&b| b as u32).sum();
    println!("{} {}", sum, buffer.len());
    Ok(())
}
