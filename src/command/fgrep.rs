use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: fgrep string [file]"));
    }
    println!("fgrep: fixed string grep");
    Ok(())
}
