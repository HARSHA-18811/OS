# Micro OS: Source-to-ISO Architecture & Presentation Guide

## 1. Executive Summary
This document provides the formal explanation required for the academic presentation:
> **"Explain how source code is packaged into an ISO binary."**

Micro OS is an ultra-lightweight, laptop-compatible, hybrid 64-bit operating system distribution built from source code, minimal Debian userspace, and standard Linux kernel infrastructure. It embeds the **Producer-Consumer Performance Research Lab** as a native compiled system utility.

---

## 2. End-to-End Build Pipeline: From Source Code to Bootable ISO

```
+-------------------------------------------------------------------------------+
|                             STAGE 1: SOURCE CODE                              |
|  - Rust Research Lab (D:/OS/micro-os/research/src/main.rs)                    |
|  - Shell Launcher (D:/OS/micro-os/scripts/micro-os-launcher.sh)               |
|  - Systemd Service Overrides & Configuration Files                            |
+-------------------------------------------------------------------------------+
                                      |
                                      v [Cargo / Rustc Compiler]
+-------------------------------------------------------------------------------+
|                         STAGE 2: NATIVE COMPILATION                           |
|  - rustc / cargo build --release                                              |
|  - Output: x86_64 ELF 64-bit LSB PIE executable (`pclab`)                     |
|  - Stripped, statically optimized with zero external crate bloat              |
+-------------------------------------------------------------------------------+
                                      |
                                      v [Debootstrap / APT]
+-------------------------------------------------------------------------------+
|                    STAGE 3: ROOT FILESYSTEM COMPOSITION                       |
|  - Minimal Debian 12 (Bookworm) userspace (`minbase`)                         |
|  - Linux Kernel (`vmlinuz-amd64`) & Live Initramfs (`live-boot`)              |
|  - Hardware Drivers: NetworkManager, wpa_supplicant, WiFi/LAN firmware        |
|  - Package Management: APT & Aptitude                                         |
|  - App Store: Snapstore (`snapd`)                                             |
|  - Ingestion: `pclab` -> `/usr/local/bin/pclab`                               |
|  - Ingestion: `micro-os-launcher` -> `/usr/local/bin/micro-os-launcher`         |
|  - Configuration: Autologin on tty1 with automatic launcher execution         |
+-------------------------------------------------------------------------------+
                                      |
                                      v [mksquashfs -comp xz]
+-------------------------------------------------------------------------------+
|                      STAGE 4: SQUASHFS COMPRESSION                            |
|  - Read-only, highly compressed root filesystem: `filesystem.squashfs`        |
|  - Achieves ~4:1 compression ratio using high-density XZ dictionary           |
+-------------------------------------------------------------------------------+
                                      |
                                      v [grub-mkrescue / xorriso]
+-------------------------------------------------------------------------------+
|                       STAGE 5: HYBRID ISO PACKAGING                           |
|  - GRUB2 multiboot configuration (`grub.cfg`)                                 |
|  - El Torito boot catalog for Legacy BIOS (PC-BIOS stage)                     |
|  - EFI System Partition (ESP) with FAT filesystem for modern UEFI laptops     |
|  - Hybrid MBR partition table (allows raw flashing to USB via Rufus or dd)    |
|  - Output: `build/micro-os.iso`                                               |
+-------------------------------------------------------------------------------+
```

---

## 3. Detailed Stage Breakdown

### Stage 1 & 2: Source Code Compilation
1. **Source Code**:
   The custom research feature is authored in Rust (`src/main.rs`). It models a high-throughput concurrent bounded-buffer using POSIX synchronization primitives (`std::sync::Mutex`, `std::sync::Condvar`, `std::sync::atomic::AtomicBool`).
2. **Compilation**:
   `cargo build --release` invokes `rustc` with LLVM optimization level 3 (`-O3`), generating a fast machine-code binary targeting the `x86_64-unknown-linux-gnu` ABI.

### Stage 3: Root Filesystem Assembly
1. **Minimal Base**:
   `debootstrap` constructs a pristine, isolated Debian root filesystem (`minbase`), downloading only essential GNU C Library (`glibc`), coreutils, and bash.
2. **Kernel & Boot Hooks**:
   The official Linux kernel image (`linux-image-amd64`) and `live-boot` hooks are installed. The `live-boot` script modifies the initial RAM disk (`initrd.img`) so that at boot time, it searches optical discs and USB storage for the live squashfs image.
