use std::io;

pub fn execute(_args: &[String]) -> io::Result<()> {
    println!("avg-cpu:  %user   %nice %system %iowait  %steal   %idle");
    println!("           1.00    0.00    0.50    0.10    0.00   98.40");
    Ok(())
}
