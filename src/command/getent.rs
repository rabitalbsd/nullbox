use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("getent: get entries from databases - {}", args.join(" "));
    Ok(())
}
