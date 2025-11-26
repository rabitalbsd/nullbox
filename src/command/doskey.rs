use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("doskey: edit command line macros - {}", args.join(" "));
    Ok(())
}
