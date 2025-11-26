use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let path = if args.is_empty() { "." } else { &args[0] };
    println!("lsattr: list file attributes for {}", path);
    Ok(())
}
