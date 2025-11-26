use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("   November 2025");
    println!("Su Mo Tu We Th Fr Sa");
    println!("                   1");
    println!(" 2  3  4  5  6  7  8");
    println!(" 9 10 11 12 13 14 15");
    println!("16 17 18 19 20 21 22");
    println!("23 24 25 26 27 28 29");
    println!("30");
    Ok(())
}
