use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Total DISK READ :       0.00 B/s | Total DISK WRITE :       0.00 B/s");
    Ok(())
}
