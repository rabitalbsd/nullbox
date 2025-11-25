use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 3 || args[0] != "-f" {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: cut -f <field> <file>"));
    }

    let field: usize = args[1].parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid field number"))?;
    
    let f = File::open(&args[2])?;
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if field > 0 && field <= parts.len() {
            println!("{}", parts[field - 1]);
        }
    }
    Ok(())
}
