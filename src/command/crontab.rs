use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("crontab: schedule periodic commands - {}", args.join(" "));
    Ok(())
}
