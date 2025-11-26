use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing file"));
    }
    let content = fs::read_to_string(&args[0])?;
    let converted = content.replace("\r\n", "\n");
    fs::write(&args[0], converted)?;
    Ok(())
}
