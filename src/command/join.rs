use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: join <file1> <file2>"));
    }

    let f1 = File::open(&args[0])?;
    let f2 = File::open(&args[1])?;
    
    let lines1: Vec<String> = BufReader::new(f1).lines().collect::<Result<_, _>>()?;
    let lines2: Vec<String> = BufReader::new(f2).lines().collect::<Result<_, _>>()?;
    
    for (l1, l2) in lines1.iter().zip(lines2.iter()) {
        println!("{} {}", l1, l2);
    }
    Ok(())
}
