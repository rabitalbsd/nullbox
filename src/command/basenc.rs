use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("basenc: encode/decode - {}", args.join(" "));
    Ok(())
}
