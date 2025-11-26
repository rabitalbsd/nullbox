use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::dumpe2fs::execute(&args) {
        eprintln!("dumpe2fs: {}", e);
        process::exit(1);
    }
}
