use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("ar: create/modify archives - {}", args.join(" "));
    Ok(())
}
