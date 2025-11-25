use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("{}", num_cpus::get());
    Ok(())
}
