use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Address                  HWtype  HWaddress           Flags Mask");
    println!("192.168.1.1              ether   00:11:22:33:44:55   C");
    Ok(())
}
