use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("make: build automation - {}", args.join(" "));
    Ok(())
}
