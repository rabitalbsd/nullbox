use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Volume in drive C has no label");
    Ok(())
}
