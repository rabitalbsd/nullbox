use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "false"))
}
