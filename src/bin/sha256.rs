use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Err(e) = nullbox::command::sha256::execute(&args) {
        eprintln!("sha256: {}", e);
        process::exit(1);
    }
}
