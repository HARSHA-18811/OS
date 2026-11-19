# Automated Headless Boot Test for RustOS in Oracle VirtualBox
$ErrorActionPreference = "Stop"

$ProjectRoot = $PSScriptRoot
Set-Location -Path $ProjectRoot

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "          RUSTOS AUTOMATED VIRTUALBOX BOOT TEST           " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Step 1: Build ISO
Write-Host "`n[Step 1/4] Building RustOS Bootable ISO..." -ForegroundColor Yellow
& "$ProjectRoot\build.ps1"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed with exit code $LASTEXITCODE"
}

# Step 2: Clear old serial log
$SerialLog = "$ProjectRoot\build\serial.log"
if (Test-Path $SerialLog) {
    Remove-Item -Force $SerialLog
}

# Step 3: Run headless in VirtualBox
Write-Host "`n[Step 2/4] Starting RustOS in VirtualBox (headless)..." -ForegroundColor Yellow
& "$ProjectRoot\run.ps1" -Headless -NoBuild
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to start VM"
}

Write-Host "[Step 3/4] Allowing 18 seconds for BIOS POST, GRUB, and kernel initialization..." -ForegroundColor Yellow
Start-Sleep -Seconds 18

# Locate VBoxManage
$VBoxManage = "VBoxManage"
if (Get-Command VBoxManage -ErrorAction SilentlyContinue) {
    $VBoxManage = "VBoxManage"
} elseif (Test-Path "D:\VirtualBox\VBoxManage.exe") {
    $VBoxManage = "D:\VirtualBox\VBoxManage.exe"
} elseif (Test-Path "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe") {
    $VBoxManage = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
}

Write-Host "Capturing screenshot of desktop screen..." -ForegroundColor Yellow
& $VBoxManage controlvm RustOS screenshotpng "$ProjectRoot\build\test_desktop_screen.png"

Write-Host "Powering off VirtualBox VM..." -ForegroundColor Yellow
& $VBoxManage controlvm RustOS poweroff
Start-Sleep -Seconds 2

# Step 4: Verify serial log
Write-Host "`n[Step 4/4] Verifying serial console log ($SerialLog)..." -ForegroundColor Yellow
if (-not (Test-Path $SerialLog)) {
    Write-Error "FAIL: Serial log file was not generated: $SerialLog"
}

$LogContent = Get-Content $SerialLog -Raw
Write-Host "--- Serial Log Output ---" -ForegroundColor DarkGray
Write-Host $LogContent -ForegroundColor White
Write-Host "-------------------------" -ForegroundColor DarkGray

$RequiredSteps = @(
    "Multiboot compliant bootloader detected",
    "VGA Text Mode Driver initialized (80x25)",
    "Serial COM1 Driver initialized (38400 baud)",
    "GDT initialized (flat 4GB code & data segments)",
    "PIC remapped (IRQ 0..7 -> 0x20..0x27, IRQ 8..15 -> 0x28..0x2F)",
    "PIT timer started at 100 Hz (10ms per tick)",
    "IDT initialized and CPU interrupts enabled",
    "Keyboard driver initialized (scancode queue active)",
    "Heap allocator initialized (2 MiB linked-list with coalescing)",
    "Dynamic memory allocation verified (Vec, String, Box)",
    "RTC CMOS initialized",
    "In-memory file system initialized",
    "Shell command engine verified (27 commands ready)",
    "Desktop launcher & Status bar initialized",
    "Games loaded (Snake, Snake & Ladders, Car Dodge)",
    "Producer-Consumer Research Lab initialized",
    "RustOS booted"
)

$AllPassed = $true
foreach ($step in $RequiredSteps) {
    if ($LogContent -match [regex]::Escape($step)) {
        Write-Host "  [PASS] $step" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] Missing step: $step" -ForegroundColor Red
        $AllPassed = $false
    }
}

if ($AllPassed) {
    Write-Host "`n==========================================================" -ForegroundColor Green
    Write-Host "      SUCCESS: ALL RUSTOS BOOT VERIFICATION TESTS PASSED!   " -ForegroundColor Green
    Write-Host "==========================================================" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`nFAIL: One or more boot verification checks failed." -ForegroundColor Red
    exit 1
}
