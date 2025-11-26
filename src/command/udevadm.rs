use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("udevadm: udev management - {}", args.join(" "));
    Ok(())
}
