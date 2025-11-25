use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: split <file>"));
    }

    let f = File::open(&args[0])?;
    let reader = BufReader::new(f);
    let lines_per_file = 1000;
    let mut file_num = 0;
    let mut line_count = 0;
    let mut out_file = File::create(format!("x{:02}", file_num))?;

    for line in reader.lines() {
        let line = line?;
        writeln!(out_file, "{}", line)?;
        line_count += 1;

        if line_count >= lines_per_file {
            file_num += 1;
            out_file = File::create(format!("x{:02}", file_num))?;
            line_count = 0;
        }
    }
    Ok(())
}
