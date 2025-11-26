use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::sha1::execute(&args) {
        eprintln!("sha1: {}", e);
        process::exit(1);
    }
}
