use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("reboot: restarting system");
    Ok(())
}
