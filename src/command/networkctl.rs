use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("networkctl: query network status - {}", args.join(" "));
    Ok(())
}
