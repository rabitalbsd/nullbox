use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        print!("{}", buffer);
    } else {
        for file in args {
            let f = File::open(file)?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }
    }
    Ok(())
}
