use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("oomctl: analyze OOM kills - {}", args.join(" "));
    Ok(())
}
