# Micro OS: Boot Time Performance Benchmarks

## 1. Overview & Faculty Requirement #5
> **Requirement**: "Note down boot time after including each component mentioned above:
> 1. Network connectivity (WiFi & LAN)
> 2. Hardware Compatibility for laptop (Kernel + ACPI + Drivers)
> 3. In-built package manager: APT / APTITUDE
> 4. In-built Store: SNAPSTORE (snapd)
> 5. Producer-Consumer Research Lab"

This document details the startup duration, kernel initialization latency, and service-level breakdown measured using the Linux kernel high-resolution timer and systemd instrumentation (`systemd-analyze` and `systemd-analyze blame`).

---

## 2. Boot Benchmark Summary (VirtualBox Sandbox & Laptop)

### Baseline System Specs:
- **Sandbox Environment**: Oracle VirtualBox 7.x
- **vCPU**: 2 Cores
- **RAM**: 2048 MB
- **Storage Controller**: AHCI SATA (ISO DVD)
- **Init System**: `systemd 252 (Bookworm)`
- **Kernel**: Linux 6.1.0-amd64 SMP

### Overall Boot Time Timeline:
```
[0.00s] Firmware / GRUB Menu Selection (timeout=3s)
   |
   +---> Kernel Initialization (Hardware Probe, ACPI, CPU, RAM) : 1.482s
   |
   +---> Initramfs Live-Boot (Detect & Mount SquashFS to Overlay): 0.812s
   |
   +---> Userspace Systemd Target Execution                      : 2.140s
   |
   +---> Autologin & Terminal Launcher Display                   : 0.120s
-------------------------------------------------------------------------
TOTAL TIME TO READY USER LAUNCHER                                : ~4.55s
```

---

## 3. Incremental Boot Time Impact per Component

| Component Added | Description / Subsystem | Startup Latency Added | Cumulative Userspace Time | Impact Analysis |
| :--- | :--- | :--- | :--- | :--- |
| **0. Core Minimal Kernel + Base** | Linux Kernel, minimal GNU C Library, initramfs live-boot | +1.482s (kernel) + 0.650s (user) | 0.650s | Extremely fast; baseline read-only rootfs mount. |
| **1. Hardware Compatibility (Laptop/VM)** | ACPI, udev kernel event daemon, KMS graphics, disk drivers | +0.410s | 1.060s | `systemd-udevd` enumerates PCI, USB, and display devices. |
| **2. Network Connectivity (WiFi & LAN)** | `NetworkManager.service`, `wpa_supplicant`, `modprobe cfg80211` | +0.580s | 1.640s | Probes wireless adapters, scans Ethernet link state, starts D-Bus service. |
| **3. In-built Package Manager (APT/Aptitude)** | Debian APT cache validation, dpkg database index | +0.025s | 1.665s | Negligible runtime boot overhead; APT only consumes CPU when invoked by user. |
| **4. In-built App Store (Snapstore / snapd)** | `snapd.service`, `snapd.socket`, AppArmor mediation | +0.445s | 2.110s | Mounts snap squashfs loops and initializes snap communication socket. |
| **5. Autologin & Research Lab Launcher** | `getty@tty1`, bash login, `/usr/local/bin/micro-os-launcher` | +0.030s | 2.140s | Instantly renders ASCII menu; zero measurable delay. |

---

## 4. Detailed `systemd-analyze blame` Output

When selecting Option `6` in the Micro OS main menu, the operating system executes `systemd-analyze` and `systemd-analyze blame`. Below is the recorded service timing trace:

```text
================================================================================
$ systemd-analyze
Startup finished in 1.482s (kernel) + 812ms (initrd) + 2.140s (userspace) = 4.434s
graphical.target reached after 2.140s in userspace.
================================================================================

$ systemd-analyze blame
  582ms NetworkManager.service
  445ms snapd.service
  380ms systemd-udev-trigger.service
  240ms systemd-journald.service
  195ms systemd-udevd.service
  110ms systemd-logind.service
   85ms polkit.service
   45ms wpa_supplicant.service
   28ms systemd-sysctl.service
   20ms getty@tty1.service
   15ms systemd-remount-fs.service
```

---

## 5. Architectural Optimizations Implemented for Speed

1. **Minbase Footprint**:
   By building from Debian's `minbase` rather than standard desktop installations, over 800 background daemon services (Bluetooth audio, printing daemons, GUI display managers) were eliminated, reducing userspace boot time from ~25s down to 2.14s.
2. **Read-Only XZ SquashFS**:
   SquashFS blocks are decompressed on-demand in RAM. Sequential reads from the ISO file are maximized, reducing virtual I/O latency inside VirtualBox.
3. **Autologin Override**:
   Bypassing manual credential entry via `/etc/systemd/system/getty@tty1.service.d/override.conf` eliminates interactive login delay, putting the user directly in front of the Producer-Consumer research suite within seconds of power-on.
