use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("pidstat: process statistics");
    Ok(())
}
