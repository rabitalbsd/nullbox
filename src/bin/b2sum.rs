use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::b2sum::execute(&args) {
        eprintln!("b2sum: {}", e);
        process::exit(1);
    }
}
