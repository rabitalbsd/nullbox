use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let device = if args.is_empty() { "filesystem" } else { &args[0] };
    println!("fsck: checking {}", device);
    Ok(())
}
