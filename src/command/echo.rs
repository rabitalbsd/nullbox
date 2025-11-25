use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("{}", args.join(" "));
    Ok(())
}
