use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Filesystem     1K-blocks      Used Available Use% Mounted on");
    println!("(disk info not available in portable Rust)");
    Ok(())
}
