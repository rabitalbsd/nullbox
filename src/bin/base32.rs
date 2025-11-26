use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::base32::execute(&args) {
        eprintln!("base32: {}", e);
        process::exit(1);
    }
}
