# RustOS Academic Report & Faculty Technical Notes

This document provides technical commentary and design justifications for university faculty evaluating the RustOS codebase, architecture, and empirical research deliverables.

---

## 1. Fulfillment of Faculty Core Requirement

> **Faculty Specification**: *"Collect the source code, make it an ISO, and run it virtually in a sandbox."*

RustOS directly fulfills each element of this specification:
1. **Source Code**: Fully modular, pure Rust bare-metal repository (`D:\OS\rustos`), structured with clean separation of concerns (`src/`, `crates/rustos_core/`, `research/`).
2. **Bootable ISO Deliverable**: The automated build pipeline generates `build/rustos.iso` (and `deliverables/rustos.iso`), an El-Torito hybrid bootable ISO image containing a Multiboot-compliant ELF binary booted via GRUB 2.
3. **Virtual Sandbox Execution**: Strictly executed within **Oracle VirtualBox 7.x** without relying on QEMU, verifying execution on realistic virtualized PC hardware (BIOS, 8259 PIC, 8254 PIT, 16550 UART COM1, standard VGA framebuffers, PS/2 controllers).

---

## 2. Key Architectural Decisions & Trade-Offs

### 2.1 32-Bit x86 Protected Mode (`i686`) vs 64-Bit Long Mode
- **Decision**: RustOS targets 32-bit x86 protected mode (`i686-rustos.json`) rather than 64-bit x86_64 long mode.
- **Justification**:
  - Educational clarity: 64-bit long mode requires setting up 4-level or 5-level paging tables before entering C/Rust code. In an educational OS project, this often leads developers to rely on pre-packaged bootloader crates (such as `bootloader 0.9` or `bootimage`) which hide foundational low-level mechanics.
  - Direct Multiboot compliance: Standard Multiboot 1 bootloaders (like GRUB) jump directly to 32-bit protected mode with paging disabled and a flat 4 GB segment model. RustOS handles its own stack setup, GDT loading, PIC remapping, and IDT configuration directly in transparent Rust code without opaque intermediate stubs.

### 2.2 Linked-List Allocator with Coalescing vs Simple Bump Allocator
- **Decision**: Implemented an in-kernel doubly-linked free-list allocator with instant adjacent block coalescing, backed by a 2 MiB static BSS buffer.
- **Justification**:
  - A bump allocator is trivial to write but cannot free memory. In an OS running interactive shell sessions, file editing (`vi`), and a 24-configuration research lab that creates and drops dynamic buffers, a bump allocator would rapidly deplete memory and crash the kernel.
  - The free-list allocator with 16-byte alignment and $O(1)$ coalescing prevents external fragmentation while maintaining predictable heap performance. Placing the heap in a dedicated static BSS buffer guarantees that heap allocations can never collide with kernel code or the execution stack.

### 2.3 Non-QEMU Poweroff & Hardware Reset
- **Decision**: Avoided QEMU-specific debug exit ports (e.g. `isa-debug-exit` on port `0x501`).
- **Justification**:
  - VirtualBox does not provide QEMU debug exit ports. RustOS reboots using the standard IBM PC keyboard controller reset line (`0xFE` to port `0x64`). For shutdown, it cleanly flushes serial output, displays poweroff instructions, and executes `cli; hlt`. VirtualBox VM power management is automated externally via `VBoxManage controlvm RustOS poweroff`.

---

## 3. The Producer-Consumer Research Lab: Methodology & Limitations

### 3.1 The Academic Context
The lab implements the experimental design described in the research report *"Problem Identification: Producer-Consumer Performance"*. The experiment evaluates bounded-buffer performance across:
- **Capacities**: 8, 32, 128, 512
- **Thread Ratios**: 1P/1C, 2P/1C, 1P/2C, 2P/2C, 4P/2C, 2P/4C (24 configurations × 5 runs)

### 3.2 Dual Implementation Strategy
Because preemptive multi-threading on single-core bare-metal requires complex timer-driven context switching and register frame spilling that can introduce measurement artifacts, the research was implemented across two levels:

1. **Host-Side Multi-Threaded Prototype (`research/rust-prototype/`)**:
   - Executes real native threads (`std::thread::spawn`) on the host CPU.
   - Measures real OS scheduling, lock contention (`Mutex`), and condition variable wakeups (`Condvar`).
   - Generates empirical data saved to `research/rust-prototype/results.csv` and summarized in `research/RESULTS_TEMPLATE.md`.

2. **In-Kernel Educational Lab (`src/research/lab.rs`)**:
   - Runs deterministic cooperative tasks inside RustOS in VirtualBox.
   - Measures PIT timer ticks and counts exact stall events directly.

### 3.3 The Prominently Displayed Limitation Notice
In compliance with academic honesty, the in-kernel lab explicitly displays:
> *"The OS version measures simulated tasks, not preemptive threads. Waiting times are counted directly, unlike the Rust prototype."*

This distinction is crucial for faculty review: in real multi-threaded operating systems, thread waiting times are difficult to isolate due to CPU scheduling latency, whereas in a discrete event cooperative simulation, every buffer-full or buffer-empty stall can be counted with 100% precision.

---

## 4. Verification and Reproducibility

Faculty members can verify the entire project through automated scripts:
1. `test_boot.ps1`: Headless VirtualBox test verifying all hardware subsystems and boot completion.
2. `cargo test --package rustos_core`: Host unit tests validating arithmetic evaluation, calendar algorithms, and text utilities.
3. `research/run_prototype.ps1`: Host benchmark generating fresh CSV results.
4. `run.ps1`: Launches the interactive VirtualBox VM to test the Shell, Games, and Research Lab directly.
