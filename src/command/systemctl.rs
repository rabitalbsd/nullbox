use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("systemctl: control systemd - {}", args.join(" "));
    Ok(())
}
