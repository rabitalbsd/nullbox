use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    if let Ok(name) = hostname::get() {
        println!("{}", name.to_string_lossy());
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to get hostname"));
    }
    Ok(())
}
