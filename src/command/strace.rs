use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("strace: trace system calls - {}", args.join(" "));
    Ok(())
}
