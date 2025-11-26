use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("bc: basic calculator");
    } else {
        println!("bc: {}", args.join(" "));
    }
    Ok(())
}
