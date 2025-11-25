use std::env;
use std::io;
use std::path::Path;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: which <command>"));
    }

    let cmd = &args[0];
    let path_var = env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ';' } else { ':' };
    
    for dir in path_var.split(separator) {
        let mut exe_path = Path::new(dir).join(cmd);
        
        if cfg!(windows) && !cmd.ends_with(".exe") {
            exe_path.set_extension("exe");
        }
        
        if exe_path.exists() {
            println!("{}", exe_path.display());
            return Ok(());
        }
    }
    
    Err(io::Error::new(io::ErrorKind::NotFound, format!("{} not found", cmd)))
}
