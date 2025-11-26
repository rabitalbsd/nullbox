use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::lsattr::execute(&args) {
        eprintln!("lsattr: {}", e);
        process::exit(1);
    }
}
