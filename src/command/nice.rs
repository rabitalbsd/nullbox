use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("nice: run with modified priority - {}", args.join(" "));
    Ok(())
}
