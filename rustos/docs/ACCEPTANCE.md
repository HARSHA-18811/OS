# RustOS Comprehensive Acceptance Matrix

This checklist verifies every mandatory specification defined in the Master Prompt.

---

## 1. Goal & Deliverables

- [x] **ISO Image Compiled from Source**: Produces a bootable ISO image (`build/rustos.iso`, 8.58 MB).
- [x] **Runs in Oracle VirtualBox**: VirtualBox 7.x acts as the sandbox virtual machine environment.
- [x] **No QEMU Constraint**: Strictly zero usage or reference to QEMU across toolchain, scripts, or docs.
- [x] **Shell with Full Command Set**: Implements all 27 specified shell commands with exact names.
- [x] **Playable Games**: Desktop launcher with 3 games: Snake, Snake and Ladders, and Car Dodge.
- [x] **Producer-Consumer Research Lab**: Implements the bounded-buffer experiment based on the academic research report.

---

## 2. Environment & Toolchain

- [x] **Host Environment**: Windows 11 host with PowerShell 5.1/7+ and project root in `D:\OS\rustos`.
- [x] **ISO Toolchain**: Uses `grub-mkrescue`, `xorriso`, and `mtools` via WSL2 Ubuntu with containerized `Dockerfile` fallback.
- [x] **Pinned Rust Nightly**: Configured in `rust-toolchain.toml` with `rust-src` and `llvm-tools-preview`.
- [x] **Target Architecture**: 32-bit x86 bare-metal (`i686-rustos.json`) with `features: "-mmx,-sse"`, `rust-lld` linker, and `target-pointer-width: 32`.
- [x] **VirtualBox Integration**: Automates VM creation, configuration, serial port logging, and execution via `VBoxManage.exe`.

---

## 3. Boot, ISO, and VirtualBox Pipeline

- [x] **Multiboot Compliant ELF**: Magic `0x1BADB002`, flags `0x00000003`, checksum `0xE4524FFB` verified by GRUB.
- [x] **Linker Script**: Loads kernel ELF sections at physical address 1 MiB (`0x100000`).
- [x] **ISO Directory Layout**: Standard `iso/boot/grub/grub.cfg` with 3-second timeout and `iso/boot/kernel.elf`.
- [x] **VirtualBox Run Script (`run.ps1`)**:
  - Automatically creates VM: `createvm --name RustOS --ostype Other --register`
  - Configures VM: `--memory 128 --boot1 dvd --firmware bios`
  - Attaches SATA storage controller and DVD drive with `rustos.iso`
  - Configures COM1 UART serial port to output log to `build/serial.log`
  - Launches VM in GUI mode (`startvm RustOS --type gui`) or headless mode (`-Headless`)
- [x] **Manual VirtualBox Guide**: Fully documented in `docs/RUN_VIRTUALBOX.md`.

---

## 4. Kernel Core & Drivers

- [x] **`no_std` and `no_main`**: Pure bare-metal Rust implementation without standard library dependencies.
- [x] **Panic Handler**: Formats panic message in red on VGA display and outputs details over serial COM1.
- [x] **VGA Text Mode Driver**: 80x25 characters at `0xB8000`, 16 colors, hardware scrolling, `print!`/`println!` macros.
- [x] **Hardware Cursor**: Synchronized with VGA cursor registers at I/O ports `0x3D4` and `0x3D5`.
- [x] **Port I/O Helpers**: Inlined assembly functions `inb`, `outb`, `inw`, `outw`, and `io_wait` in `src/io.rs`.
- [x] **Global Descriptor Table (GDT)**: Flat 4 GB code (`0x08`) and data (`0x10`) segments loaded via `lgdt`.
- [x] **PIC Remapping**: 8259 master and slave PICs remapped to vectors `0x20..0x2F` to avoid CPU exception collisions.
- [x] **PIT Timer**: 8254 Channel 0 configured at 100 Hz (`10ms` per tick). Provides `sleep_ms(ms)` and `uptime_hms()`.
- [x] **IDT & Interrupt Handling**: 256-entry IDT with 48 assembly ISR stubs, CPU exception register dumps, and hardware IRQ dispatch.
- [x] **Keyboard Driver**: Interrupt-driven via IRQ1 with scancode ring buffer, Shift/Caps support, Enter/Backspace/Tab/Esc, and extended arrow keys (`0xE0`).
- [x] **CMOS Real-Time Clock**: Reads hardware time and date, handles BCD conversion, 12/24-hour modes, and computes day of week.
- [x] **Heap Allocator**: 2 MiB linked-list allocator with O(1) adjacent free block coalescing, enabling `Vec`, `String`, and `Box`.
- [x] **Serial COM1 Driver**: Configured at port `0x3F8` at 38400 baud, 8N1, mirroring all boot logs.
- [x] **SAFETY Comments**: Every single `unsafe` block includes an explicit `// SAFETY:` explanatory comment.

---

## 5. System Features

- [x] **Boot Log**: Step-by-step startup sequence displaying `[ OK ]` for each initialized subsystem.
- [x] **Visual Desktop Application Menu**: Visual menu with 7 choices (Shell, Snake, Snake & Ladders, Car Dodge, PC Lab, Files, About), navigated with Up/Down arrows and Enter.
- [x] **Status Bar**: Located at row 24 displaying active application name and real-time CMOS RTC clock.
- [x] **Command History**: Preserves at least 16 commands in memory, recallable via Up/Down arrow keys.

---

## 6. Shell Commands (27 Mandatory Commands)

