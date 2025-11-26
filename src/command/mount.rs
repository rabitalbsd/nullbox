use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("mount: show mounted filesystems");
    } else {
        println!("mount: {}", args.join(" "));
    }
    Ok(())
}
