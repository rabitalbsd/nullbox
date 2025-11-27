use std::env;
use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("{}", get_username());
    Ok(())
}

#[cfg(windows)]
fn get_username() -> String {
    env::var("USERNAME").unwrap_or_else(|_| "user".to_string())
}

#[cfg(unix)]
fn get_username() -> String {
    env::var("USER").unwrap_or_else(|_| "user".to_string())
}
