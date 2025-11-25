use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    let (lines, files) = parse_args(args);
    
    if files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: head [-n lines] <file>..."));
    }

    for (i, file) in files.iter().enumerate() {
        if files.len() > 1 {
            if i > 0 { println!(); }
            println!("==> {} <==", file);
        }
        
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        
        for (idx, line) in reader.lines().enumerate() {
            if idx >= lines { break; }
            println!("{}", line?);
        }
    }
    Ok(())
}

fn parse_args(args: &[String]) -> (usize, Vec<String>) {
    let mut lines = 10;
    let mut files = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        if args[i] == "-n" && i + 1 < args.len() {
            lines = args[i + 1].parse().unwrap_or(10);
            i += 2;
        } else {
            files.push(args[i].clone());
            i += 1;
        }
    }
    
    (lines, files)
}
