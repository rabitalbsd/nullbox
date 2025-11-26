use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("MemTotal:       16777216 kB");
    println!("MemFree:         8388608 kB");
    Ok(())
}
