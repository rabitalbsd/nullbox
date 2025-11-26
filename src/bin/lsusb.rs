use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::lsusb::execute(&args) {
        eprintln!("lsusb: {}", e);
        process::exit(1);
    }
}
