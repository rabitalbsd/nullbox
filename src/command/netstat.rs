use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Active Internet connections");
    println!("Proto Recv-Q Send-Q Local Address           Foreign Address");
    Ok(())
}
