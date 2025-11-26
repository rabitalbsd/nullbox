use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("halt: stopping system");
    Ok(())
}
