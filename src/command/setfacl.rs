use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: setfacl -m acl file"));
    }
    println!("setfacl: set file access control lists");
    Ok(())
}
