use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("resolvectl: resolve domain names - {}", args.join(" "));
    Ok(())
}
