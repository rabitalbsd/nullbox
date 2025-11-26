use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    crate::command::ls::execute(args)
}
