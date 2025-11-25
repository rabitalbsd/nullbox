use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: fold <file>"));
    }

    let f = File::open(&args[0])?;
    let reader = BufReader::new(f);
    let width = 80;

    for line in reader.lines() {
        let line = line?;
        for chunk in line.as_bytes().chunks(width) {
            println!("{}", String::from_utf8_lossy(chunk));
        }
    }
    Ok(())
}
