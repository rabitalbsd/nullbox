use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: comp file1 file2"));
    }
    println!("comp: compare files");
    Ok(())
}
