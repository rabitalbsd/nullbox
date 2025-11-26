use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("jobs: list active jobs");
    Ok(())
}
