# Debian Micro OS
### Hybrid UEFI/BIOS Bootable Operating System with Native Producer-Consumer Concurrency Research Lab

---

## 1. Project Overview
**Micro OS** is a custom minimal, hybrid bootable operating system built to satisfy all academic requirements set forth by faculty:

1. **Network Connectivity**: Full support for both **WiFi** (WPA/WPA2/WPA3 via `wpa_supplicant` and `NetworkManager`) and **LAN** (wired Ethernet via DHCP).
2. **Laptop Hardware Compatibility**: Hybrid ISO image bootable on modern physical laptops (via UEFI USB flash drive) and in Oracle VirtualBox sandbox (via Legacy BIOS / SATA).
3. **In-Built Package Managers**: Full support for Debian package management via **APT** and **APTITUDE** (`apt`, `apt-get`, `aptitude`).
4. **In-Built Application Store**: Pre-installed **Snapstore** infrastructure (`snapd`).
5. **Boot Time Performance Benchmarking**: Documented startup duration per component using kernel timers and `systemd-analyze`.
6. **Presentation Packaging Architecture**: Detailed explanation of how high-level source code is compiled, containerized, and mastered into an ISO binary.
7. **Producer-Consumer Performance Research Lab**: Custom multi-threaded concurrent bounded-buffer research lab written in Rust, integrated natively into the OS as a core system command (`pclab` / `research`).

---

## 2. Directory Layout
```
D:\OS\micro-os\
├── build\
│   └── micro-os.iso                <-- Bootable Hybrid ISO (VirtualBox + Laptop USB)
├── docs\
│   ├── PRESENTATION_GUIDE.md       <-- Detailed guide for faculty presentation
│   ├── BOOT_TIMES.md               <-- Incremental boot time measurements & analysis
│   ├── HARDWARE_COMPATIBILITY.md   <-- Flashing guide for USB & real laptop boot
│   └── RESEARCH_PAPER_ANALYSIS.md  <-- 24 benchmark configs & hypotheses evaluation
├── research\                       <-- Source code for Academic Feature
│   ├── Cargo.toml                  <-- Rust build configuration
│   └── src\
│       └── main.rs                 <-- High-throughput Bounded-Buffer Lab in Rust
├── scripts\
│   ├── build_iso.sh                <-- Automated end-to-end ISO construction pipeline
│   └── micro-os-launcher.sh        <-- Interactive ASCII system launcher (tty1)
├── build.ps1                       <-- One-click PowerShell build script
├── run.ps1                         <-- One-click PowerShell VirtualBox sandbox runner
└── README.md                       <-- This document
```

---

## 3. How to Run in Oracle VirtualBox (Sandbox)

Micro OS includes an automated PowerShell script that configures the VM, attaches the ISO, and boots it with one command:

```powershell
# From Windows PowerShell (Run as Administrator or standard user)
cd D:\OS\micro-os
.\run.ps1
```

### Note on VirtualBox Mouse/Keyboard Capture:
When VirtualBox opens for the first time, it may display an informational dialog saying:
> *"You have clicked the mouse inside the Virtual Machine display or pressed the host key combo. This will cause the Virtual Machine to capture the host mouse pointer..."*

* **Check the box**: `Do not show this message again`
* **Click**: `Capture`
* If you ever need to release your mouse cursor back to Windows, press the **Right Ctrl** key on your keyboard.
* Use the **Arrow Keys** and **Number Keys (1-8)** to interact with the Micro OS launcher.

---

## 4. How to Boot on a Physical Laptop

1. Plug a USB flash drive (>= 2 GB) into your PC.
2. Open **Rufus** (https://rufus.ie).
3. Select your USB drive and choose `D:\OS\micro-os\build\micro-os.iso`.
4. Partition scheme: **MBR** | Target system: **BIOS or UEFI**.
5. Click **START** (Write in ISO mode).
6. Insert the USB drive into your laptop, reboot, and press your boot menu key (`F12`, `F9`, or `Fn+F12`).
7. Select your USB drive to boot directly into Micro OS.

---

## 5. Main Launcher Features

Upon boot, the system automatically logs in and presents the interactive dashboard:

```text
================================================================================
       __  __ _                    ____   _____ 
      |  \/  (_)                  / __ \ / ____|
      | \  / |_  ___ _ __ ___    | |  | | (___  
      | |\/| | |/ __| '__/ _ \   | |  | |\___ \ 
      | |  | | | (__| | | (_) |  | |__| |____) |
      |_|  |_|_|\___|_|  \___/    \____/|_____/ 
                                                
   Debian 12 Bookworm (Minimal) - Laptop Compatible - Live ISO
================================================================================
  Hardware Architecture : x86_64 (UEFI / Legacy BIOS Hybrid)
  Kernel                : Linux 6.1+ (WiFi, LAN & Laptop Drivers Loaded)
  Package Manager       : APT & APTITUDE (Debian Mirrors Configured)
  App Container Store   : SNAPSTORE (snapd service active)
  Research Lab          : Producer-Consumer Performance Lab (pre-installed)
================================================================================

  [1] Producer-Consumer Performance Research Lab (Academic Feature)
  [2] Open Shell Terminal (bash)
  [3] Network & WiFi Configuration (NetworkManager / nmtui)
  [4] Package Manager: APT / Aptitude Demonstration
  [5] App Store: Snapstore / snapd Demonstration
  [6] Boot Performance Benchmark (systemd-analyze)
  [7] System Hardware & Laptop Specs (lshw, lspci, ip)
  [8] Power Off / Reboot System
================================================================================
```

- **Option 1**: Launches the **Producer-Consumer Performance Research Lab**. Run single tests or the complete 24-benchmark suite (`--benchmark`).
- **Option 2**: Enters an interactive `bash` shell with root privileges (`sudo`).
- **Option 3**: Launches `nmtui` to connect to WiFi networks or view Ethernet status.
- **Option 4**: Demonstrates `apt` and `aptitude` package managers.
- **Option 5**: Demonstrates `snapd` / Snapstore.
- **Option 6**: Displays kernel and userspace boot duration (`systemd-analyze blame`).
- **Option 7**: Displays CPU, RAM, network interfaces, and PCI hardware devices.
- **Option 8**: Clean shutdown or reboot.
