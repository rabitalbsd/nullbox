use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("base32: encode/decode - {}", args.join(" "));
    Ok(())
}
