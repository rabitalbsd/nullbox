use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::cksum256::execute(&args) {
        eprintln!("cksum256: {}", e);
        process::exit(1);
    }
}
