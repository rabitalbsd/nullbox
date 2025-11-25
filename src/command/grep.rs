use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: grep <pattern> <file>..."));
    }

    let pattern = &args[0];
    for file in &args[1..] {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            let line = line?;
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
    }
    Ok(())
}
