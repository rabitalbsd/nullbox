use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("sar: system activity reporter");
    Ok(())
}
