use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("homectl: manage home directories - {}", args.join(" "));
    Ok(())
}
