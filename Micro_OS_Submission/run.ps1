<#
.SYNOPSIS
    Creates and launches the Debian Micro OS VirtualBox virtual machine.
#>
param (
    [switch]$Headless,
    [switch]$NoBuild,
    [switch]$Restart
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# Locate VBoxManage.exe
$vboxManage = $null
if (Get-Command VBoxManage -ErrorAction SilentlyContinue) {
    $vboxManage = "VBoxManage"
} elseif (Test-Path "D:\VirtualBox\VBoxManage.exe") {
    $vboxManage = "D:\VirtualBox\VBoxManage.exe"
} elseif (Test-Path "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe") {
    $vboxManage = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
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

$isoPath = Join-Path $ScriptDir "build\micro-os.iso"
$buildDir = Join-Path $ScriptDir "build"
$serialLog = Join-Path $buildDir "serial.log"

if (-not $NoBuild -or -not (Test-Path $isoPath)) {
    Write-Host "Building Debian Micro OS ISO..." -ForegroundColor Cyan
    & .\build.ps1
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed."
        exit $LASTEXITCODE
    }
}

# Check if VM is currently running
$runningVMs = & $vboxManage list runningvms
if ($runningVMs -match '"MicroOS"') {
    if ($Restart) {
        Write-Host "MicroOS is currently running. Stopping it cleanly..." -ForegroundColor Yellow
        & $vboxManage controlvm MicroOS poweroff
        Start-Sleep -Seconds 2
    } else {
        Write-Host "======================================================================" -ForegroundColor Green
        Write-Host " [INFO] MicroOS is ALREADY RUNNING on your desktop!" -ForegroundColor Green
        Write-Host " Look at your taskbar for the window titled: 'MicroOS [Running]'" -ForegroundColor Yellow
        Write-Host " Or switch to it using Alt + Tab." -ForegroundColor Yellow
        Write-Host " (To force restart it from scratch, run: .\run.ps1 -NoBuild -Restart)" -ForegroundColor Cyan
        Write-Host "======================================================================" -ForegroundColor Green
        exit 0
    }
}

# Check if VM already exists
$vmList = & $vboxManage list vms
$vmExists = $vmList -match '"MicroOS"'

if (-not $vmExists) {
    Write-Host "Creating Virtual Machine 'MicroOS' (Debian 64-bit)..." -ForegroundColor Cyan
    & $vboxManage createvm --name MicroOS --ostype Debian_64 --register
}

Write-Host "Configuring Virtual Machine settings..." -ForegroundColor Cyan
& $vboxManage modifyvm MicroOS --memory 2048 --cpus 2 --boot1 dvd --nic1 nat --nictype1 82540EM --graphicscontroller vboxsvga --vram 64

# Clear serial log if exists
if (Test-Path $serialLog) {
    Remove-Item $serialLog -Force -ErrorAction SilentlyContinue
}

# Configure COM1 serial port to write to build/serial.log
& $vboxManage modifyvm MicroOS --uart1 0x3F8 4 --uart-mode1 file "$serialLog"

# Ensure SATA storage controller is attached
try {
    & $vboxManage storagectl MicroOS --name "SATA" --add sata --controller IntelAhci --bootable on 2>$null
} catch {}

# Attach ISO to DVD drive
& $vboxManage storageattach MicroOS --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "$isoPath"

$vmType = if ($Headless) { "headless" } else { "gui" }
Write-Host "Starting MicroOS Virtual Machine (type: $vmType)..." -ForegroundColor Green
& $vboxManage startvm MicroOS --type $vmType
