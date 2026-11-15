# RustOS: Educational x86 Operating System & Research Lab

> **A bootable 32-bit x86 operating system written in pure Rust (`no_std`, `no_main`), featuring a visual desktop environment, an interactive UNIX-style shell, playable arcade and board games, and an in-kernel Producer-Consumer Performance Research Laboratory.**
> Built and verified inside **Oracle VirtualBox** as a bare-metal virtualization sandbox.

---

## 1. Project Overview

RustOS is built from the ground up to demonstrate modern systems programming fundamentals in Rust without relying on heavy third-party OS frameworks. It compiles to an ELF kernel, packs into a standard ISO image via GRUB Multiboot, and boots natively in VirtualBox.

### Core Specifications
- **Target Architecture**: 32-bit x86 (`i686`), Protected Mode, Flat 4 GB memory segmentation.
- **Boot Protocol**: Multiboot 1 / Multiboot 2 compliant ELF loaded at physical address 1 MiB (`0x100000`).
- **Bootloader**: GRUB 2 (`grub-mkrescue` hybrid ISO with BIOS el-torito boot).
- **Virtualization Sandbox**: Oracle VirtualBox (strictly non-QEMU).
- **Kernel Core**: VGA Text Mode (80x25), Serial COM1 (38400 baud), GDT, 8259 PIC, 8254 PIT (100 Hz), PS/2 Keyboard with extended scancodes, CMOS RTC, and a 2 MiB linked-list heap allocator with block coalescing.
- **User Interface**: Visual Desktop Application Menu with Up/Down keyboard navigation, dedicated status bar with real-time RTC clock and active application name. *(Note: Mouse support is not required in this version and is planned as a future step).*
- **Shell**: 27 built-in commands with argument validation, red error coloring, and command history.
- **In-Memory File System**: 32-file table with read/write/readonly permissions and pre-populated system files.
- **Games**: Snake arcade, 10x10 Snake and Ladders (vs Computer AI), and 3-lane Car Dodge.
- **Research Lab**: Built-in Producer-Consumer performance laboratory based on the academic paper *"Problem Identification: Producer-Consumer Performance"*, evaluating 24 configurations with live ASCII charts, hypothesis validation, and filesystem persistence.
- **Companion Prototype**: Host-side multi-threaded prototype in `research/rust-prototype/` with real measured data in `results.csv` and `research/RESULTS_TEMPLATE.md`.

---

## 2. Quick Start Guide

### Prerequisites
1. **Windows 11** with PowerShell 5.1+ or PowerShell 7+.
2. **Oracle VirtualBox 7.x** (installed at default path `C:\Program Files\Oracle\VirtualBox` or `D:\VirtualBox`).
3. **WSL2 (Ubuntu)** or Docker Desktop for `grub-mkrescue` and `xorriso`.
4. **Rust Nightly Toolchain** (`rustup toolchain install nightly-x86_64-pc-windows-msvc` with `rust-src` and `llvm-tools-preview`).

### 1-Click Build & Run (PowerShell)

Open PowerShell in the project directory (`D:\OS\rustos`):

```powershell
# 1. Build the bootable ISO (build/rustos.iso)
.\build.ps1

# 2. Launch RustOS in Oracle VirtualBox GUI
.\run.ps1
```

### Automated Headless Verification Test
To verify the entire build, boot, serial log, and driver chain in a headless VirtualBox VM without manual intervention:

```powershell
.\test_boot.ps1
```

### Running the Host-Side Research Prototype
To run the companion multi-threaded Producer-Consumer benchmark on your host CPU:

```powershell
.\research\run_prototype.ps1
```

---

## 3. Directory Layout

