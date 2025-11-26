use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("bootctl: control boot loader - {}", args.join(" "));
    Ok(())
}
