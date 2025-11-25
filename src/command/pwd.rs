use std::env;
use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    let cwd = env::current_dir()?;
    println!("{}", cwd.display());
    Ok(())
}
