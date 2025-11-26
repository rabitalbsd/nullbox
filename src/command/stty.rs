use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("speed 38400 baud; line = 0;");
    Ok(())
}
