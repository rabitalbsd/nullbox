use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("NAME   MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT");
    println!("sda      8:0    0 238.5G  0 disk");
    Ok(())
}
