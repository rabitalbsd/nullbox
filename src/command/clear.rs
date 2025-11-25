use std::io::{self, Write};

pub fn execute(_args: &[String]) -> io::Result<()> {
    if cfg!(target_os = "windows") {
        print!("\x1B[2J\x1B[1;1H");
    } else {
        print!("\x1B[2J\x1B[H");
    }
    io::stdout().flush()?;
    Ok(())
}
