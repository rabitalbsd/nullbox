use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: rx <pattern> <file>..."));
    }

    let pattern = &args[0];
    let re = Regex::new(pattern).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("invalid regex: {}", e))
    })?;

    for file in &args[1..] {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if re.is_match(&line) {
                println!("{}:{}: {}", file, line_num + 1, line);
            }
        }
    }
    Ok(())
}
