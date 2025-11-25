use std::env;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        env::set_current_dir(&home)?;
    } else {
        env::set_current_dir(&args[0])?;
    }
    println!("{}", env::current_dir()?.display());
    Ok(())
}
