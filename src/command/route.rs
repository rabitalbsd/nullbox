use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Kernel IP routing table");
    println!("Destination     Gateway         Genmask         Flags Metric Ref");
    Ok(())
}
