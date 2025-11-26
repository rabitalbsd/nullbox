use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>");
    println!("      inet 192.168.1.100");
    Ok(())
}
