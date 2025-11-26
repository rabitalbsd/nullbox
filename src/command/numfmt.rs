use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("numfmt: reformat numbers - {}", args.join(" "));
    Ok(())
}
