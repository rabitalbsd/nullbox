use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("pr: convert text files for printing - {}", args.join(" "));
    Ok(())
}
