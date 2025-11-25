use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if let Err(e) = nullbox::command::md5::execute(&args) {
        eprintln!("md5: {}", e);
        process::exit(1);
    }
}
