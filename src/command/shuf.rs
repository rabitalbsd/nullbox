use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("shuf: shuffle lines - {}", args.join(" "));
    Ok(())
}
