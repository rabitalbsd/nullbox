use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("tcpdump: packet analyzer - {}", args.join(" "));
    Ok(())
}
