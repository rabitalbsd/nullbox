use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("slabtop: kernel slab cache information");
    Ok(())
}
