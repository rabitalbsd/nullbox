use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: expand <file>"));
    }

    let f = File::open(&args[0])?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let expanded = line.replace('\t', "        ");
        println!("{}", expanded);
    }
    Ok(())
}
