use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("column: columnate lists - {}", args.join(" "));
    Ok(())
}
