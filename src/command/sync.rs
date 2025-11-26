use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("sync: flushing buffers");
    Ok(())
}
