# RustOS Faculty Submission: Screenshot Checklist

This document details the recommended screenshots to capture from the **Oracle VirtualBox** window for the academic project submission.

---

## Required Screenshots

| # | Screen Name | How to Reach in VirtualBox | What to Highlight |
|:---:|:---|:---|:---|
| **1** | **Boot Sequence & Drivers** | Start VM with `.\run.ps1` | The green `[ OK ]` initialization list showing VGA, Serial, GDT, PIC, PIT 100Hz, IDT, Keyboard, 2MB Heap, RTC, and VFS. |
| **2** | **Desktop Application Menu** | Boot screen (or run `games` from shell) | The visual menu box with options 1–7, highlight bar on selected item, and navigation hints. |
| **3** | **Status Bar & CMOS Clock** | Bottom row (row 24) on any screen | Active application name on left (`[RustOS] App: Shell`), hints in middle, real-time CMOS clock on right. |
| **4** | **Shell Environment & Commands** | Select option `1` from menu | Prompt `root@rustos:/#`, output of `help`, `uname`, `whoami`, and `uptime`. |
| **5** | **Calendar & Integer Calculator** | In shell: `cal` and `bc 25 * 4`, `bc 10 / 0` | Today's date highlighted in blue/yellow; arithmetic calculation and divide-by-zero error handling. |
| **6** | **In-Memory File System & Vi** | In shell: `ls`, `echo Hello > test.txt`, `cat test.txt`, `wc readme.txt` | File listing with `rw-`/`r--` permissions, file creation, word counts, and file editing via `vi`. |
| **7** | **Snake Arcade Game** | Select option `2` from Desktop Menu | Snake moving in arena, score and high-score displayed on top banner, food item (`*`). |
| **8** | **Snake and Ladders Game** | Select option `3` from Desktop Menu | 10x10 serpentine board (1..100), player indicators `[H]` and `[C]`, ladders (`^L^`), snakes (`vSv`), and dice log sidebar. |
| **9** | **Car Dodge Survival** | Select option `4` from Desktop Menu | 3-lane road with animated dividers, player car `|####|` at bottom, falling roadblocks `[XXXX]`, and speed indicator. |
| **10**| **Producer-Consumer Lab Menu** | Select option `5` from Desktop Menu (or `research` from shell) | Main lab menu showing options 1–6 and the **mandatory limitation notice** in yellow/gray box. |
| **11**| **PC Lab 24-Config Results** | In Lab: press `1` (Run All) | Progress bar filling up, followed by the experimental results table showing Capacity, Thread Model, Throughput, and Wait counts. |
| **12**| **PC Lab ASCII Throughput Chart**| In Lab: press `3` (View Chart) | Bar chart (`#` blocks) demonstrating throughput growth from Cap 8 to Cap 128 and saturation at Cap 512. |
| **13**| **PC Lab Hypothesis Evaluation** | In Lab: press `5` (Check Hypotheses) | Formal evaluation of $H_1$, $H_2$, and $H_3$ marked as `[CONFIRMED]` with actual measured numbers. |
| **14**| **VirtualBox Sandbox Settings** | In VirtualBox Manager GUI | VM settings showing 128 MB RAM, SATA Controller attached to `rustos.iso`, and COM1 UART serial port enabled. |
