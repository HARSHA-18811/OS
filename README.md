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
> GitHub automatically renders all **PDF** documents directly in your web browser. Click any **[View Online (PDF)]** link below to read the complete report including all diagrams, graphs, charts, and tables immediately on GitHub.

| Assessment Component | Read Online (PDF) | Word File (.docx) | Embedded Content | Description & Scope |
|:---|:---:|:---:|:---:|:---|
| 📁 **CA-6 MicroOS Submission** | — | — | Source, Scripts & Docs | [📁 **`Micro_OS_Submission/`**](./Micro_OS_Submission): Minimal bootable Debian MicroOS Live GUI & Bare-Metal RustOS Kernel. |
| **CA-2 Investigation** | [📖 **`CA2.pdf`**](./CA2.pdf) | [💾 `CA2.docx`](./Micro_OS_Submission/docs/word_documents/CA2.docx) | 5 Graphs & Charts | *How Buffer Size, Thread Ratios, and the Linear Scaling Fallacy Influence Producer–Consumer Performance* (Rust prototype, 24 configurations). |
| **CA-3 Debugging Challenge** | [📖 **`CA3.pdf`**](./CA3.pdf) | [💾 `CA3.docx`](./Micro_OS_Submission/docs/word_documents/CA3.docx) | 4 Concurrency Charts | *OS Design & Debugging Challenge: Bounded Producer–Consumer Synchronization, Concurrency Defect Diagnosis & Buffer Scaling*. |
| **CA-5 Systems Engineering** | [📖 **`CA5.pdf`**](./CA5.pdf) | [💾 `CA5.docx`](./Micro_OS_Submission/docs/word_documents/CA5.docx) | 11 Systems Graphs | *Investigating the Impact of Bounded Buffer Sizing and Thread Allocation Ratios on Concurrent Producer–Consumer Throughput: A Linux Systems Engineering Empirical Study*. |
| **Research Paper Proposal** | [📖 **`Research.pdf`**](./25BTRCL289_Harsha_Vardhan_Research.pdf) | [💾 `Research.docx`](./Micro_OS_Submission/docs/word_documents/25BTRCL289_Harsha_Vardhan_Research.docx) | 4 Figures | Problem Identification, Literature Discovery, and Formal Research Formulation. |
| **Lab Record Portfolio** | [📖 **`Lab_Assignment.pdf`**](./Operating_Systems_Lab_Assignment_Harsha.pdf) | [💾 `Lab_Assignment.docx`](./Micro_OS_Submission/docs/word_documents/Operating_Systems_Lab_Assignment_Harsha.docx) | 145 Screenshots & Code | Comprehensive Laboratory Record: 20 chapters of Linux system programming, concurrency models, and architecture simulations. |
| **OS Project Report** | [📖 **`Project_Report.pdf`**](./Operating_Systems_Project_Report.pdf) | [💾 `Report.docx`](./Micro_OS_Submission/docs/word_documents/Operating_Systems_Project_Report.docx) | Complete Report | Academic Project Report covering Debian MicroOS Live GUI, Bare-Metal RustOS Kernel, and Concurrency Lab. |

*(Note: Original editable Microsoft Word `.docx` source files are preserved inside [`Micro_OS_Submission/docs/word_documents/`](./Micro_OS_Submission/docs/word_documents).)*

---

## 2. MicroOS & RustOS Architecture Overview (CA-6)

### A. Debian MicroOS Live GUI
- **Lightweight Distribution:** Engineered from a clean Debian 12 Bookworm base (~557 MB Live Hybrid ISO).
- **Physical Hardware & USB Pen Drive Ready:** Built-in non-free wireless firmware (`firmware-iwlwifi`, `firmware-realtek`, `firmware-atheros`) in initramfs for bare-metal live USB booting on physical laptops (Intel AX201, AX200, RTL8821, etc.).
- **Windows 11 Ergonomics:** Bottom taskbar, Whisker Start menu, system tray with Wi-Fi flyout, and acrylic dark theme.
- **Independent Network & Authentic Wi-Fi Security:**
  - **Offline by Default:** Boots strictly disconnected/isolated until user explicitly authenticates with Wi-Fi.
  - **Nearby Access Point Scanning:** Lists real available access points (`MOJO-JAN-C1-F8`, `MOJO-JAN-C1-F9`, `Harsha`, `OPPO F29 Pro 5G 232d`, `BHYRAVA.PG.3RD.FLOOR`, `Jain_Student`, `YAGNESH`, etc.).
  - **Strict Password Authentication:** Live security key validation. Rejecting incorrect passwords with inline red warning text (*"The network security key isn't correct. Please try again."*) and keeping the system offline until the valid key is entered.
