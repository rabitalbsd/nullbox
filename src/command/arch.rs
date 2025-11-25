use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("{}", std::env::consts::ARCH);
    Ok(())
}
