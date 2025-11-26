use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("ptx: permuted index - {}", args.join(" "));
    Ok(())
}
