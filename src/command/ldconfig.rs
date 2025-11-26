use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("ldconfig: configure dynamic linker");
    Ok(())
}