- **Built-in Desktop Apps:**
  - **Default Web Browser (Firefox ESR):** Pre-configured modern web browser with desktop launcher and default MIME associations (shows offline status until Wi-Fi is connected).
  - **Producer–Consumer Performance Research Lab (`pclab`):** Academic concurrency research testbed with 24-configuration benchmark matrix.
  - **Retro Arcade Snake Game:** Graphical desktop game running natively.
  - **Terminal (Bash) & APT Package Manager:** Full command-line terminal and Debian package manager.
- **Download ISO:**
  - [**Hugging Face Direct Download (`micro-os.iso`)**](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/micro-os.iso)
  - [**Hugging Face Direct Download (`debian-micro-os-x86_64.iso`)**](https://huggingface.co/datasets/HARSHA-289/MICRO-OS/resolve/main/debian-micro-os-x86_64.iso)
  - [**Hugging Face Repository: `HARSHA-289/MICRO-OS`**](https://huggingface.co/datasets/HARSHA-289/MICRO-OS)

### B. Bare-Metal RustOS Kernel
- **Freestanding x86 Kernel:** 100% written in Rust using `#![no_std]` and `#![no_main]`.
- **Protected Mode Drivers:** GDT, 256-entry IDT (captures CPU faults), dual 8259 PIC remap, PIT timer scheduling (100 Hz), memory-mapped VGA text buffer (`0xb8000`), PS/2 keyboard controller, and interactive kernel shell.

---

## 3. Quick Start & Execution

### Option 1: VirtualBox VM (1-Click Window Launch)
Double-click the **`Start MicroOS`** shortcut on your Windows desktop, or run from PowerShell:
```powershell
# Navigate to Micro_OS_Submission
cd Micro_OS_Submission

# 1-Click Launch Debian MicroOS in VirtualBox (Interactive GUI Window)
.\Start-MicroOS.bat

# 1-Click Launch Bare-Metal RustOS in VirtualBox
.\Start-RustOS.bat
```
*(Or launch Oracle VirtualBox Manager, select **MicroOS**, and click **Start**).*

### Option 2: Live USB Booting on Physical Laptop
1. Download **`micro-os.iso`** from the Hugging Face link above.
2. Flash the ISO to a USB pen drive using **Rufus** (select MBR/BIOS or GPT/UEFI) or **Ventoy**.
3. Plug the pen drive into your laptop, reboot, and press your laptop's boot key (F12, F9, or Esc).
4. Select **Debian Micro OS** from the boot menu — it boots directly to the graphical desktop with physical Wi-Fi hardware detected and ready.

---

## 4. Grading Rubric Alignment Summary

| Assessment Requirement | Technical Implementation | Status |
|:---|:---|:---:|
| **Live GUI & Desktop Environment** | Automated autologin to XFCE desktop styled with Windows 11 ergonomics | **Complete (100%)** |
| **Authentic Wi-Fi Management** | Independent taskbar flyout with strict password authentication, AP discovery & IP assignment | **Complete (100%)** |
| **Physical USB Live Booting** | Full non-free wireless firmware (`firmware-iwlwifi`, `firmware-realtek`, `firmware-atheros`) | **Complete (100%)** |
| **Bare-Metal Operating System** | Freestanding 32-bit x86 Rust kernel with GDT, IDT, PIC, PIT & VGA text drivers | **Complete (100%)** |
| **Concurrency & Synchronization** | 24-configuration empirical benchmark matrix in Rust testing H1, H2, and H3 | **Complete (100%)** |
| **Academic Reports & Documentation** | Complete CA-2, CA-3, CA-5 research papers and Lab Assignment Record | **Complete (100%)** |
