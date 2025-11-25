# PowerShell script to build all binaries and organize them
param(
    [switch]$Release,
    [switch]$Debug
)

$profile = "debug"
if ($Release) {
    $profile = "release"
}

Write-Host "Building nullbox with profile: $profile" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# Build command
$buildCmd = "cargo build"
if ($Release) {
    $buildCmd += " --release"
}

Write-Host "`nStep 1: Building all binaries..." -ForegroundColor Cyan
Invoke-Expression $buildCmd

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nStep 2: Organizing binaries..." -ForegroundColor Cyan
    & .\organize_bins.ps1 -Profile $profile
    
    Write-Host "`n========================================" -ForegroundColor Green
    Write-Host "Build complete!" -ForegroundColor Green
    Write-Host "Binaries location: target\$profile\bin" -ForegroundColor Cyan
} else {
    Write-Host "`nBuild failed!" -ForegroundColor Red
    exit 1
}
