use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn execute(_args: &[String]) -> io::Result<()> {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    println!("{}", duration.as_secs());
    Ok(())
}
