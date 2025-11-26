use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("sed: stream editor - {}", args.join(" "));
    Ok(())
}
