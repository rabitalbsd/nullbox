use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::crc32::execute(&args) {
        eprintln!("crc32: {}", e);
        process::exit(1);
    }
}
