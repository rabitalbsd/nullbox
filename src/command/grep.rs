use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut ignore_case = false;
    let mut invert_match = false;
    let mut line_numbers = false;
    let mut count_only = false;
    let mut pattern = String::new();
    let mut files = Vec::new();
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--ignore-case" => ignore_case = true,
            "-v" | "--invert-match" => invert_match = true,
            "-n" | "--line-number" => line_numbers = true,
            "-c" | "--count" => count_only = true,
            s if !s.starts_with('-') => {
                if pattern.is_empty() {
                    pattern = s.to_string();
                } else {
                    files.push(s);
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if pattern.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: grep [options] pattern [file...]"));
    }
    
    let pattern_lower = pattern.to_lowercase();
    
    for file in files {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let mut match_count = 0;
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let matches = if ignore_case {
                line.to_lowercase().contains(&pattern_lower)
            } else {
                line.contains(&pattern)
            };
            
            let should_print = if invert_match { !matches } else { matches };
            
            if should_print {
                match_count += 1;
                if !count_only {
                    if line_numbers {
                        println!("{}:{}", line_num + 1, line);
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
        
        if count_only {
            println!("{}", match_count);
        }
    }
    Ok(())
}
