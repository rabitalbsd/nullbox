use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::ptx::execute(&args) {
        eprintln!("ptx: {}", e);
        process::exit(1);
    }
}
