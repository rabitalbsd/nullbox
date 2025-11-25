use std::io;
use std::thread;
use std::time::Duration;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: sleep <seconds>"));
    }

    let seconds: u64 = args[0].parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid number"))?;
    
    thread::sleep(Duration::from_secs(seconds));
    Ok(())
}
