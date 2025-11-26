use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("COMMAND    PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME");
    Ok(())
}
