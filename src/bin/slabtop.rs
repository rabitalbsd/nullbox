use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::slabtop::execute(&args) {
        eprintln!("slabtop: {}", e);
        process::exit(1);
    }
}
