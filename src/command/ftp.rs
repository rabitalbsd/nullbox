use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("ftp>");
    } else {
        println!("ftp: connecting to {}", args[0]);
    }
    Ok(())
}
