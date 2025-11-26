use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("chkconfig: system services - {}", args.join(" "));
    Ok(())
}
