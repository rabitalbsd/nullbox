use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("processor\t: 0");
    println!("model name\t: CPU @ 2.40GHz");
    Ok(())
}
