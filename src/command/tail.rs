use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

pub fn execute(args: &[String]) -> io::Result<()> {
    let (lines, follow, quiet, verbose, files) = parse_args(args);
    
    if files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: tail [options] file..."));
    }

    for (i, file) in files.iter().enumerate() {
        let show_header = (files.len() > 1 && !quiet) || verbose;
        
        if show_header {
            if i > 0 { println!(); }
            println!("==> {} <==", file);
        }
        
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let mut buffer: VecDeque<String> = VecDeque::with_capacity(lines);
        
        for line in reader.lines() {
            let line = line?;
            if buffer.len() == lines {
                buffer.pop_front();
            }
            buffer.push_back(line);
        }
        
        for line in &buffer {
            println!("{}", line);
        }
        
        if follow {
            let filename = file.clone();
            let mut file_handle = File::open(file)?;
            file_handle.seek(SeekFrom::End(0))?;
            let mut reader = BufReader::new(file_handle);
            
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Ok(_) => {
                        print!("{}", line);
                    }
                    Err(e) => {
                        eprintln!("tail: error reading '{}': {}", filename, e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_args(args: &[String]) -> (usize, bool, bool, bool, Vec<String>) {
    let mut lines = 10;
    let mut follow = false;
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
            "-f" | "--follow" => {
                follow = true;
                i += 1;
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
            s if !s.starts_with('-') => {
                files.push(s.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }
    
    (lines, follow, quiet, verbose, files)
}
