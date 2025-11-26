use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("ionice: set I/O scheduling priority - {}", args.join(" "));
    Ok(())
}
