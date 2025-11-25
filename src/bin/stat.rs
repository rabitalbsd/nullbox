use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Err(e) = nullbox::command::stat::execute(&args) {
        eprintln!("stat: {}", e);
        process::exit(1);
    }
}
