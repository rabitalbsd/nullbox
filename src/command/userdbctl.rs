use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("userdbctl: inspect users/groups - {}", args.join(" "));
    Ok(())
}
