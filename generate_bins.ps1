# PowerShell script to generate standalone binary files for all commands

$commands = @(
    "doskey", "dumpe2fs", "e2fsck", "edlin", "egrep", "eject", "ethtool", "exec", "expr",
    "fc", "fdisk", "fg", "fgrep", "file", "finger", "fmt", "free", "fsck", "ftp",
    "getent", "getfacl", "groups", "gunzip", "gzip", "halt", "hash", "hdparm", "help",
    "hexdump", "history", "homectl", "host", "hostnamectl", "htop", "hwinfo", "iconv",
    "id", "ifconfig", "insmod", "install", "inxi", "ionice", "iostat", "iotop", "ip",
    "ipconfig", "iwconfig", "jobs", "journalctl", "kill", "killall", "label", "last",
    "ldd", "ldconfig", "less", "link", "locale", "localectl", "locate", "loginctl",
    "logger", "logname", "look", "lsattr", "lsblk", "lscpu", "lshw", "lsmod", "lsof",
    "lspci", "lsusb", "ltrace", "machinectl", "make", "md5sum", "meminfo", "mesg",
    "mkfifo", "mkfs", "mknod", "mkswap", "mktemp", "modprobe", "more", "mount", "mpstat",
    "netstat", "networkctl", "nice", "nm", "nohup", "nslookup", "numfmt", "objdump",
    "oomctl", "parted", "patch", "pathchk", "pathping", "perf", "pgrep", "pidstat",
    "ping", "pinky", "pkill", "pmap", "portablectl", "pr", "printcap", "printf", "ps",
    "ptx", "pwdx", "ranlib", "reboot", "renice", "reset", "resize2fs", "resolvectl",
    "rmmod", "route", "runcon", "sar", "script", "sed", "sensors", "service", "setfacl",
    "sha1", "sha1sum", "sha256sum", "sha384", "sha384sum", "sha512", "sha512sum",
    "shred", "shuf", "shutdown", "size", "slabtop", "smartctl", "ss", "stdbuf",
    "strace", "strip", "stty", "su", "suspend", "swapon", "swapoff", "sync", "sysctl",
    "systemctl", "tabs", "talk", "tar", "tcpdump", "telnet", "test", "time",
    "timedatectl", "timeout", "top", "traceroute", "tracert", "tsort", "tty", "tune2fs",
    "type", "udevadm", "ulimit", "umask", "umount", "uncompress", "unix2dos", "unlink",
    "unset", "unzip", "userdbctl", "vdir", "ver", "vmstat", "vol", "w", "wait", "wall",
    "watch", "wget", "where", "who", "whois", "write", "xargs", "zcat", "zip"
)

$template = @'
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = nullbox::command::{CMD}::execute(&args) {
        eprintln!("{CMD}: {}", e);
        process::exit(1);
    }
}
'@

foreach ($cmd in $commands) {
    $modName = $cmd.Replace("-", "_")
    $content = $template.Replace("{CMD}", $modName)
    $filePath = "src\bin\$cmd.rs"
    Set-Content -Path $filePath -Value $content -Encoding UTF8
    Write-Host "Created $filePath"
}

Write-Host "`nGenerated $($commands.Count) standalone binarie