# Submissions for OS CA2, CA3, CA5 & CA6

**Course:** 25PCCS201 – Operating Systems Concepts (3rd Semester B.Tech)  
**Author:** Yalagani Harsha Vardhan  
**USN:** 25BTRCL289  
**Section:** CSE AIML-E  
**Department:** Computer Science & Engineering (AIML), School of Engineering & Technology  

---

## 1. Portfolio Navigation & Assessment Components

| Directory / File | Assessment Component | Description & Scope |
|:---|:---:|:---|
| [📁 **`Micro_OS_Submission/`**](./Micro_OS_Submission) | **CA-6 Project Submission** | Dual-tier Operating System: Minimal bootable Debian MicroOS Live GUI (Windows 11 UI, authentic Wi-Fi management with password authentication, Snake arcade, system monitor) and 32-bit Bare-Metal RustOS Kernel. |
| [📄 **`FINAL__CA2_Producer_Consumer_Performance_Investigation.docx`**](./FINAL__CA2_Producer_Consumer_Performance_Investigation.docx) | **CA-2 Investigation** | *How Buffer Size, Thread Ratios, and the Linear Scaling Fallacy Influence Producer–Consumer Performance* (Rust prototype). |
| [📄 **`FINAL__CA3_OS_Research_Paper_Producer_Consumer_Performance.docx`**](./FINAL__CA3_OS_Research_Paper_Producer_Consumer_Performance.docx) | **CA-3 Debugging Challenge** | *An Empirical Investigation into Bounded Producer–Consumer Synchronization: Diagnosing Concurrency Defects and Analyzing Buffer Scaling*. |
| [📄 **`final_CA5_Producer_Consumer_Linux_Systems_Engineering_Final.docx`**](./final_CA5_Producer_Consumer_Linux_Systems_Engineering_Final.docx) | **CA-5 Systems Engineering** | *Investigating the Impact of Bounded Buffer Sizing and Thread Allocation Ratios on Concurrent Producer–Consumer Throughput: A Linux Systems Engineering Empirical Study*. |
| [📄 **`25BTRCL289_Harsha_Vardhan_Research.docx`**](./25BTRCL289_Harsha_Vardhan_Research.docx) | **Research Proposal** | Problem Identification, Literature Discovery, and Formal Research Formulation. |
| [📄 **`Operating_Systems_Lab_Assignment_Harsha.docx`**](./Operating_Systems_Lab_Assignment_Harsha.docx) | **Lab Record Portfolio** | Laboratory Record: Comprehensive Linux system programming, concurrency models, and architecture simulations. |
| [📄 **`Operating_Systems_Project_Report.docx`**](./Operating_Systems_Project_Report.docx) | **Project Report** | Academic Project Report covering Debian MicroOS Live GUI, Bare-Metal RustOS Kernel, and Concurrency Lab (also available as [PDF inside `Micro_OS_Submission/`](./Micro_OS_Submission/Operating_Systems_Project_Report.pdf)). |

---

## 2. MicroOS & RustOS Architecture Overview (CA-6)

### A. Debian MicroOS Live GUI
- **Lightweight Distribution:** Engineered from a clean Debian 12 Bookworm base (<450 MB ISO).
- **Windows 11 Ergonomics:** Bottom taskbar, Whisker Start menu, system tray, and dark theme.
- **Authentic Wi-Fi Subsystem:** Integrated taskbar tray flyout with dynamic AP scanning (`Jain_Student`, `MOJO-JAN-C1-F8`, `OPPO F29 Pro 5G`, `MOJO-JAN-C1-F9`) and real WPA2 password validation.
- **Academic Feature:** Integrated Producer–Consumer Performance Research Lab (`pclab`) with 24-configuration benchmark matrix.
- **Desktop Apps:** Snake game, APT Package Manager, Bash Terminal, and MicroOS Dashboard.
- **Download ISO:** [Hugging Face Repository: `HARSHA-289/MICRO-OS`](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/debian-micro-os-x86_64.iso)

### B. Bare-Metal RustOS Kernel
- **Freestanding x86 Kernel:** 100% written in Rust using `![no_std]` and `![no_main]`.
- **Protected Mode Drivers:** GDT, 256-entry IDT (captures CPU faults), dual 8259 PIC remap, PIT timer scheduling, memory-mapped VGA (`0xb8000`), PS/2 keyboard controller, and interactive kernel shell.

---

## 3. Quick Start & Execution

To launch Debian MicroOS or Bare-Metal RustOS directly in VirtualBox:
```bash
# Navigate to Micro_OS_Submission
cd Micro_OS_Submission

# 1-Click Launch Debian MicroOS in VirtualBox (Full-screen, Wi-Fi ready)
.\Start-MicroOS.bat

# 1-Click Launch Bare-Metal RustOS in VirtualBox
.\Start-RustOS.bat
```

---

## 4. Grading Rubric Alignment Summary

| Assessment Requirement | Technical Implementation | Status |
|:---|:---|:---:|
| **Live GUI & Desktop Environment** | Automated autologin to XFCE desktop styled with Windows 11 ergonomics | **Complete (100%)** |
| **Authentic Wi-Fi Management** | Independent taskbar flyout with password verification, AP scanning & IP assignment | **Complete (100%)** |
| **Bare-Metal Operating System** | Freestanding 32-bit x86 Rust kernel with GDT, IDT, PIC, PIT & VGA text drivers | **Complete (100%)** |
| **Concurrency & Synchronization** | 24-configuration empirical benchmark matrix in Rust testing H1, H2, and H3 | **Complete (100%)** |
| **Academic Reports & Documentation** | Complete CA-2, CA-3, CA-5 research papers and Lab Assignment Record | **Complete (100%)** |
