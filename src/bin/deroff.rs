use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::deroff::execute(&args) {
        eprintln!("deroff: {}", e);
        process::exit(1);
    }
}
