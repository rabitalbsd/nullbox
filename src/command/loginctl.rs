use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("loginctl: control login manager - {}", args.join(" "));
    Ok(())
}
