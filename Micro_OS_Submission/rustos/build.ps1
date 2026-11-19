<#
.SYNOPSIS
    Builds the RustOS kernel and packages it into a bootable ISO.
#>
param (
    [switch]$Release
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# Try to detach any mounted ISO in VirtualBox to release file locks
$vboxManage = $null
if (Get-Command VBoxManage -ErrorAction SilentlyContinue) {
    $vboxManage = "VBoxManage"
} elseif (Test-Path "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe") {
    $vboxManage = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
} elseif (Test-Path "D:\VirtualBox\VBoxManage.exe") {
    $vboxManage = "D:\VirtualBox\VBoxManage.exe"
}
if ($vboxManage) {
    try {
        & $vboxManage storageattach RustOS --storagectl "SATA" --port 0 --device 0 --type dvddrive --medium "none" 2>$null
    } catch {}
}

Write-Host "=== Step 1: Compiling RustOS Kernel ===" -ForegroundColor Cyan
$buildArgs = @("build", "-Zjson-target-spec", "--package", "rustos")
if ($Release) {
    $buildArgs += "--release"
}
& cargo @buildArgs
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

$targetSubdir = if ($Release) { "release" } else { "debug" }
$kernelBinary = Join-Path $ScriptDir "target\i686-rustos\$targetSubdir\rustos"
$isoBootDir = Join-Path $ScriptDir "iso\boot"
$kernelElf = Join-Path $isoBootDir "kernel.elf"
$buildDir = Join-Path $ScriptDir "build"
$isoOutput = Join-Path $buildDir "rustos.iso"

if (-not (Test-Path $isoBootDir)) {
    New-Item -ItemType Directory -Path $isoBootDir -Force | Out-Null
}
if (-not (Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir -Force | Out-Null
}

Write-Host "=== Step 2: Copying kernel ELF to ISO directory ===" -ForegroundColor Cyan
Copy-Item -Path $kernelBinary -Destination $kernelElf -Force

Write-Host "=== Step 3: Generating Bootable ISO with grub-mkrescue ===" -ForegroundColor Cyan

# Try to dismount if mounted in Windows
Get-DiskImage -ImagePath $isoOutput -ErrorAction SilentlyContinue | Dismount-DiskImage -ErrorAction SilentlyContinue

# Remove existing ISO if possible
if (Test-Path $isoOutput) {
    Remove-Item $isoOutput -Force -ErrorAction SilentlyContinue
}

# Check if WSL is available
$hasWsl = Get-Command wsl -ErrorAction SilentlyContinue
$hasDocker = Get-Command docker -ErrorAction SilentlyContinue

if ($hasWsl) {
    Write-Host "Building ISO using WSL (Ubuntu)..." -ForegroundColor Green
    $drive = $ScriptDir.Substring(0, 1).ToLower()
    $subPath = $ScriptDir.Substring(2) -replace '\\', '/'
    $wslPath = "/mnt/$drive$subPath"
    & wsl -d Ubuntu sh -c "cd '$wslPath' && grub-mkrescue -o build/rustos.iso iso"
} elseif ($hasDocker) {
    Write-Host "Building ISO using Docker container..." -ForegroundColor Green
    & docker build -t rustos-builder .
    & docker run --rm -v "${ScriptDir}:/work" rustos-builder
} else {
    Write-Error "Neither WSL with grub-mkrescue nor Docker is available to run grub-mkrescue. Please install WSL Ubuntu or Docker."
    exit 1
}

if (Test-Path $isoOutput) {
    $isoSize = (Get-Item $isoOutput).Length
    Write-Host "=== SUCCESS: $isoOutput ($([math]::Round($isoSize / 1MB, 2)) MB) generated! ===" -ForegroundColor Green
} else {
    Write-Error "Failed to generate $isoOutput"
    exit 1
}
