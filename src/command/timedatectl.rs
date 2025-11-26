use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("timedatectl: control time/date - {}", args.join(" "));
    Ok(())
}
