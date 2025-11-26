use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    print!("\x1b[0m\x1bc");
    Ok(())
}
