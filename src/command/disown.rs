use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("disown: remove job from table - {}", args.join(" "));
    Ok(())
}
