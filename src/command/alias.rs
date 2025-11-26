use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("alias: display or create command aliases");
    } else {
        println!("{}", args.join(" "));
    }
    Ok(())
}
