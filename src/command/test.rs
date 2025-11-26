use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}
