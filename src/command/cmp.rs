use std::io;
use std::fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: cmp file1 file2"));
    }
    let data1 = fs::read(&args[0])?;
    let data2 = fs::read(&args[1])?;
    if data1 == data2 {
        Ok(())
    } else {
        println!("{} {} differ", args[0], args[1]);
        Ok(())
    }
}
