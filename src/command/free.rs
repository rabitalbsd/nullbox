use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("              total        used        free");
    println!("Mem:       16777216     8388608     8388608");
    Ok(())
}
