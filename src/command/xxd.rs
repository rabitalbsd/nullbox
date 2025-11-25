use std::fs::File;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: xxd <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    for (i, chunk) in buffer.chunks(16).enumerate() {
        print!("{:08x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in chunk.len()..16 {
            print!("   ");
        }
        print!(" ");
        for byte in chunk {
            let c = *byte as char;
            print!("{}", if c.is_ascii_graphic() || c == ' ' { c } else { '.' });
        }
        println!();
    }
    Ok(())
}
