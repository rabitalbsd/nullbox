use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut verbose = false;
    let mut interactive = false;
    let mut force = false;
    let mut sources = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "-i" | "--interactive" => interactive = true,
            "-f" | "--force" => force = true,
            s if !s.starts_with('-') => sources.push(s),
            _ => {}
        }
    }
    
    if sources.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: mv [options] source... dest"));
    }
    
    let dest = sources.pop().unwrap();
    let dst_path = Path::new(dest);
    
    for source in sources {
        let src_path = Path::new(source);
        
        if !src_path.exists() {
            eprintln!("mv: cannot stat '{}': No such file or directory", source);
            continue;
        }
        
        let target = if dst_path.is_dir() {
            let file_name = src_path.file_name().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "invalid source path")
            })?;
            dst_path.join(file_name)
        } else {
            dst_path.to_path_buf()
        };
        
        if interactive && target.exists() && !force {
            print!("mv: overwrite '{}'? ", target.display());
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if !response.trim().to_lowercase().starts_with('y') {
                continue;
            }
        }
        
        match fs::rename(src_path, &target) {
            Ok(_) => {
                if verbose {
                    println!("'{}' -> '{}'", source, target.display());
                }
            }
            Err(e) => {
                eprintln!("mv: cannot move '{}' to '{}': {}", source, target.display(), e);
            }
        }
    }
    Ok(())
}
