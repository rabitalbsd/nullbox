use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("  PID TTY          TIME CMD");
    println!(" 1234 pts/0    00:00:00 bash");
    Ok(())
}
