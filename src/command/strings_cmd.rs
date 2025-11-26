use std::io;

pub fn execute(args: &[String]) -> io::Result<()> {
    crate::command::strings::execute(args)
}
