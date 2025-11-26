use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("iconv: convert character encoding - {}", args.join(" "));
    Ok(())
}
