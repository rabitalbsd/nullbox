use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    let file = if args.is_empty() { "typescript" } else { &args[0] };
    println!("script: recording to {}", file);
    Ok(())
}
