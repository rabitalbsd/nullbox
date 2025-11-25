use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Err(e) = nullbox::command::whoami::execute(&args) {
        eprintln!("whoami: {}", e);
        process::exit(1);
    }
}
