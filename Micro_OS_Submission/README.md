# Debian MicroOS & Bare-Metal RustOS Kernel Submission (CA-6)

**Course:** 25PCCS201 – Operating Systems Concepts (3rd Semester B.Tech)  
**Author:** Yalagani Harsha Vardhan  
**USN:** 25BTRCL289  
**Section:** CSE AIML-E  
**Department:** Computer Science & Engineering (AIML), School of Engineering & Technology  

---

## 1. Executive Summary & Dual-Tier OS Architecture
This submission directory contains the complete source code, build scripts, drivers, and runtime assets for the **CA-6 Operating System Project**:

```
                                  +------------------------------------+
                                  |     MICRO OS DUAL-TIER PROJECT     |
                                  +-----------------+------------------+
                                                    |
                         +--------------------------+--------------------------+
                         |                                                     |
                         v                                                     v
        +-----------------------------------+                 +-----------------------------------+
        |      TIER 1: DEBIAN MICRO-OS      |                 |        TIER 2: BARE-METAL RUSTOS  |
        |  Live Graphical Linux Distribution|                 |  Freestanding x86 32-bit Kernel   |
        +-----------------------------------+                 +-----------------------------------+
        | - Minimal Debian 12 Bookworm      |                 | - #![no_std], #![no_main] in Rust |
        | - Windows 11 UI & Whisker Menu    |                 | - Flat Protected Mode GDT & IDT   |
        | - Authentic Wi-Fi Tray & Manager  |                 | - Dual 8259 PIC + PIT Timer       |
        | - Producer-Consumer Research Lab  |                 | - Memory-Mapped VGA Video (0xb8000|
        | - Retro Arcade Snake Desktop App  |                 | - PS/2 Keyboard Driver & Shell    |
        | - 1-Click Launch: Start-MicroOS   |                 | - 1-Click Launch: Start-RustOS    |
        +-----------------------------------+                 +-----------------------------------+
```

---

## 2. ISO Artifact Download Links

| Artifact Name | Platform Architecture | Direct Download Link |
|:---|:---|:---|
| **`debian-micro-os-x86_64.iso`** (~420 MB) | Debian 12 Live GUI (VirtualBox & Physical Laptop USB) | [![Download on Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-Download%20ISO-blue)](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/debian-micro-os-x86_64.iso) |
| **`rustos.iso`** (~15 MB) | Bare-Metal x86 Kernel (Multiboot) | [Download Mirror](https://drive.google.com/file/d/12ML_ZVn8thM8TBMi7EPKr-XWNeuxVwT9/view?usp=sharing) |

---

## 3. Key Operating System Features

### A. Windows 11 Desktop Ergonomics
- **Bottom Taskbar & Whisker Start Menu:** Pre-configured with quick launcher icons, system monitor tray, and clock.
- **Autologin:** Passwordless boot straight to desktop as non-root user `user` with `sudo` privileges.
- **Desktop Apps:**
  - **Wi-Fi & Network Settings:** Interactive flyout supporting live AP discovery and authentic WPA2 password validation.
  - **Producer-Consumer Research Lab:** Native multi-threaded Rust benchmark testbed evaluating lock contention and buffer capacities.
  - **Retro Arcade Snake Game:** Clean graphical arcade application running natively on the desktop.
  - **Micro OS Dashboard:** One-click system monitor showing memory, CPU load, and disk metrics.
  - **Terminal (Bash) & APT Package Manager:** Full Debian repository access via `apt`, `aptitude`, and `dpkg`.

### B. Authentic Wi-Fi & Network Manager
- **Single Taskbar Icon:** Clean Wi-Fi tray icon cleanly embedded in the XFCE panel.
- **Dynamic Access Point Discovery:** Scans and lists available wireless networks (e.g., `Jain_Student`, `MOJO-JAN-C1-F8`, `OPPO F29 Pro 5G`, `MOJO-JAN-C1-F9`).
- **Authentic Password Validation:**
  - Entering the correct password successfully authenticates, assigns DHCP IP (`10.0.2.15`), and updates tray icon to connected state.
  - Entering an incorrect password triggers a validation error rejection (*"Authentication failed: Incorrect network security key"*).

### C. Bare-Metal RustOS Kernel
- **Freestanding Compilation:** Compiled targeting `i686-unknown-none` with red-zone and SSE disabled.
- **Protected Mode GDT & 256-Entry IDT:** Catches hardware CPU exceptions (Div0, GPF, Page Faults) with full register dumps.
- **Hardware Drivers:** Dual 8259 PIC remapped to vectors `0x20-0x2F`, PIT timer scheduling at 100 Hz, memory-mapped VGA text buffer (`0xb8000`), and PS/2 keyboard controller.

---

## 4. One-Click Quick Start (Windows & VirtualBox)

### Launch Debian MicroOS:
Double-click `Start-MicroOS.bat` or run in PowerShell:
```powershell
.\Start-MicroOS.bat
```
*Configures VirtualBox with Intel PRO/1000 MT Desktop NIC, AHCI SATA controller (portcount 2 with host I/O cache enabled), 2048 MB RAM, and boots directly to the graphical desktop in full-screen mode.*

### Launch Bare-Metal RustOS:
Double-click `Start-RustOS.bat` or run in PowerShell:
```powershell
.\Start-RustOS.bat
```
*Boots the 15 MB freestanding bare-metal Rust kernel in VirtualBox, presenting the interactive kernel shell.*

---

## 5. Directory Structure
```
Micro_OS_Submission/
├── build/                      # Build artifacts and intermediate files
├── docs/
│   ├── screenshots/            # Verification screenshots (win_gui_final, snake_desktop_verified)
│   ├── PRESENTATION_GUIDE.md   # Faculty demonstration walkthrough
│   ├── BOOT_TIMES.md           # Component-by-component boot benchmark
│   └── HARDWARE_COMPATIBILITY.md
├── research/                   # Producer-Consumer Performance Lab in Rust
│   ├── Cargo.toml
│   └── src/main.rs
├── rustos/                     # Freestanding 32-bit Bare-Metal Rust Kernel
│   ├── src/                    # gdt, idt, pic, pit, vga, keyboard, shell
│   ├── Cargo.toml
│   ├── i686-rustos.json
│   ├── linker.ld
│   └── Start-RustOS.bat
├── scripts/
│   ├── apply_wifi_enhancements.sh
│   ├── micro-wifi.py           # Authentic Wi-Fi flyout GUI application
│   └── rebuild_iso.sh          # ISO remastering script
├── build.ps1                   # PowerShell build automation
├── build_clean.sh              # Debian live-build wrapper
├── run.ps1                     # PowerShell VirtualBox sandbox runner
├── Start-MicroOS.bat           # 1-Click Windows batch launcher
├── Start-RustOS.bat            # 1-Click Bare-metal kernel launcher
└── README.md                   # This documentation
```
