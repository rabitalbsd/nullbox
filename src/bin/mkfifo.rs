use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::mkfifo::execute(&args) {
        eprintln!("mkfifo: {}", e);
        process::exit(1);
    }
}
