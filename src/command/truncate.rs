use std::fs::OpenOptions;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 3 || args[0] != "-s" {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: truncate -s <size> <file>"));
    }

    let size = parse_size(&args[1])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    
    let file = OpenOptions::new().write(true).create(true).open(&args[2])?;
    file.set_len(size)?;
    Ok(())
}

fn parse_size(size_str: &str) -> Result<u64, String> {
    let size_str = size_str.trim();
    
    if size_str.is_empty() {
        return Err("empty size".to_string());
    }
    
    // Check for suffix
    let (num_part, multiplier) = if let Some(last_char) = size_str.chars().last() {
        match last_char.to_ascii_uppercase() {
            'K' => (&size_str[..size_str.len()-1], 1024u64),
            'M' => (&size_str[..size_str.len()-1], 1024u64 * 1024),
            'G' => (&size_str[..size_str.len()-1], 1024u64 * 1024 * 1024),
            'T' => (&size_str[..size_str.len()-1], 1024u64 * 1024 * 1024 * 1024),
            'P' => (&size_str[..size_str.len()-1], 1024u64 * 1024 * 1024 * 1024 * 1024),
            _ if last_char.is_ascii_digit() => (size_str, 1u64),
            _ => return Err(format!("invalid size suffix: {}", last_char)),
        }
    } else {
        (size_str, 1u64)
    };
    
    // Parse the numeric part
    let num: u64 = num_part.trim().parse()
        .map_err(|_| format!("invalid number: {}", num_part))?;
    
    num.checked_mul(multiplier)
        .ok_or_else(|| "size overflow".to_string())
}
