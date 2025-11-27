use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_bytes = false;
    let mut show_chars = false;
    let mut files = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-l" | "--lines" => show_lines = true,
            "-w" | "--words" => show_words = true,
            "-c" | "--bytes" => show_bytes = true,
            "-m" | "--chars" => show_chars = true,
            s if !s.starts_with('-') => files.push(s),
            _ => {}
        }
    }
    
    // If no flags specified, show all
    if !show_lines && !show_words && !show_bytes && !show_chars {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }
    
    if files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: wc [options] file..."));
    }
    
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for file in &files {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let mut lines = 0;
        let mut words = 0;
        let mut bytes = 0;
        let mut chars = 0;

        for line in reader.lines() {
            let line = line?;
            lines += 1;
            words += line.split_whitespace().count();
            bytes += line.len() + 1;
            chars += line.chars().count() + 1;
        }

        total_lines += lines;
        total_words += words;
        total_bytes += bytes;
        total_chars += chars;

        let mut output = String::new();
        if show_lines {
            output.push_str(&format!("{:8} ", lines));
        }
        if show_words {
            output.push_str(&format!("{:8} ", words));
        }
        if show_bytes {
            output.push_str(&format!("{:8} ", bytes));
        }
        if show_chars {
            output.push_str(&format!("{:8} ", chars));
        }
        println!("{}{}", output, file);
    }
    
    if files.len() > 1 {
        let mut output = String::new();
        if show_lines {
            output.push_str(&format!("{:8} ", total_lines));
        }
        if show_words {
            output.push_str(&format!("{:8} ", total_words));
        }
        if show_bytes {
            output.push_str(&format!("{:8} ", total_bytes));
        }
        if show_chars {
            output.push_str(&format!("{:8} ", total_chars));
        }
        println!("{}total", output);
    }
    
    Ok(())
}