3. **Component Integration**:
   - **WiFi & LAN**: NetworkManager, `wpasupplicant`, wireless tools, and Linux firmware packages enable automatic DHCP over Ethernet and WPA2/WPA3 WiFi.
   - **Package Managers**: `apt` and `aptitude` provide Debian repository dependency resolution.
   - **Application Store**: `snapd` enables sandboxed containerized app distribution.
   - **Custom Binary Ingestion**: The compiled `pclab` binary is copied to `/usr/local/bin/pclab`.
   - **Autologin Configuration**: Systemd's `getty@tty1.service` is overridden to automatically log in the non-root user `user` without prompting for a password, immediately launching the interactive console menu.

### Stage 4: Read-Only Filesystem Compression (SquashFS)
- The entire assembled root filesystem is compressed using `mksquashfs` with `xz` compression into `iso/live/filesystem.squashfs`.
- At boot time, Linux mounts this read-only compressed image into an **OverlayFS** with a memory-backed (`tmpfs`) read-write upper layer. This ensures that any file modified during runtime stays in RAM and does not alter the underlying ISO binary.

### Stage 5: Hybrid ISO Mastering
- Standard ISO 9660 format was designed strictly for optical CDs. Modern hardware, however, boots via UEFI from USB flash drives or Legacy BIOS from virtual SATA controllers.
- `grub-mkrescue` combines:
  1. **El Torito BIOS boot catalog** for Legacy BIOS compatibility (VirtualBox default).
  2. **EFI System Partition (ESP)** containing `bootx64.efi` for modern laptop UEFI boot.
  3. **MBR Partition Table** written to sector 0 so USB drives appear as standard hard disks.
- The result is a single binary file (`micro-os.iso`) that can be booted directly inside VirtualBox or flashed onto a physical USB drive.

---

## 4. Boot Sequence Architecture (From Power-On to Lab)

1. **Firmware Initialisation (BIOS / UEFI)**:
   - System powers on. Firmware locates the boot device (DVD drive in VirtualBox or USB on laptop).
   - Reads sector 0 (MBR) or EFI bootloader (`bootx64.efi`).
2. **GRUB Bootloader**:
   - Displays the boot menu: `"Debian Micro OS (Live with Research Lab)"`.
   - Loads the Linux kernel (`/live/vmlinuz`) and initramfs (`/live/initrd.img`) into RAM.
3. **Kernel & Initramfs (`live-boot`)**:
   - Kernel initializes CPU, memory controllers, and ACPI tables.
   - Hands control to `initramfs`.
   - `live-boot` scans block devices, locates `/live/filesystem.squashfs`, and mounts it as the root filesystem via OverlayFS.
4. **Init Process (`systemd`)**:
   - Initializes system hardware drivers (PCI, USB, NICs).
   - Starts essential background daemons: `NetworkManager` (networking) and `snapd` (app store).
   - Spawns `getty@tty1`.
5. **Autologin & Welcome Launcher**:
   - `getty` automatically authenticates user `user`.
   - `~/.bashrc` detects `tty1` and executes `/usr/local/bin/micro-os-launcher`.
   - User is presented with the menu: Press `1` to run the Producer-Consumer Performance Lab!

---

## 5. Defense / Presentation Talking Points

| Question from Faculty | Technical Response |
| :--- | :--- |
| **"How did you create an ISO from source code?"** | "We compiled our Rust research algorithm into a standalone native binary (`pclab`). We then bootstrapped a minimal Linux kernel and userspace environment with drivers, package managers, and systemd. We compressed the filesystem into SquashFS and mastered it with GRUB2 and xorriso into a hybrid UEFI/BIOS ISO image." |
| **"Why is it called a 'Hybrid' ISO?"** | "Because it includes both an MBR El Torito record for legacy BIOS/VirtualBox and an EFI System Partition for modern laptops. It can be burned to a DVD or written byte-for-byte to a USB flash drive using `dd` or Rufus." |
| **"Where are APT, Aptitude, and Snapstore?"** | "They are pre-installed into the rootfs. APT and Aptitude are linked to the official Debian 12 Bookworm repositories. Snapstore is powered by `snapd.service` running in the background." |
| **"How does the OS handle WiFi and LAN?"** | "The Linux kernel loads network interface drivers, and `NetworkManager` runs as a system daemon. The user can type `3` in the launcher or run `nmtui` to connect to WiFi networks or Ethernet with automatic DHCP." |
| **"Where is the research lab in this OS?"** | "It is embedded in `/usr/local/bin/pclab`. Choosing option 1 in the launcher executes the research lab with real-time ASCII visualization and hypothesis verification." |
