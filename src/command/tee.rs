use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: tee <file>"));
    }

    let mut file = File::create(&args[0])?;
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
