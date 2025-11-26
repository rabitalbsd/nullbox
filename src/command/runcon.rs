use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("runcon: run with security context - {}", args.join(" "));
    Ok(())
}
