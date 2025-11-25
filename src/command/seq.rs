use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: seq <last> or seq <first> <last>"));
    }

    let (start, end) = if args.len() == 1 {
        (1, args[0].parse().unwrap_or(1))
    } else {
        (args[0].parse().unwrap_or(1), args[1].parse().unwrap_or(1))
    };

    for i in start..=end {
        println!("{}", i);
    }
    Ok(())
}
