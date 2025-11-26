use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("bg: resume job in background - {}", args.join(" "));
    Ok(())
}
