use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing device"));
    }
    println!("resize2fs: resize ext2/ext3/ext4 filesystem");
    Ok(())
}
