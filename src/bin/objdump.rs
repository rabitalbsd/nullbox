use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::objdump::execute(&args) {
        eprintln!("objdump: {}", e);
        process::exit(1);
    }
}
