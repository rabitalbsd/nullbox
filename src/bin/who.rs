use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::who::execute(&args) {
        eprintln!("who: {}", e);
        process::exit(1);
    }
}
