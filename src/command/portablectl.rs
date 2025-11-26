use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("portablectl: attach portable services - {}", args.join(" "));
    Ok(())
}
