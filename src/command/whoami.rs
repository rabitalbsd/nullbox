use std::env;
use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    let user = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    println!("{}", user);
    Ok(())
}
