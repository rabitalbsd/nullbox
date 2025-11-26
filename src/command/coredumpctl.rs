use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("coredumpctl: retrieve coredumps - {}", args.join(" "));
    Ok(())
}
