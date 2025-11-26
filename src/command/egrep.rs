use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: egrep pattern [file]"));
    }
    println!("egrep: extended grep");
    Ok(())
}
