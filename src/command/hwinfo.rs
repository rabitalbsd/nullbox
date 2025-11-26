use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("hwinfo: hardware information");
    Ok(())
}
