use std::fs;
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let path = if args.is_empty() { "." } else { &args[0] };
    let pattern = if args.len() > 1 { Some(&args[1]) } else { None };
    
    walk_dir(Path::new(path), pattern)?;
    Ok(())
}

fn walk_dir(dir: &Path, pattern: Option<&String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();
            
            if let Some(pat) = pattern {
                if name.contains(pat.as_str()) {
                    println!("{}", path.display());
                }
            } else {
                println!("{}", path.display());
            }
            
            if path.is_dir() {
                walk_dir(&path, pattern)?;
            }
        }
    }
    Ok(())
}
