use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let device = if args.is_empty() { "cdrom" } else { &args[0] };
    println!("eject: {}", device);
    Ok(())
}
