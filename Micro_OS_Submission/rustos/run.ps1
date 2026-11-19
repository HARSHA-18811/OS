<#
.SYNOPSIS
    Creates and launches the RustOS VirtualBox virtual machine.
#>
param (
    [switch]$Headless,
    [switch]$NoBuild
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# Locate VBoxManage.exe
$vboxManage = $null
if (Get-Command VBoxManage -ErrorAction SilentlyContinue) {
    $vboxManage = "VBoxManage"
} elseif (Test-Path "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe") {
    $vboxManage = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
} elseif (Test-Path "D:\VirtualBox\VBoxManage.exe") {
    $vboxManage = "D:\VirtualBox\VBoxManage.exe"
} else {
    $reg = Get-ItemProperty 'HKLM:\SOFTWARE\Oracle\VirtualBox' -ErrorAction SilentlyContinue
    if ($reg -and $reg.InstallDir -and (Test-Path (Join-Path $reg.InstallDir "VBoxManage.exe"))) {
        $vboxManage = Join-Path $reg.InstallDir "VBoxManage.exe"
    }
}

if (-not $vboxManage) {
    Write-Error "Could not find VBoxManage.exe. Please install Oracle VirtualBox or add it to PATH."
    exit 1
}

Write-Host "Using VBoxManage: $vboxManage" -ForegroundColor Cyan

$isoPath = Join-Path $ScriptDir "build\rustos.iso"
$buildDir = Join-Path $ScriptDir "build"
$serialLog = Join-Path $buildDir "serial.log"

if (-not $NoBuild -or -not (Test-Path $isoPath)) {
    Write-Host "Building RustOS ISO..." -ForegroundColor Cyan
    & .\build.ps1
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed."
        exit $LASTEXITCODE
    }
}

# Check if VM already exists
$vmList = & $vboxManage list vms
$vmExists = $vmList -match '"RustOS"'

if (-not $vmExists) {
    Write-Host "Creating Virtual Machine 'RustOS'..." -ForegroundColor Cyan
    & $vboxManage createvm --name RustOS --ostype Other --register
}

Write-Host "Configuring Virtual Machine settings..." -ForegroundColor Cyan
& $vboxManage modifyvm RustOS --memory 128 --boot1 dvd --boot2 disk --boot3 none --boot4 none --nicbootprio1 0 --nictype1 82540EM --firmware bios

# Clear serial log if exists
if (Test-Path $serialLog) {
    Remove-Item $serialLog -Force -ErrorAction SilentlyContinue
}

# Configure COM1 serial port to write to build/serial.log
& $vboxManage modifyvm RustOS --uart1 0x3F8 4 --uart-mode1 file "$serialLog"

# Ensure SATA storage controller is attached
try {
    & $vboxManage storagectl RustOS --name "SATA" --add sata --controller IntelAhci --bootable on 2>$null
} catch {}

# Attach ISO to DVD drive
& $vboxManage storageattach RustOS --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "$isoPath"

$vmType = if ($Headless) { "headless" } else { "gui" }
Write-Host "Starting RustOS Virtual Machine (type: $vmType)..." -ForegroundColor Green
& $vboxManage startvm RustOS --type $vmType
