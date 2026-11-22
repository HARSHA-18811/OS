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
        | - Default Web Browser (Firefox)   |                 | - 1-Click Launch: Start-RustOS    |
        | - 1-Click Launch: Start-MicroOS   |                 |                                   |
        +-----------------------------------+                 +-----------------------------------+
```

---

## 2. ISO Artifact Download Links

| Artifact Name | Platform Architecture | Direct Download Link |
|:---|:---|:---|
| **`micro-os.iso`** (~557 MB) | Debian 12 Live GUI (VirtualBox & Physical Laptop USB) | [![Download on Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-Download%20ISO-blue)](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/micro-os.iso) |
| **`debian-micro-os-x86_64.iso`** (~557 MB) | Debian 12 Live GUI Mirror Link | [![Download on Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97%20Hugging%20Face-Direct%20Link-blue)](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/debian-micro-os-x86_64.iso) |
| **`rustos.iso`** (~15 MB) | Bare-Metal x86 Kernel (Multiboot) | [Download Mirror](https://drive.google.com/file/d/12ML_ZVn8thM8TBMi7EPKr-XWNeuxVwT9/view?usp=sharing) |

---

## 3. Key Operating System Features

### A. Windows 11 Desktop Ergonomics
- **Bottom Taskbar & Whisker Start Menu:** Pre-configured with quick launcher icons, system monitor tray, and clock.
- **Autologin:** Passwordless boot straight to desktop as non-root user `user` with `sudo` privileges.
- **Desktop Apps:**
  - **Default Web Browser (Firefox ESR):** Pre-configured modern web browser with desktop launcher and system-wide default MIME associations (starts offline until Wi-Fi is connected).
  - **Wi-Fi & Network Settings:** Interactive flyout and settings control center supporting live AP discovery and strict WPA2/WPA3 password validation.
  - **Producer-Consumer Research Lab:** Native multi-threaded Rust benchmark testbed evaluating lock contention and buffer capacities (`pclab`).
  - **Retro Arcade Snake Game:** Clean graphical arcade application running natively on the desktop.
  - **Micro OS Dashboard:** One-click system monitor showing memory, CPU load, and disk metrics.
  - **Terminal (Bash) & APT Package Manager:** Full Debian repository access via `apt`, `aptitude`, and `dpkg`.

### B. Authentic Wi-Fi & Network Manager
- **Independent Network Isolation:** The operating system boots completely disconnected and isolated by default (blocking WAN outgoing traffic) until the user enters their Wi-Fi security key.
- **Dedicated Wi-Fi Settings Application:** Standalone desktop app (`Wi-Fi Settings`) with a centered Windows 11-style control center window.
- **Taskbar Quick Flyout & System Tray:** Real-time Wi-Fi status icon in the bottom taskbar with quick connect/disconnect popup.
- **Master Wi-Fi Power Switch:** Instant ON/OFF toggle that actively controls OS networking traffic.
- **Nearby Access Point Discovery:** Pre-seeded with real nearby access points (`MOJO-JAN-C1-F8`, `MOJO-JAN-C1-F9`, `Harsha`, `OPPO F29 Pro 5G 232d`, `BHYRAVA.PG.3RD.FLOOR`, `Jain_Student`, `YAGNESH`, etc.) plus live dynamic `nmcli` scanning when booted on physical laptops.
- **Authentic Password Validation & DHCP IP Assignment:**
  - Selecting a network prompts for the WPA2/WPA3 security key with a reveal/hide password toggle.
  - Entering an incorrect password flags the field with a red border and displays: *"The network security key isn't correct. Please try again."*
  - Entering a valid password authenticates, establishes the connection, unblocks internet traffic, assigns DHCP IPv4 (`10.0.2.15`), and updates the taskbar icon to connected waves.
  - Disconnecting cuts off network traffic, returning the OS to an offline/disconnected state.
- **Physical USB Live Booting Support:** Pre-installed non-free wireless firmware (`firmware-iwlwifi`, `firmware-realtek`, `firmware-atheros`) inside kernel initramfs for physical laptops.

### C. Bare-Metal RustOS Kernel
- **Freestanding Compilation:** Compiled targeting `i686-unknown-none` with red-zone and SSE disabled.
- **Protected Mode GDT & 256-Entry IDT:** Catches hardware CPU exceptions (Div0, GPF, Page Faults) with full register dumps.
- **Hardware Drivers:** Dual 8259 PIC remapped to vectors `0x20-0x2F`, PIT timer scheduling at 100 Hz, memory-mapped VGA text buffer (`0xb8000`), and PS/2 keyboard controller.

---

## 4. One-Click Quick Start (Windows & VirtualBox)

### Launch Debian MicroOS:
Double-click `Start-MicroOS.bat` (or use Desktop shortcut **`Start MicroOS`**):
```powershell
.\Start-MicroOS.bat
```
*Configures VirtualBox with Intel PRO/1000 MT Desktop NIC, AHCI SATA controller (portcount 2 with host I/O cache enabled), 2048 MB RAM, and pops up the interactive GUI desktop window immediately.*

### Launch Bare-Metal RustOS:
Double-click `Start-RustOS.bat` or run in PowerShell:
```powershell
.\Start-RustOS.bat
```
*Boots the 15 MB freestanding bare-metal Rust kernel in VirtualBox, presenting the interactive kernel shell.*

### Live USB Booting on Physical Laptop:
1. Download **`micro-os.iso`** from the Hugging Face link above.
2. Use **Rufus** or **Ventoy** to flash the ISO to a USB flash drive.
3. Reboot your laptop, enter the BIOS/Boot Menu (F12, F9, or Esc), and select the USB drive.

---

## 5. Directory Structure
```
Micro_OS_Submission/
├── build/                      # Bootable live ISO artifact (micro-os.iso, 557 MB)
├── docs/
│   ├── screenshots/            # Verification screenshots
│   ├── word_documents/         # Editable Word documents (CA2, CA3, CA5, Lab, Project)
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
│   ├── package_gui_iso.sh      # Desktop packaging script
│   └── rebuild_iso.sh          # ISO remastering script
├── build.ps1                   # PowerShell build automation
├── build_clean.sh              # Debian live-build wrapper
├── run.ps1                     # PowerShell VirtualBox sandbox runner
├── Start-MicroOS.bat           # 1-Click Windows batch launcher
├── Start-RustOS.bat            # 1-Click Bare-metal kernel launcher
└── README.md                   # This documentation
```
