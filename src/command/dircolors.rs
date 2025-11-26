use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("LS_COLORS='di=01;34:ln=01;36'");
    Ok(())
}
