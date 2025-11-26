use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----");
    println!(" r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st");
    Ok(())
}
