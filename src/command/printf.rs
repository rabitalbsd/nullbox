use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    print!("{}", args.join(" "));
    Ok(())
}
