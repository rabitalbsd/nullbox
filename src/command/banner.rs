use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let text = if args.is_empty() { "BANNER" } else { &args.join(" ") };
    println!("\n  {}  \n", text.to_uppercase());
    Ok(())
}
