use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("time: measure command execution - {}", args.join(" "));
    Ok(())
}
