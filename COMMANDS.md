# nullbox - 250+ Commands

A modern BusyBox-like utility with 250+ commands inspired by DOS, Linux, Unix, and modern tools.

## Command Categories

### File Operations (30+)
cat, cp, mv, rm, del, mkdir, rmdir, touch, ln, link, file, stat, truncate, install, shred, split, csplit, dd, mkfifo, mknod, mktemp, unlink, readlink, realpath, basename, dirname, pathchk, attrib, dos2unix, unix2dos

### Text Processing (35+)
grep, egrep, fgrep, sed, awk, cut, sort, uniq, head, tail, wc, tr, rev, tac, nl, fold, expand, unexpand, paste, join, comm, column, colrm, col, fmt, pr, strings, xxd, hexdump, od, deroff, look, tsort, ptx, iconv

### System Information (40+)
uname, hostname, uptime, whoami, id, ps, top, htop, free, df, du, stat, lsblk, lsusb, lspci, lscpu, lshw, hwinfo, dmidecode, acpi, sensors, dmesg, vmstat, iostat, iotop, mpstat, pidstat, sar, perf, lsof, lsmod, lsattr, blkid, inxi, cpuinfo, meminfo, w, who, users, last, finger, pinky

### Process Management (20+)
kill, killall, pkill, pgrep, ps, top, htop, nice, renice, nohup, timeout, bg, fg, jobs, wait, suspend, pmap, pwdx, strace, ltrace

### Network Tools (25+)
ping, curl, wget, netstat, ifconfig, ipconfig, ip, nslookup, dig, host, whois, arp, route, ss, ethtool, iwconfig, tcpdump, telnet, ftp, traceroute, tracert, pathping, resolvectl, networkctl, getent

### Compression & Archives (15+)
gzip, gunzip, zip, unzip, tar, compress, uncompress, zcat, b2sum

### Checksums & Hashing (15+)
md5, md5sum, sha1, sha1sum, sha256, sha256sum, sha384, sha384sum, sha512, sha512sum, cksum, cksum256, crc32, b2sum, sum

### Disk & Filesystem (25+)
fdisk, parted, mkfs, fsck, e2fsck, tune2fs, resize2fs, dumpe2fs, mount, umount, swapon, swapoff, mkswap, df, du, sync, hdparm, smartctl, badblocks, lsblk, blkid

### System Administration (30+)
systemctl, service, loginctl, timedatectl, hostnamectl, localectl, journalctl, bootctl, machinectl, portablectl, homectl, userdbctl, coredumpctl, oomctl, busctl, udevadm, sysctl, chkconfig, update-rc.d, shutdown, reboot, halt, su, sudo

### User & Permissions (15+)
chmod, chown, chgrp, chattr, chroot, umask, getfacl, setfacl, id, whoami, logname, groups, users, who, w

### Development Tools (15+)
make, ar, ranlib, nm, objdump, strip, size, ldd, ldconfig, strace, ltrace, perf, gdb

### Shell Utilities (20+)
echo, printf, test, expr, alias, history, jobs, bg, fg, wait, exec, type, which, where, hash, help, script, tee, xargs, watch

### DOS Commands (15+)
dir, del, copy, move, type, cls, ver, vol, label, attrib, comp, fc, doskey, edlin

### Modern Utilities (10+)
curl, wget, jq, base32, base64, basenc, numfmt, shuf, stdbuf, runcon

## Building

```bash
cd nullbox
cargo build --release
```

## Usage

### As Single Binary
```bash
nullbox <command> [args...]
nullbox ls -la
nullbox grep pattern file.txt
```

### As Standalone Commands
Build individual command binaries:
```bash
cargo build --release --bin cat
cargo build --release --bin grep
# Or build all at once
cargo build --release
```

## Features

- **No GNU dependencies** - Clean, modern implementations
- **Cross-platform** - Windows and FreeBSD support
- **Minimal size** - Optimized with LTO and strip
- **250+ commands** - Comprehensive toolset
- **Standalone binaries** - Each command can be built separately
- **BusyBox-style** - Single binary with symlink support

## License

MIT
