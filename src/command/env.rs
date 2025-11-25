use std::env;
use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (key, value) in vars {
        println!("{}={}", key, value);
    }
    Ok(())
}
