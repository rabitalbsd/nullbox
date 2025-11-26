use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("LANG=en_US.UTF-8");
    Ok(())
}
