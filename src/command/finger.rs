use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let user = if args.is_empty() { "user" } else { &args[0] };
    println!("Login: {}                Name: User", user);
    Ok(())
}
