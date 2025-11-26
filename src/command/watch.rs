use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("watch: execute periodically - {}", args.join(" "));
    Ok(())
}
