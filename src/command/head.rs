use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    let (lines, bytes, quiet, verbose, files) = parse_args(args);
    
    if files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: head [options] file..."));
    }

    for (i, file) in files.iter().enumerate() {
        let show_header = (files.len() > 1 && !quiet) || verbose;
        
        if show_header {
            if i > 0 { println!(); }
            println!("==> {} <==", file);
        }
        
        if let Some(byte_count) = bytes {
            use std::io::Read;
            let mut f = File::open(file)?;
            let mut buffer = vec![0u8; byte_count];
            let n = f.read(&mut buffer)?;
            print!("{}", String::from_utf8_lossy(&buffer[..n]));
        } else {
            let f = File::open(file)?;
            let reader = BufReader::new(f);
            for (idx, line) in reader.lines().enumerate() {
                if idx >= lines { break; }
                println!("{}", line?);
            }
        }
    }
    Ok(())
}

fn parse_args(args: &[String]) -> (usize, Option<usize>, bool, bool, Vec<String>) {
    let mut lines = 10;
    let mut bytes = None;
    let mut quiet = false;
    let mut verbose = false;
    let mut files = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--lines" if i + 1 < args.len() => {
                lines = args[i + 1].parse().unwrap_or(10);
                i += 2;
            }
            "-c" | "--bytes" if i + 1 < args.len() => {
                bytes = Some(args[i + 1].parse().unwrap_or(512));
                i += 2;
            }
            "-q" | "--quiet" | "--silent" => {
                quiet = true;
                i += 1;
            }
            "-v" | "--verbose" => {
                verbose = true;
                i += 1;
            }
            s if s.starts_with("-n") => {
                lines = s[2..].parse().unwrap_or(10);
                i += 1;
            }
            s if s.starts_with("-c") => {
                bytes = Some(s[2..].parse().unwrap_or(512));
                i += 1;
            }
            s if !s.starts_with('-') => {
                files.push(s.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }
    
    (lines, bytes, quiet, verbose, files)
}
