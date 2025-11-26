use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("dd: convert and copy - {}", args.join(" "));
    Ok(())
}
