use std::fs::File;
use std::io::{self, Read, BufReader, BufRead};

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut number_lines = false;
    let mut number_nonblank = false;
    let mut show_ends = false;
    let mut files = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-n" | "--number" => number_lines = true,
            "-b" | "--number-nonblank" => number_nonblank = true,
            "-E" | "--show-ends" => show_ends = true,
            _ if !arg.starts_with('-') => files.push(arg.as_str()),
            _ => {}
        }
    }
    
    if files.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        print!("{}", buffer);
        return Ok(());
    }
    
    let mut line_num = 1;
    for file in files {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            let line = line?;
            let is_blank = line.trim().is_empty();
            
            if number_lines || (number_nonblank && !is_blank) {
                print!("{:6}  ", line_num);
                if !is_blank || number_lines {
                    line_num += 1;
                }
            }
            
            print!("{}", line);
            if show_ends {
                print!("$");
            }
            println!();
        }
    }
    Ok(())
}
