use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: wc <file>..."));
    }

    for file in args {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let mut lines = 0;
        let mut words = 0;
        let mut bytes = 0;

        for line in reader.lines() {
            let line = line?;
            lines += 1;
            words += line.split_whitespace().count();
            bytes += line.len() + 1;
        }

        println!("{:8} {:8} {:8} {}", lines, words, bytes, file);
    }
    Ok(())
}
