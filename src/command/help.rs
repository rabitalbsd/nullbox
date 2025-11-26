use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("help: display command help");
    } else {
        println!("help: {}", args[0]);
    }
    Ok(())
}
