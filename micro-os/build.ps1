<#
.SYNOPSIS
    Builds the Debian Micro OS Bootable ISO using WSL2 Ubuntu.
#>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "         DEBIAN MICRO OS AUTOMATED ISO BUILDER            " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Check if WSL is available
if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) {
    Write-Error "WSL is not available on this machine. Please enable WSL."
    exit 1
}

$wslScriptPath = "/mnt/d/OS/micro-os/scripts/build_iso.sh"
Write-Host "`nLaunching build in WSL root environment..." -ForegroundColor Yellow
wsl -u root /mnt/d/OS/micro-os/scripts/build_iso.sh

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

$isoPath = Join-Path $ScriptDir "build\micro-os.iso"
if (Test-Path $isoPath) {
    $item = Get-Item $isoPath
    $sizeMB = [math]::Round($item.Length / 1MB, 2)
    Write-Host "`n==========================================================" -ForegroundColor Green
    Write-Host " [SUCCESS] ISO generated: $isoPath ($sizeMB MB)" -ForegroundColor Green
    Write-Host "==========================================================" -ForegroundColor Green
} else {
    Write-Error "ISO was not found at $isoPath"
    exit 1
}
