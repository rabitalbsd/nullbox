use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("top - process monitor");
    println!("  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM");
    Ok(())
}
