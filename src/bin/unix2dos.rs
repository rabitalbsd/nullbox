use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::unix2dos::execute(&args) {
        eprintln!("unix2dos: {}", e);
        process::exit(1);
    }
}
