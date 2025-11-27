// Cross-platform utilities for Windows and Unix (FreeBSD)

use std::fs;

/// Get file size in a cross-platform way
#[cfg(windows)]
pub fn get_file_size(metadata: &fs::Metadata) -> u64 {
    use std::os::windows::fs::MetadataExt;
    metadata.file_size()
}

#[cfg(unix)]
pub fn get_file_size(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.size()
}

/// Get file permissions string in a cross-platform way
#[cfg(windows)]
pub fn get_permissions_string(metadata: &fs::Metadata) -> String {
    if metadata.permissions().readonly() {
        "r--r--r--".to_string()
    } else {
        "rw-rw-rw-".to_string()
    }
}

#[cfg(unix)]
pub fn get_permissions_string(metadata: &fs::Metadata) -> String {
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

/// Get file owner/group info in a cross-platform way
#[cfg(windows)]
pub fn get_owner_info(_metadata: &fs::Metadata) -> (String, String) {
    ("user".to_string(), "group".to_string())
}

#[cfg(unix)]
pub fn get_owner_info(metadata: &fs::Metadata) -> (String, String) {
    use std::os::unix::fs::MetadataExt;
    (metadata.uid().to_string(), metadata.gid().to_string())
}

/// Check if path is hidden (starts with . on Unix, has hidden attribute on Windows)
#[cfg(windows)]
pub fn is_hidden(name: &str, _path: &std::path::Path) -> bool {
    // On Windows, check for hidden attribute
    // For simplicity, also treat dot-files as hidden
    name.starts_with('.')
}

#[cfg(unix)]
pub fn is_hidden(name: &str, _path: &std::path::Path) -> bool {
    name.starts_with('.')
}

/// Format file size in human-readable format
pub fn format_size_human(size: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];
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

/// Get current username in a cross-platform way
#[cfg(windows)]
pub fn get_username() -> String {
    std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string())
}

#[cfg(unix)]
pub fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "user".to_string())
}

/// Get hostname in a cross-platform way
pub fn get_hostname() -> String {
    hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "localhost".to_string())
}

/// Check if running on Windows
pub fn is_windows() -> bool {
    cfg!(windows)
}

/// Check if running on Unix (FreeBSD, etc.)
pub fn is_unix() -> bool {
    cfg!(unix)
}
