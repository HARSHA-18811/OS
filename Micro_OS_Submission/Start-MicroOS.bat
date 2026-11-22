@echo off
setlocal enabledelayedexpansion
title Starting Debian Micro OS...
echo ======================================================================
echo          Debian Micro OS (Windows 11-Style GUI & Wi-Fi)
echo ======================================================================
echo.

:: 1. Detect active and nearby Wi-Fi networks from host
echo [*] Scanning available Wi-Fi networks in your area...
netsh wlan show networks | findstr "SSID"
echo.

set "CURRENT_WIFI=Disconnected"
for /f "tokens=1,2 delims=:" %%a in ('netsh wlan show interfaces') do (
    if "%%a"=="    SSID                   " (
        set "RAW_SSID=%%b"
        set "CURRENT_WIFI=!RAW_SSID:~1!"
    )
)
echo [*] Active Laptop Wi-Fi: !CURRENT_WIFI!
echo [*] Launching MicroOS in VirtualBox...
echo.

cd /d "D:\VirtualBox"
VBoxManage.exe modifyvm "MicroOS" --boot1 dvd --boot2 disk --boot3 none --boot4 none --nicbootprio1 0 --nictype1 82540EM >nul 2>&1
VBoxManage.exe storagectl "MicroOS" --name "SATA" --portcount 2 --hostiocache on >nul 2>&1
start "" "VirtualBoxVM.exe" --startvm "MicroOS"
