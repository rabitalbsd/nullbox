use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("exec: replace shell with command - {}", args.join(" "));
    Ok(())
}
