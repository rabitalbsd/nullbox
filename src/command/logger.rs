use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("logger: {}", args.join(" "));
    Ok(())
}