- [x] `help`: Tabulated command listing.
- [x] `clear`: Clears VGA screen buffer.
- [x] `reboot`: Keyboard controller reset via port `0x64` with `0xFE`.
- [x] `shutdown`: Clean CPU halt (`cli; hlt`) with instructions to power off VM.
- [x] `date`: CMOS date, time, and day of week.
- [x] `cal`: Current month calendar with today highlighted.
- [x] `echo <text>`: Prints text to screen.
- [x] `echo <text> > <file>`: Writes or creates file, respecting read-only permissions.
- [x] `ls`: Lists files with permissions (`rw-` or `r--`), size, and name.
- [x] `lp <file>`: Displays content with `[lp] no printer attached` banner.
- [x] `whoami`: Outputs `root`.
- [x] `uname`: Outputs `RustOS 1.0.0 i686 (VirtualBox bare-metal)`.
- [x] `hostname`: Outputs `rustos`.
- [x] `uptime`: Displays elapsed time in `hh:mm:ss` from PIT ticks.
- [x] `bc <n> <op> <n>`: Evaluates `+`, `-`, `*`, `/`, `%` with divide-by-zero check and negative numbers.
- [x] `cat <file>`: Prints file content.
- [x] `grab <word> <file>`: Prints lines containing query word.
- [x] `rm <file>`: Deletes file (enforces read-only protection).
- [x] `touch <file>`: Creates empty file if missing.
- [x] `cp <src> <dst>`: Copies file.
- [x] `mv <src> <dst>`: Renames file (fails if destination exists).
- [x] `cut <start> <end> <f>`: Extracts character columns (1-based inclusive).
- [x] `head <file> [n]`: Displays first `n` lines (default 10).
- [x] `tail <file> [n]`: Displays last `n` lines (default 10).
- [x] `chmod <6|4> <file>`: Changes permission to writable (`6`) or read-only (`4`).
- [x] `wc <file>`: Counts lines, words, and bytes.
- [x] `vi <file>`: Interactive line editor (`:wq` saves, `:q` aborts).
- [x] `games`: Opens Desktop Menu.
- [x] `research` / `pclab`: Opens Producer-Consumer Performance Lab.
- [x] **Error Handling**: Displays usage on invalid arguments and prints all errors in red text.

---

## 7. In-Memory File System

- [x] **Capacity**: 32-entry file table, each supporting up to 2048 bytes.
- [x] **Metadata**: Name (max 24 chars), permission bit (`rw-` / `r--`), length, data buffer.
- [x] **Default Files**: Pre-populated at boot:
  - `readme.txt` (writable)
  - `notes.txt` (writable)
  - `todo.txt` (writable)
  - `about.txt` (read-only)

---

## 8. Playable Games Suite

- [x] **Snake**:
  - Box border drawn with VGA characters.
  - Arrow key directional controls.
  - Timed on PIT ticks (~80ms).
  - Growing body, score counter, and PRNG food spawning.
  - Game over on wall or self-collision; Enter restarts, Esc returns to menu.
- [x] **Snake and Ladders**:
  - 10x10 board (1..100) in serpentine layout.
  - Human vs Computer AI.
  - 6 ladders and 6 snakes.
  - Space/Enter rolls 6-sided dice; move log history.
  - First to 100 wins; Esc returns to menu.
- [x] **Car Dodge**:
  - 3-lane vertical road with animated divider markings.
  - Player car steered with Left/Right arrows.
  - Obstacles fall from top at accelerating speed.
  - Score based on survival time and passed obstacles; session high score tracked.
  - Esc returns to menu.

---

## 9. Producer-Consumer Research Lab

- [x] **Academic Model**: Bounded buffer with capacities $\{8, 32, 128, 512\}$.
- [x] **Configurations**: 24 distinct configurations across thread counts $\{1P/1C, 2P/1C, 1P/2C, 2P/2C, 4P/2C, 2P/4C\}$, run 5 times each.
- [x] **Cooperative Simulation**: Round-robin scheduler running simulated producer and consumer tasks with direct wait event counting.
- [x] **Limitation Statement**: Prominently displayed:
  > *"The OS version measures simulated tasks, not preemptive threads. Waiting times are counted directly, unlike the Rust prototype."*
- [x] **Lab Menu Options**:
  - Option 1: Run All (executes 24 configs, prints results table).
  - Option 2: Run Custom benchmark.
  - Option 3: View ASCII / Block bar chart for 2P/2C capacity scaling.
  - Option 4: Save results to `results.txt` in VFS.
  - Option 5: Automatic hypothesis evaluation (`H1`, `H2`, `H3`) with real data.
  - Option 6: Clean exit back to desktop or shell.
- [x] **Host-Side Prototype**: Standalone Cargo project in `research/rust-prototype/` with real multi-threaded OS threads, `results.csv`, runner scripts (`run_prototype.ps1`/`run_prototype.sh`), and `research/RESULTS_TEMPLATE.md` with populated empirical tables.

---

## 10. Verification & Deliverables

- [x] **Host Unit Tests**: `cargo test --package rustos_core` runs with 100% pass rate.
- [x] **Automated Boot Test**: `test_boot.ps1` runs headless VirtualBox test and verifies serial log.
- [x] **Documentation Package**: Complete documentation suite in `docs/` and root `README.md`.
- [x] **Deliverables Folder**: Bootable ISO (`rustos.iso`), source code archive (`source_code.zip`), and `SCREENSHOT_CHECKLIST.md`.
- [x] **Git Repository History**: Step-by-step milestone commits with verified builds.
