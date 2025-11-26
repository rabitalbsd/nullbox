use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("USER     TTY      FROM             LOGIN@   IDLE");
    println!("user     tty1     -                10:00    0.00s");
    Ok(())
}
