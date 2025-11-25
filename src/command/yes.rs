use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let msg = if args.is_empty() { "y" } else { &args[0] };
    
    loop {
        println!("{}", msg);
    }
}
