use std::fs;
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let path = if args.is_empty() { "." } else { &args[0] };
    let size = get_dir_size(Path::new(path))?;
    println!("{}\t{}", size / 1024, path);
    Ok(())
}

fn get_dir_size(path: &Path) -> io::Result<u64> {
    let mut total = 0;
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if metadata.is_dir() {
                total += get_dir_size(&entry.path())?;
            } else {
                total += metadata.len();
            }
        }
    } else {
        total = fs::metadata(path)?.len();
    }
    
    Ok(total)
}
