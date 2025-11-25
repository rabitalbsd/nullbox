# PowerShell script to organize binaries into target/${profile}/bin
param(
    [string]$Profile = "release"
)

$targetDir = "target\$Profile"
$binDir = "$targetDir\bin"

# Create bin directory
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# List of all command binaries
$commands = @(
    "arch", "base64", "basename", "cat", "cd", "cksum", "clear", "clr", "cls",
    "comm", "cp", "cut", "date", "del", "df", "diff", "dir", "dirname", "du",
    "echo", "env", "expand", "factor", "false", "find", "fold", "grep", "head",
    "hostname", "join", "ln", "ls", "md5", "mkdir", "mv", "nl", "nproc", "od",
    "paste", "printenv", "pwd", "readlink", "realpath", "regex", "rev", "rm",
    "rmdir", "rx", "seq", "sha256", "sleep", "sort", "split", "stat", "strings",
    "sum", "tac", "tail", "tee", "touch", "tr", "tree", "true", "truncate",
    "uname", "unexpand", "uniq", "uptime", "wc", "which", "whoami", "xxd", "yes"
)

Write-Host "Organizing binaries into $binDir..." -ForegroundColor Green

# Copy nullbox main binary
if (Test-Path "$targetDir\nullbox.exe") {
    Copy-Item "$targetDir\nullbox.exe" "$binDir\nullbox.exe" -Force
    Write-Host "  [OK] nullbox.exe" -ForegroundColor Cyan
}

# Copy all command binaries
$copied = 0
$missing = 0

foreach ($cmd in $commands) {
    $exePath = "$targetDir\$cmd.exe"
    if (Test-Path $exePath) {
        Copy-Item $exePath "$binDir\$cmd.exe" -Force
        $copied++
    } else {
        Write-Host "  [SKIP] $cmd.exe (not found)" -ForegroundColor Yellow
        $missing++
    }
}

Write-Host "`nSummary:" -ForegroundColor Green
Write-Host "  Copied: $copied binaries" -ForegroundColor Cyan
Write-Host "  Missing: $missing binaries" -ForegroundColor Yellow
Write-Host "  Location: $binDir" -ForegroundColor Cyan
Write-Host "`nDone! All binaries are in: $binDir" -ForegroundColor Green
