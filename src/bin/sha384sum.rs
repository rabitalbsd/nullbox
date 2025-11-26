use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::sha384sum::execute(&args) {
        eprintln!("sha384sum: {}", e);
        process::exit(1);
    }
}
