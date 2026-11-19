# Submissions for OS CA2, CA3, CA5 & CA6

**Course:** 25PCCS201 – Operating Systems Concepts (3rd Semester B.Tech)  
**Author:** Yalagani Harsha Vardhan  
**USN:** 25BTRCL289  
**Section:** CSE AIML-E  
**Department:** Computer Science & Engineering (AIML), School of Engineering & Technology  

---

## 1. Portfolio Navigation & Instant Online Document Viewers

> [!NOTE]  
> **Direct Browser Viewing (No Download Required):**  
> GitHub automatically renders all **PDF** documents directly in your web browser. Click any document below to view the full report including all diagrams, graphs, charts, and tables immediately on GitHub.

| Assessment Component | Document Link (Click to Read Online on GitHub) | Embedded Content | Description & Scope |
|:---|:---:|:---:|:---|
| 📁 **CA-6 MicroOS Submission** | [📁 **`Micro_OS_Submission/`**](./Micro_OS_Submission) | Source, Scripts & Docs | Minimal bootable Debian MicroOS Live GUI (Windows 11 UI, authentic Wi-Fi management with password authentication, Snake arcade) & Bare-Metal RustOS Kernel. |
| **CA-2 Investigation** | [📖 **`CA2.pdf`**](./CA2.pdf) | Complete PDF (5 Graphs & Charts) | *How Buffer Size, Thread Ratios, and the Linear Scaling Fallacy Influence Producer–Consumer Performance* (Rust prototype, 24 configurations). |
| **CA-3 Debugging Challenge** | [📖 **`CA3.pdf`**](./CA3.pdf) | Complete PDF (4 Concurrency Charts) | *OS Design & Debugging Challenge: Bounded Producer–Consumer Synchronization, Concurrency Defect Diagnosis & Buffer Scaling*. |
| **CA-5 Systems Engineering** | [📖 **`CA5.pdf`**](./CA5.pdf) | Complete PDF (11 Systems Engineering Graphs) | *Investigating the Impact of Bounded Buffer Sizing and Thread Allocation Ratios on Concurrent Producer–Consumer Throughput: A Linux Systems Engineering Empirical Study*. |
| **Research Paper Proposal** | [📖 **`25BTRCL289_Harsha_Vardhan_Research.pdf`**](./25BTRCL289_Harsha_Vardhan_Research.pdf) | Complete PDF (4 Figures) | Problem Identification, Literature Discovery, and Formal Research Formulation. |
| **Lab Record Portfolio** | [📖 **`Operating_Systems_Lab_Assignment_Harsha.pdf`**](./Operating_Systems_Lab_Assignment_Harsha.pdf) | Complete PDF (145 Screenshots & Code) | Comprehensive Laboratory Record: Linux system programming, concurrency models, and architecture simulations. |
| **OS Project Report** | [📖 **`Operating_Systems_Project_Report.pdf`**](./Operating_Systems_Project_Report.pdf) | Complete PDF Report | Academic Project Report covering Debian MicroOS Live GUI, Bare-Metal RustOS Kernel, and Concurrency Lab. |

*(Note: Original editable Microsoft Word `.docx` source files are preserved inside [`Micro_OS_Submission/docs/word_documents/`](./Micro_OS_Submission/docs/word_documents).)*

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
