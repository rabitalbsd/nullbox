use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut parents = false;
    let mut verbose = false;
    let mut dirs = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-p" | "--parents" => parents = true,
            "-v" | "--verbose" => verbose = true,
            s if !s.starts_with('-') => dirs.push(s),
            _ => {}
        }
    }
    
    if dirs.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: mkdir [options] directory..."));
    }

    for dir in dirs {
        let result = if parents {
            fs::create_dir_all(dir)
        } else {
            fs::create_dir(dir)
        };
        
        match result {
            Ok(_) => {
                if verbose {
                    println!("mkdir: created directory '{}'", dir);
                }
            }
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
            }
        }
    }
    Ok(())
}
