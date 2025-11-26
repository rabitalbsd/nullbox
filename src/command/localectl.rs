use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("localectl: control locale - {}", args.join(" "));
    Ok(())
}
