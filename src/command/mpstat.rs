use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("mpstat: multiprocessor statistics");
    Ok(())
}
