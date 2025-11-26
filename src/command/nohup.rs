use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("nohup: run immune to hangups - {}", args.join(" "));
    Ok(())
}
