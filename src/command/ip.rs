use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("ip: show/manipulate routing - {}", args.join(" "));
    Ok(())
}