```
D:\OS\rustos\
├── Cargo.toml                  # Workspace and kernel crate configuration
├── rust-toolchain.toml         # Pinned nightly compiler specification
├── i686-rustos.json            # 32-bit x86 bare-metal target specification
├── linker.ld                   # Linker script (kernel loaded at 1 MiB)
├── build.ps1                   # 1-Click Windows PowerShell ISO build script
├── build.sh                    # Linux / WSL2 build script
├── run.ps1                     # Oracle VirtualBox VM automation script
├── test_boot.ps1               # Automated headless boot verification test
├── Dockerfile                  # Containerized build environment
│
├── iso/                        # GRUB ISO staging directory
│   └── boot/
│       ├── grub/grub.cfg       # GRUB boot menu configuration
│       └── kernel.elf          # Compiled kernel binary
│
├── src/                        # RustOS Kernel Source Code
│   ├── main.rs                 # Multiboot entry point & application loop
│   ├── io.rs                   # x86 Port I/O helpers (inb, outb, inw, outw)
│   ├── sync.rs                 # Custom Spinlock & SpinlockGuard primitives
│   ├── vga.rs                  # 80x25 VGA text mode driver & scrolling
│   ├── serial.rs               # COM1 UART serial console driver
│   ├── gdt.rs                  # Global Descriptor Table (flat 4GB)
│   ├── pic.rs                  # 8259 PIC remapping (IRQ 0..15)
│   ├── pit.rs                  # 8254 PIT channel 0 timer (100 Hz, sleep_ms)
│   ├── idt.rs                  # 256-entry Interrupt Descriptor Table & ISRs
│   ├── keyboard.rs             # PS/2 keyboard driver & scancode queue
│   ├── allocator.rs            # 2 MiB linked-list heap allocator with coalescing
│   ├── rtc.rs                  # CMOS Real-Time Clock driver (BCD/24h)
│   ├── fs.rs                   # In-memory file system (32 entries, max 2048B)
│   ├── shell.rs                # 27 command interactive shell & line editor
│   ├── statusbar.rs            # Row 24 status bar (app name & RTC clock)
│   ├── menu.rs                 # Visual Desktop Application Menu
│   ├── games/                  # Playable Games Suite
│   │   ├── mod.rs              # PRNG (Xorshift32) & game module exports
│   │   ├── snake.rs            # Snake arcade game
│   │   ├── snakes_ladders.rs   # 10x10 Snake & Ladders vs Computer
│   │   └── car_dodge.rs        # 3-lane highway reflex survival
│   └── research/               # In-Kernel Research Laboratory
│       ├── mod.rs              # Research lab module
│       └── lab.rs              # 24-config bounded buffer lab & ASCII charts
│
├── crates/rustos_core/         # Pure logic library (host & kernel shared)
│   ├── Cargo.toml
│   └── src/lib.rs              # bc, wc, cut, head, tail, calendar, unit tests
│
├── research/                   # Producer-Consumer Performance Research
│   ├── RESULTS_TEMPLATE.md     # Populated empirical result tables (Tables 1, 2, 3)
│   ├── run_prototype.ps1       # Prototype runner (PowerShell)
│   ├── run_prototype.sh        # Prototype runner (Bash)
│   └── rust-prototype/         # Multi-threaded standalone Rust prototype
│       ├── Cargo.toml
│       ├── results.csv         # 120 raw execution data points
│       └── src/main.rs         # Mutex+Condvar bounded buffer benchmark
│
├── docs/                       # Comprehensive System Documentation
│   ├── ARCHITECTURE.md         # Detailed hardware & software architecture
│   ├── COMMANDS.md             # Complete reference of all 27 shell commands
│   ├── GAMES.md                # Rules, controls, and scoring for all 3 games
│   ├── RESEARCH_LAB.md         # Academic research lab user guide & theory
│   ├── RUN_VIRTUALBOX.md       # Manual & automated VirtualBox guide
│   ├── ACCEPTANCE.md           # Requirements verification matrix
│   └── REPORT_NOTES.md         # Faculty explanatory notes & design decisions
│
└── deliverables/               # Final deliverables package
    ├── rustos.iso              # Bootable ISO deliverable
    ├── source_code.zip         # Source code archive
    └── SCREENSHOT_CHECKLIST.md # Required faculty screenshot guide
```

---

## 4. Key Features & Highlights

### 1. Boot, ISO & VirtualBox Integration
- Conforms to Multiboot specification (`0x1BADB002`).
- Boots via GRUB into 32-bit protected mode.
- Serial output mirrored to VirtualBox COM1 UART log (`build/serial.log`).

### 2. Robust Hardware Drivers
- **VGA Driver**: 80x25 text mode with hardware cursor synchronization (ports `0x3D4`/`0x3D5`) and reserved bottom row for status bar.
- **PIC Remapping**: 8259 PIC vectors remapped to `0x20..0x2F` to prevent collisions with CPU exceptions.
- **PIT Timer**: 8254 PIT running at 100 Hz (`10ms` per tick), driving system uptime and deterministic delays.
- **PS/2 Keyboard**: Interrupt-driven ring queue supporting alphanumeric keys, Shift, Enter, Backspace, Tab, Esc, and extended arrow keys (`0xE0`).
- **CMOS RTC**: Real-time clock reading with Update-In-Progress synchronization, BCD decoding, and 24-hour normalization.
- **Heap Allocator**: 2 MiB linked-list allocator with O(1) adjacent free block coalescing, enabling `Vec`, `String`, and `Box`.

### 3. Full 27-Command Shell
Includes exact implementations of:
`help`, `clear`, `reboot`, `shutdown`, `date`, `cal`, `echo` (with `>` file redirection), `ls`, `lp`, `whoami`, `uname`, `hostname`, `uptime`, `bc` (evaluator with negative numbers & divide-by-zero checks), `cat`, `grab`, `rm`, `touch`, `cp`, `mv`, `cut`, `head`, `tail`, `chmod`, `wc`, `vi` (in-memory line editor), `games`, and `research` / `pclab`.

### 4. Visual Desktop & Games
- **Menu System**: Visual interactive menu with Up/Down navigation, quick number keys, and status bar.
- **Snake**: Box arena, growing snake, score counter, and random food.
- **Snake & Ladders**: 100-cell serpentine board, dice rolling, ladders, snakes, and Computer AI.
- **Car Dodge**: 3-lane highway survival with increasing obstacle velocity and session high scores.

### 5. Producer-Consumer Research Lab
- Built-in performance evaluation of 24 bounded-buffer configurations ({8, 32, 128, 512} capacities × {1P/1C, 2P/1C, 1P/2C, 2P/2C, 4P/2C, 2P/4C} thread models).
- Interactive live ASCII bar charts.
- Automated hypothesis evaluation (`H1`, `H2`, `H3`) with empirical verification.
- Persistence to in-memory filesystem (`results.txt`).
- Companion host-side prototype generating real measured results (`results.csv`).

---

## 5. License & Academic Attribution
Developed as an educational operating system and systems engineering research demonstration. Designed for evaluation inside Oracle VirtualBox.
