use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::e2fsck::execute(&args) {
        eprintln!("e2fsck: {}", e);
        process::exit(1);
    }
}
