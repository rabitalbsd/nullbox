use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("wait: {}", args.join(" "));
    Ok(())
}
