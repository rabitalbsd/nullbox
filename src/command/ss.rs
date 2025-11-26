use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Netid  State      Recv-Q Send-Q Local Address:Port");
    Ok(())
}
