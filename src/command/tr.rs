use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: tr <from> <to> <file>"));
    }

    let from = &args[0];
    let to = &args[1];
    let file = &args[2];
    
    if from.len() != to.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "from and to must be same length"));
    }

    let f = File::open(file)?;
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        let mut line = line?;
        for (f_char, t_char) in from.chars().zip(to.chars()) {
            line = line.replace(f_char, &t_char.to_string());
        }
        println!("{}", line);
    }
    Ok(())
}
