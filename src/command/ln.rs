use std::io;

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

#[cfg(windows)]
use std::os::windows::fs as windows_fs;

pub fn execute(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "usage: ln <target> <link>"));
    }

    let target = &args[0];
    let link = &args[1];

    #[cfg(unix)]
    unix_fs::symlink(target, link)?;

    #[cfg(windows)]
    {
        if std::path::Path::new(target).is_dir() {
            windows_fs::symlink_dir(target, link)?;
        } else {
            windows_fs::symlink_file(target, link)?;
        }
    }

    Ok(())
}
