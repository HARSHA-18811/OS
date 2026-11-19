# Submissions for OS CA2, CA3, CA5 & CA6

**Course:** 25PCCS201 – Operating Systems Concepts (3rd Semester B.Tech)  
**Author:** Yalagani Harsha Vardhan  
**USN:** 25BTRCL289  
**Section:** CSE AIML-E  
**Department:** Computer Science & Engineering (AIML), School of Engineering & Technology  

---

## 1. Portfolio Navigation & Online Document Viewers

> [!TIP]
> **Read Online Directly in Your Browser (No Download Required):**  
> GitHub automatically renders **PDF** and **Markdown** documents directly in the web browser. Click any **[View Online (PDF)]** or **[View Online (Markdown)]** link below to read the complete submission online immediately.

| Component | Read Online (PDF) | Read Online (Markdown) | Word Online (.docx) | Download (.docx) | Description |
|:---|:---:|:---:|:---:|:---:|:---|
| **📁 CA-6 MicroOS Submission** | — | [**Open Folder**](./Micro_OS_Submission) | — | — | Debian MicroOS Live GUI (Windows 11 theme, authentic Wi-Fi flyout with password validation, arcade) + Bare-Metal RustOS Kernel. |
| **CA-2 Investigation** | [📖 **View Online (PDF)**](./CA2.pdf) | [📝 **View Online (MD)**](./CA2.md) | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/CA2.docx) | [💾 `CA2.docx`](./CA2.docx) | *How Buffer Size, Thread Ratios, and the Linear Scaling Fallacy Influence Producer–Consumer Performance* (Rust prototype, 24 configurations). |
| **CA-3 Debugging Challenge** | [📖 **View Online (PDF)**](./CA3.pdf) | [📝 **View Online (MD)**](./CA3.md) | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/CA3.docx) | [💾 `CA3.docx`](./CA3.docx) | *OS Design & Debugging Challenge: Bounded Producer–Consumer Synchronization, Concurrency Defect Diagnosis & Buffer Scaling*. |
| **CA-5 Systems Engineering** | [📖 **View Online (PDF)**](./CA5.pdf) | [📝 **View Online (MD)**](./CA5.md) | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/CA5.docx) | [💾 `CA5.docx`](./CA5.docx) | *Investigating the Impact of Bounded Buffer Sizing and Thread Allocation Ratios on Concurrent Producer–Consumer Throughput: A Linux Systems Engineering Empirical Study*. |
| **Research Paper Proposal** | [📖 **View Online (PDF)**](./25BTRCL289_Harsha_Vardhan_Research.pdf) | — | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/25BTRCL289_Harsha_Vardhan_Research.docx) | [💾 `Research.docx`](./25BTRCL289_Harsha_Vardhan_Research.docx) | Problem Identification, Literature Discovery, and Formal Research Formulation. |
| **Lab Record Portfolio** | [📖 **View Online (PDF)**](./Operating_Systems_Lab_Assignment_Harsha.pdf) | — | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/Operating_Systems_Lab_Assignment_Harsha.docx) | [💾 `Lab_Record.docx`](./Operating_Systems_Lab_Assignment_Harsha.docx) | Complete Laboratory Record: Linux system programming, concurrency models, and architecture simulations. |
| **OS Project Report** | [📖 **View Online (PDF)**](./Operating_Systems_Project_Report.pdf) | [📝 `REPORT.md`](./Micro_OS_Submission/REPORT.md) | [🌐 **Word Online**](https://view.officeapps.live.com/op/view.aspx?src=https://raw.githubusercontent.com/HARSHA-18811/OS/main/Operating_Systems_Project_Report.docx) | [💾 `Report.docx`](./Operating_Systems_Project_Report.docx) | Academic Project Report covering Debian MicroOS Live GUI, Bare-Metal RustOS Kernel, and Concurrency Lab. |

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
