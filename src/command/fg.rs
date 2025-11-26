use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("fg: bring job to foreground - {}", args.join(" "));
    Ok(())
}
