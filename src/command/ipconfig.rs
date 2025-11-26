use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Windows IP Configuration");
    println!("\nEthernet adapter:");
    println!("   IPv4 Address: 192.168.1.100");
    Ok(())
}
