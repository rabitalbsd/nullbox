use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut recursive = false;
    let mut verbose = false;
    let mut interactive = false;
    let mut sources = Vec::new();
    
    for arg in args {
        match arg.as_str() {
            "-r" | "-R" | "--recursive" => recursive = true,
            "-v" | "--verbose" => verbose = true,
            "-i" | "--interactive" => interactive = true,
            s if !s.starts_with('-') => sources.push(s),
            _ => {}
        }
    }
    
    if sources.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: cp [options] source... dest"));
    }
    
    let dest = sources.pop().unwrap();
    let dst_path = Path::new(dest);
    
    for source in sources {
        let src_path = Path::new(source);
        
        if !src_path.exists() {
            eprintln!("cp: cannot stat '{}': No such file or directory", source);
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
        
        if interactive && target.exists() {
            print!("cp: overwrite '{}'? ", target.display());
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if !response.trim().to_lowercase().starts_with('y') {
                continue;
            }
        }
        
        if src_path.is_dir() {
            if !recursive {
                eprintln!("cp: -r not specified; omitting directory '{}'", source);
                continue;
            }
            copy_dir_recursive(src_path, &target, verbose)?;
        } else {
            fs::copy(src_path, &target)?;
            if verbose {
                println!("'{}' -> '{}'", source, target.display());
            }
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path, verbose: bool) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
        if verbose {
            println!("created directory '{}'", dst.display());
        }
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path, verbose)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
            if verbose {
                println!("'{}' -> '{}'", src_path.display(), dst_path.display());
            }
        }
    }
    Ok(())
}
