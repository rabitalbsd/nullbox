use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::md5sum::execute(&args) {
        eprintln!("md5sum: {}", e);
        process::exit(1);
    }
}
