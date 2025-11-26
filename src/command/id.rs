use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("uid=1000 gid=1000 groups=1000");
    Ok(())
}
