use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: nl <file>"));
    }

    let f = File::open(&args[0])?;
    let reader = BufReader::new(f);
    
    for (i, line) in reader.lines().enumerate() {
        println!("{:6}\t{}", i + 1, line?);
    }
    Ok(())
}
