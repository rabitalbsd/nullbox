use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: diff <file1> <file2>"));
    }

    let f1 = File::open(&args[0])?;
    let f2 = File::open(&args[1])?;
    
    let lines1: Vec<String> = BufReader::new(f1).lines().collect::<Result<_, _>>()?;
    let lines2: Vec<String> = BufReader::new(f2).lines().collect::<Result<_, _>>()?;
    
    let max_len = lines1.len().max(lines2.len());
    
    for i in 0..max_len {
        let l1 = lines1.get(i);
        let l2 = lines2.get(i);
        
        if l1 != l2 {
            if let Some(line) = l1 {
                println!("< {}", line);
            }
            if let Some(line) = l2 {
                println!("> {}", line);
            }
        }
    }
    Ok(())
}
