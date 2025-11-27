use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut recursive = false;
    let mut force = false;
    let mut verbose = false;
    let mut interactive = false;
    let mut files = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-r" | "-R" | "--recursive" => recursive = true,
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-i" | "--interactive" => interactive = true,
            "-rf" | "-fr" => { recursive = true; force = true; }
            s if !s.starts_with('-') => files.push(s),
            _ => {}
        }
    }
    
    if files.is_empty() {
        if !force {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: rm [options] file..."));
        }
        return Ok(());
    }
    
    for file in files {
        let path = Path::new(file);
        
        if !path.exists() {
            if !force {
                eprintln!("rm: cannot remove '{}': No such file or directory", file);
            }
            continue;
        }
        
        if interactive {
            let prompt = if path.is_dir() {
                format!("rm: remove directory '{}'? ", file)
            } else {
                format!("rm: remove file '{}'? ", file)
            };
            print!("{}", prompt);
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if !response.trim().to_lowercase().starts_with('y') {
                continue;
            }
        }
        
        let result = if path.is_dir() {
            if !recursive {
                eprintln!("rm: cannot remove '{}': Is a directory", file);
                continue;
            }
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };
        
        match result {
            Ok(_) => {
                if verbose {
                    println!("removed '{}'", file);
                }
            }
            Err(e) => {
                if !force {
                    eprintln!("rm: cannot remove '{}': {}", file, e);
                }
            }
        }
    }
    Ok(())
}
