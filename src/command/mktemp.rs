use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("/tmp/tmp.XXXXXXXXXX");
    Ok(())
}
