use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("service: run System V init script - {}", args.join(" "));
    Ok(())
}
