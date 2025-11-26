use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::fmt::execute(&args) {
        eprintln!("fmt: {}", e);
        process::exit(1);
    }
}
