use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: link file1 file2"));
    }
    println!("link: create hard link");
    Ok(())
}
