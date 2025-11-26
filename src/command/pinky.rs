use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("Login    Name                 TTY      Idle  When");
    println!("user     User Name            tty1           Nov 26");
    Ok(())
}
