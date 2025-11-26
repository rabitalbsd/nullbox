use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("parted: partition manipulation - {}", args.join(" "));
    Ok(())
}
