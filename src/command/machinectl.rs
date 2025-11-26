use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    println!("machinectl: control VMs/containers - {}", args.join(" "));
    Ok(())
}
