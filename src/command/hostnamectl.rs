use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("hostnamectl: control hostname - {}", args.join(" "));
    Ok(())
}
