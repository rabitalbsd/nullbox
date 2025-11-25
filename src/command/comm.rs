use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: comm <file1> <file2>"));
    }

    let f1 = File::open(&args[0])?;
    let f2 = File::open(&args[1])?;
    
    let lines1: Vec<String> = BufReader::new(f1).lines().collect::<Result<_, _>>()?;
    let lines2: Vec<String> = BufReader::new(f2).lines().collect::<Result<_, _>>()?;
    
    for line in &lines1 {
        if !lines2.contains(line) {
            println!("{}", line);
        }
    }
    Ok(())
}
