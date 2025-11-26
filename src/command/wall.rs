use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("Broadcast message: {}", args.join(" "));
    Ok(())
}
