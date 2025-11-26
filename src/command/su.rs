use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let user = if args.is_empty() { "root" } else { &args[0] };
    println!("su: switch user to {}", user);
    Ok(())
}
