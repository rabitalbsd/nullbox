use std::fs;
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: cp <source> <dest>"));
    }

    let source = &args[0];
    let dest = &args[1];
    
    let src_path = Path::new(source);
    let dst_path = Path::new(dest);

    if src_path.is_dir() {
        copy_dir_recursive(src_path, dst_path)?;
    } else {
        if dst_path.is_dir() {
            let file_name = src_path.file_name().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "invalid source path")
            })?;
            let target = dst_path.join(file_name);
            fs::copy(src_path, target)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
