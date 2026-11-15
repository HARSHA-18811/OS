@echo off
title Starting Debian Micro OS...
echo ===================================================
echo   Starting Debian Micro OS (Windows-Style GUI)...
echo ===================================================
cd /d "D:\VirtualBox"
VBoxManage.exe discardstate "MicroOS" >nul 2>&1
start "" "VirtualBoxVM.exe" --startvm "MicroOS"
