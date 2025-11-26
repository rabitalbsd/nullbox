use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("xargs: build and execute command lines - {}", args.join(" "));
    Ok(())
}
