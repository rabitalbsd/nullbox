use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: factor <number>"));
    }

    let mut n: u64 = args[0].parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid number"))?;
    
    print!("{}:", n);
    
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            print!(" {}", d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        print!(" {}", n);
    }
    println!();
    Ok(())
}
