use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("tar: archive utility - {}", args.join(" "));
    Ok(())
}
