use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::resize2fs::execute(&args) {
        eprintln!("resize2fs: {}", e);
        process::exit(1);
    }
}
