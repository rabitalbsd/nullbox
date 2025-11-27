use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let mut long_format = false;
    let mut show_all = false;
    let mut human_readable = false;
    let mut path = ".";

    for arg in args {
        match arg.as_str() {
            "-l" => long_format = true,
            "-a" | "--all" => show_all = true,
            "-h" | "--human-readable" => human_readable = true,
            "-la" | "-al" => {
                long_format = true;
                show_all = true;
            }
            "-lh" | "-hl" => {
                long_format = true;
                human_readable = true;
            }
            "-lah" | "-alh" | "-hal" | "-hla" | "-ahl" | "-lha" => {
                long_format = true;
                show_all = true;
                human_readable = true;
            }
            s if !s.starts_with('-') => path = s,
            _ => {}
        }
    }

    let entries = fs::read_dir(path)?;
    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let entry_path = entry.path();

        if !show_all && is_hidden(&name_str) {
            continue;
        }

        if let Ok(metadata) = entry.metadata() {
            if long_format {
                let size = get_file_size(&metadata);
                let size_str = if human_readable {
                    format_size_human(size)
                } else {
                    size.to_string()
                };

                let file_type = if metadata.is_dir() { "d" } else { "-" };
                let perms = get_permissions_string(&metadata);

                println!("{}{} {:>10} {}", file_type, perms, size_str, name_str);
            } else {
                if metadata.is_dir() {
                    println!("{}/", name_str);
                } else {
                    println!("{}", name_str);
                }
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn get_file_size(metadata: &fs::Metadata) -> u64 {
    use std::os::windows::fs::MetadataExt;
    metadata.file_size()
}

#[cfg(unix)]
fn get_file_size(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.size()
}

#[cfg(windows)]
fn get_permissions_string(metadata: &fs::Metadata) -> String {
    if metadata.permissions().readonly() {
        "r--r--r--".to_string()
    } else {
        "rw-rw-rw-".to_string()
    }
}

#[cfg(unix)]
fn get_permissions_string(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;
    let mode = metadata.permissions().mode();
    
    format!(
        "{}{}{}",
        if mode & 0o400 != 0 { "r" } else { "-" },
        if mode & 0o200 != 0 { "w" } else { "-" },
        if mode & 0o100 != 0 { "x" } else { "-" }
    )
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn format_size_human(size: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut size = size as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{}{}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}
