# nullbox

A modern, cross-platform BusyBox-like utility with 250+ commands inspired by DOS, Linux, Unix, and modern tools.

## Platform Support

nullbox is designed to work seamlessly on:
- **Windows** (Windows 10/11, Server 2019+)
- **Unix** (FreeBSD, and other BSD variants)

The codebase uses conditional compilation to provide platform-specific implementations where needed while maintaining a consistent command-line interface across all platforms.

## Features

- 🚀 **250+ Commands** - Comprehensive toolset covering file operations, text processing, system info, networking, and more
- 🔄 **Cross-Platform** - Single codebase works on Windows and Unix systems
- 📦 **No GNU Dependencies** - Clean, modern Rust implementations
- ⚡ **Minimal Size** - Optimized with LTO and strip for small binaries
- 🔗 **BusyBox-Style** - Single binary with symlink support
- 🛠️ **Standalone Binaries** - Each command can be built separately

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repo-url>
cd nullbox

# Build release version
cargo build --release

# The binary will be in target/release/nullbox
```

### Build Specific Commands

```bash
# Build individual commands
cargo build --release --bin ls
cargo build --release --bin grep
cargo build --release --bin cat

# Or build all commands at once
cargo build --release
```

## Usage

### As Single Binary

```bash
# Use nullbox as a single binary
nullbox ls -la
nullbox grep pattern file.txt
nullbox cat -n file.txt
```

### As Standalone Commands

On Unix systems, create symlinks:
```bash
ln -s nullbox ls
ln -s nullbox grep
ln -s nullbox cat
```

On Windows, copy the binary:
```cmd
copy nullbox.exe ls.exe
copy nullbox.exe grep.exe
copy nullbox.exe cat.exe
```

## Command Categories

### File Operations (30+)
`cat`, `cp`, `mv`, `rm`, `del`, `mkdir`, `rmdir`, `touch`, `ln`, `link`, `file`, `stat`, `truncate`, `install`, `shred`, `split`, `dd`, `mkfifo`, `mknod`, `mktemp`, `unlink`, `readlink`, `realpath`, `basename`, `dirname`, `pathchk`, `attrib`, `dos2unix`, `unix2dos`

### Text Processing (35+)
`grep`, `egrep`, `fgrep`, `sed`, `awk`, `cut`, `sort`, `uniq`, `head`, `tail`, `wc`, `tr`, `rev`, `tac`, `nl`, `fold`, `expand`, `unexpand`, `paste`, `join`, `comm`, `column`, `colrm`, `col`, `fmt`, `pr`, `strings`, `xxd`, `hexdump`, `od`, `deroff`, `look`, `tsort`, `ptx`, `iconv`

### System Information (40+)
`uname`, `hostname`, `uptime`, `whoami`, `id`, `ps`, `top`, `htop`, `free`, `df`, `du`, `stat`, `lsblk`, `lsusb`, `lspci`, `lscpu`, `lshw`, `hwinfo`, `dmidecode`, `acpi`, `sensors`, `dmesg`, `vmstat`, `iostat`, `iotop`, `mpstat`, `pidstat`, `sar`, `perf`, `lsof`, `lsmod`, `lsattr`, `blkid`, `inxi`, `cpuinfo`, `meminfo`, `w`, `who`, `users`, `last`, `finger`, `pinky`

### Network Tools (25+)
`ping`, `curl`, `wget`, `netstat`, `ifconfig`, `ipconfig`, `ip`, `nslookup`, `dig`, `host`, `whois`, `arp`, `route`, `ss`, `ethtool`, `iwconfig`, `tcpdump`, `telnet`, `ftp`, `traceroute`, `tracert`, `pathping`, `resolvectl`, `networkctl`, `getent`

### Compression & Archives (15+)
`gzip`, `gunzip`, `zip`, `unzip`, `tar`, `compress`, `uncompress`, `zcat`

### Checksums & Hashing (15+)
`md5`, `md5sum`, `sha1`, `sha1sum`, `sha256`, `sha256sum`, `sha384`, `sha384sum`, `sha512`, `sha512sum`, `cksum`, `cksum256`, `crc32`, `b2sum`, `sum`

## Enhanced Commands

The following commands have been enhanced with full flag support:

### ls / dir
- `-l` - Long format with permissions and size
- `-a` - Show hidden files
- `-h` - Human-readable sizes (K, M, G)

### cat
- `-n` - Number all lines
- `-b` - Number non-blank lines
- `-E` - Show line endings with $

### grep
- `-i` - Case-insensitive search
- `-v` - Invert match
- `-n` - Show line numbers
- `-c` - Count matches only

### cp
- `-r` - Recursive copy
- `-v` - Verbose output
- `-i` - Interactive (prompt before overwrite)

### rm / del
- `-r` - Recursive delete
- `-f` - Force (no errors)
- `-v` - Verbose output
- `-i` - Interactive (prompt before delete)

### mkdir
- `-p` - Create parent directories
- `-v` - Verbose output

### mv
- `-v` - Verbose output
- `-i` - Interactive (prompt before overwrite)
- `-f` - Force overwrite

### wc
- `-l` - Count lines only
- `-w` - Count words only
- `-c` - Count bytes
- `-m` - Count characters

### head
- `-n N` - Show first N lines (default 10)
- `-c N` - Show first N bytes
- `-q` - Quiet (no headers)
- `-v` - Verbose (always show headers)

### tail
- `-n N` - Show last N lines (default 10)
- `-f` - Follow file (watch for changes)
- `-q` - Quiet (no headers)
- `-v` - Verbose (always show headers)

## Cross-Platform Notes

### File Permissions
- **Unix**: Full rwxrwxrwx permissions displayed
- **Windows**: Simplified r--/rw- based on readonly attribute

### File Paths
- Both forward slashes (/) and backslashes (\\) work on Windows
- Unix systems use forward slashes (/)

### Hidden Files
- **Unix**: Files starting with `.` are hidden
- **Windows**: Files starting with `.` are treated as hidden

### User Information
- **Unix**: Uses `USER` environment variable and UID/GID
- **Windows**: Uses `USERNAME` environment variable

## Building for Specific Platforms

### Windows
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### FreeBSD
```bash
cargo build --release --target x86_64-unknown-freebsd
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please ensure:
- Code works on both Windows and Unix
- Use the `platform` module for platform-specific operations
- Add tests for new commands
- Update documentation

## Architecture

The project uses conditional compilation (`#[cfg(windows)]` and `#[cfg(unix)]`) to provide platform-specific implementations while maintaining a unified API through the `platform` module.

```
nullbox/
├── src/
│   ├── command/      # 250+ command implementations
│   ├── platform.rs   # Cross-platform utilities
│   ├── main.rs       # Entry point and command dispatcher
│   └── lib.rs        # Library exports
├── Cargo.toml
└── build.rs          # Build script for binary generation
```
