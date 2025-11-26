use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: install source dest"));
    }
    println!("install: copy files and set attributes");
    Ok(())
}
