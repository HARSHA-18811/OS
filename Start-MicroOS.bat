@echo off
title Starting Debian Micro OS...
echo ===================================================
echo   Starting Debian Micro OS (Windows-Style GUI)...
echo ===================================================
cd /d "D:\VirtualBox"
VBoxManage.exe discardstate "MicroOS" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/Fullscreen" "true" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/AutoresizeGuest" "true" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "GUI/LastGuestSizeHint" "1536,864" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "CustomVideoMode1" "1536x864x32" >nul 2>&1
VBoxManage.exe setextradata "MicroOS" "CustomVideoMode2" "1920x1080x32" >nul 2>&1
start "" "VirtualBoxVM.exe" --startvm "MicroOS" --fullscreen
