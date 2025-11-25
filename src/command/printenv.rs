use std::env;
use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
    } else {
        for key in args {
            if let Ok(value) = env::var(key) {
                println!("{}", value);
            }
        }
    }
    Ok(())
}
