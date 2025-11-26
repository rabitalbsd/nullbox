use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("lshw: list hardware");
    Ok(())
}
