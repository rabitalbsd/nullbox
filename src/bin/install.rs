use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::install::execute(&args) {
        eprintln!("install: {}", e);
        process::exit(1);
    }
}
