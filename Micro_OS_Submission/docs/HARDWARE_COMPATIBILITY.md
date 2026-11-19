# Micro OS: Laptop Hardware Compatibility & Installation Guide

## 1. Overview
Micro OS is engineered for dual compatibility:
1. **Virtual Machine Sandbox**: Oracle VirtualBox (via Legacy BIOS or UEFI, SATA/IDE controller).
2. **Physical Laptop Hardware**: Boots on modern 64-bit x86 laptops (Intel Core, AMD Ryzen) with full support for WiFi, Ethernet, USB 3.0, and display output.

---

## 2. Hardware Driver Matrix

| Component | Laptop Hardware Supported | Driver / Subsystem in Micro OS |
| :--- | :--- | :--- |
| **CPU Architecture** | Intel 64-bit / AMD64 (Dual, Quad, Octa-Core) | Linux SMP Kernel (`linux-image-amd64`) |
| **Firmware Mode** | Modern UEFI (64-bit) & Legacy BIOS / CSM | GRUB2 Hybrid Boot (`grub-efi-amd64` + `grub-pc`) |
| **Wireless (WiFi)** | Intel AX200/AX201, Realtek 8821CE/8822CE, Qualcomm Atheros, Broadcom | `wpa_supplicant`, `wireless-tools`, `firmware-linux-free` |
| **Wired (LAN)** | Realtek RTL8111/8168, Intel e1000e, Killer Ethernet | Kernel gigabit drivers, `isc-dhcp-client` |
| **Display / Graphics** | Intel Iris Xe / UHD, AMD Radeon Vega / RDNA, basic VESA/KMS | Direct Linux DRM / KMS Framebuffer |
| **Storage / Media** | USB 2.0 / 3.0 Flash Drives, NVMe SSDs, SATA HDDs | `live-boot` OverlayFS via RAM (`tmpfs`) |
| **Input Devices** | Laptop Built-in PS/2 Keyboards, Touchpads, USB Keyboards & Mice | Linux input event subsystem (`evdev`) |

---

## 3. How to Flash Micro OS to a Physical USB Drive

To boot Micro OS on your physical laptop, flash `micro-os.iso` to a USB drive (at least 2 GB):

### Method A: Using Rufus (Windows - Recommended)
1. Insert your USB flash drive into your laptop.
2. Download and run **Rufus** (https://rufus.ie).
3. Select your USB drive under **Device**.
4. Click **SELECT** and choose `D:\OS\micro-os\build\micro-os.iso`.
5. Under **Partition scheme**:
   - Select **MBR** (compatible with both BIOS and UEFI).
   - Target system: **BIOS or UEFI**.
6. Click **START**. When prompted, choose **Write in ISO Image mode (Recommended)**.
7. Wait for completion.

### Method B: Using Ventoy (Multi-boot USB)
1. Format a USB drive with **Ventoy** (https://www.ventoy.net).
2. Simply copy `micro-os.iso` directly into the USB drive's root folder.
3. Boot the laptop and select `micro-os.iso` from the Ventoy menu.

---

## 4. Booting on Your Physical Laptop

1. Insert the flashed USB drive into the laptop.
2. Power off the laptop completely.
3. Turn on the laptop and immediately tap the **Boot Menu Key** repeatedly:
   - **Lenovo**: `F12` or `Fn + F12`
   - **HP**: `F9` or `Esc`
   - **Dell**: `F12`
   - **ASUS**: `F8` or `Esc`
   - **Acer**: `F12`
4. In the boot menu, select the USB drive (e.g., `UEFI: SanDisk...` or `USB HDD`).
5. The GRUB bootloader appears:
   - Select `"Debian Micro OS (Live with Research Lab)"`.
6. Within 5–10 seconds, the kernel initializes and automatically logs you in to `tty1`, launching the interactive menu.

---

## 5. Network Configuration on Physical Laptop

### Connecting to WiFi:
1. In the Micro OS main launcher, press `3` (Network & WiFi Configuration).
2. This opens the **NetworkManager Text User Interface (`nmtui`)**.
3. Use the arrow keys to select **Activate a connection**.
4. Locate your home or college WiFi network (SSID).
5. Press **Enter**, type your WiFi password, and select **OK**.
6. The connection is established immediately, and an IP address is assigned via DHCP.
7. To test connectivity, select option `2` (Open Shell) and run:
   ```bash
   ping -c 3 google.com
   ```

### Connecting via Ethernet (LAN):
- Simply plug the RJ45 Ethernet cable into your laptop. `NetworkManager` will automatically detect the link and obtain an IP address via DHCP within 2 seconds.
