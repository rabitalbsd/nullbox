use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let path = if args.is_empty() { "." } else { &args[0] };
    
    let entries = fs::read_dir(path)?;
    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                println!("{}/", name_str);
            } else {
                println!("{}", name_str);
            }
        }
    }
    Ok(())
}
