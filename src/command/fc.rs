use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: fc file1 file2"));
    }
    println!("fc: file compare");
    Ok(())
}
