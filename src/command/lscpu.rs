use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Architecture:        x86_64");
    println!("CPU(s):              4");
    Ok(())
}
