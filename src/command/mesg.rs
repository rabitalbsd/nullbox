use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("is y");
    } else {
        println!("mesg: {}", args[0]);
    }
    Ok(())
}
