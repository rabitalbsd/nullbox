use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("inxi: system information");
    Ok(())
}
