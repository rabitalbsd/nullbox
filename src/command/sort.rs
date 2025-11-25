use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: sort <file>"));
    }

    let f = File::open(&args[0])?;
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    lines.sort();
    
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}
