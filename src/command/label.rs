use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("label: volume label - {}", args.join(" "));
    Ok(())
}
