use std::fs;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: stat <file>"));
    }

    let path = &args[0];
    let metadata = fs::metadata(path)?;
    let size = get_file_size(&metadata);
    let (owner, group) = get_owner_info(&metadata);
    let perms = get_permissions_string(&metadata);
    
    println!("  File: {}", path);
    println!("  Size: {} bytes", size);
    println!("  Type: {}", if metadata.is_dir() { "directory" } else if metadata.is_file() { "regular file" } else { "other" });
    println!("  Permissions: {}", perms);
    println!("  Owner: {}  Group: {}", owner, group);
    
    if let Ok(modified) = metadata.modified() {
        println!("  Modified: {:?}", modified);
    }
    if let Ok(accessed) = metadata.accessed() {
        println!("  Accessed: {:?}", accessed);
    }
    if let Ok(created) = metadata.created() {
        println!("  Created: {:?}", created);
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
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { "r" } else { "-" },
        if mode & 0o200 != 0 { "w" } else { "-" },
        if mode & 0o100 != 0 { "x" } else { "-" },
        if mode & 0o040 != 0 { "r" } else { "-" },
        if mode & 0o020 != 0 { "w" } else { "-" },
        if mode & 0o010 != 0 { "x" } else { "-" },
        if mode & 0o004 != 0 { "r" } else { "-" },
        if mode & 0o002 != 0 { "w" } else { "-" },
        if mode & 0o001 != 0 { "x" } else { "-" }
    )
}

#[cfg(windows)]
fn get_owner_info(_metadata: &fs::Metadata) -> (String, String) {
    ("user".to_string(), "group".to_string())
}

#[cfg(unix)]
fn get_owner_info(metadata: &fs::Metadata) -> (String, String) {
    use std::os::unix::fs::MetadataExt;
    (metadata.uid().to_string(), metadata.gid().to_string())
}
}
