use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("ltrace: library call tracer - {}", args.join(" "));
    Ok(())
}
