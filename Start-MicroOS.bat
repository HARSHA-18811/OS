@echo off
setlocal enabledelayedexpansion
title Starting Debian Micro OS...
echo ===================================================
echo   Starting Debian Micro OS (Windows-Style GUI)...
echo ===================================================

:: Detect active Wi-Fi SSID from Windows
set "CURRENT_WIFI=Jain_Student"
for /f "tokens=1,2 delims=:" %%a in ('netsh wlan show interfaces') do (
    if "%%a"=="    SSID                   " (
        set "RAW_SSID=%%b"
        set "CURRENT_WIFI=!RAW_SSID:~1!"
    )
)
echo [INFO] Detected Active Wi-Fi: !CURRENT_WIFI!

cd /d "D:\VirtualBox"
VBoxManage.exe discardstate "MicroOS" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/Fullscreen" "true" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/AutoresizeGuest" "true" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/LastGuestSizeHint" "1536,864" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "CustomVideoMode1" "1536x864x32" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "CustomVideoMode2" "1920x1080x32" >nul 2>&1
start "" "VirtualBoxVM.exe" --startvm "MicroOS" --fullscreen
