use std::fs::File;
use std::io::{self, Read};
use base64::{Engine as _, engine::general_purpose};

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: base64 <file>"));
    }

    let mut f = File::open(&args[0])?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    
    let encoded = general_purpose::STANDARD.encode(&buffer);
    println!("{}", encoded);
    Ok(())
}
