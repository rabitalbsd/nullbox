use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("wlan0     IEEE 802.11  ESSID:\"Network\"");
    Ok(())
}
