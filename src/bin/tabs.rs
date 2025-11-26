use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::tabs::execute(&args) {
        eprintln!("tabs: {}", e);
        process::exit(1);
    }
}
