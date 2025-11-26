use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("/dev/sda1: UUID=\"xxxx-xxxx\" TYPE=\"ext4\"");
    Ok(())
}
