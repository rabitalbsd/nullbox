use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("awk: pattern scanning - {}", args.join(" "));
    Ok(())
}
