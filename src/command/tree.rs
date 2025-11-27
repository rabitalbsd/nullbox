use std::fs;
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    let path = if args.is_empty() { "." } else { &args[0] };
    println!("{}", path);
    print_tree(Path::new(path), "", true)?;
    Ok(())
}

fn print_tree(dir: &Path, prefix: &str, _is_last: bool) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let entries: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
    let count = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let is_last_entry = i == count - 1;
        let connector = if is_last_entry { "└── " } else { "├── " };
        let name = entry.file_name();
        
        println!("{}{}{}", prefix, connector, name.to_string_lossy());
        
        if entry.path().is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last_entry { "    " } else { "│   " });
            print_tree(&entry.path(), &new_prefix, is_last_entry)?;
        }
    }
    
    Ok(())
}
