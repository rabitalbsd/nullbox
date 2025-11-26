use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("shutdown: {}", args.join(" "));
    Ok(())
}
