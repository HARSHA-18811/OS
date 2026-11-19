@echo off
title Starting RustOS (Bare-Metal 32-bit Kernel)...
echo ======================================================================
echo              RustOS Bare-Metal Kernel (32-bit x86)
echo ======================================================================
echo.
echo [*] Discarding any stale VM saved states...
cd /d "D:\VirtualBox"
VBoxManage.exe discardstate "RustOS" >nul 2>&1
VBoxManage.exe modifyvm "RustOS" --boot1 dvd --boot2 disk --boot3 none --boot4 none --nicbootprio1 0 --nictype1 82540EM >nul 2>&1
VBoxManage.exe storagectl "RustOS" --name "SATA" --portcount 2 --hostiocache on >nul 2>&1
echo [*] Launching RustOS in VirtualBox...
start "" "VirtualBoxVM.exe" --startvm "RustOS"
