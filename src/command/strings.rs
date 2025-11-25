use std::fs::File;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: strings <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    let mut current = String::new();
    for &byte in &buffer {
        if byte >= 32 && byte <= 126 {
            current.push(byte as char);
        } else if current.len() >= 4 {
            println!("{}", current);
            current.clear();
        } else {
            current.clear();
        }
    }
    if current.len() >= 4 {
        println!("{}", current);
    }
    Ok(())
}
