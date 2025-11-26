use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("update-rc.d: install/remove init scripts - {}", args.join(" "));
    Ok(())
}
