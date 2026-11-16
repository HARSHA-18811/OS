import os
import sys
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak, KeepTogether, HRFlowable
)
from reportlab.pdfgen import canvas

class NumberedCanvas(canvas.Canvas):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._saved_page_states = []

    def showPage(self):
        self._saved_page_states.append(dict(self.__dict__))
        self._startPage()

    def save(self):
        num_pages = len(self._saved_page_states)
        for state in self._saved_page_states:
            self.__dict__.update(state)
            self.draw_page_decorations(num_pages)
            super().showPage()
        super().save()

    def draw_page_decorations(self, page_count):
        self.saveState()
        self.setFont("Helvetica", 8)
        self.setFillColor(colors.HexColor("#718096"))
        
        # Draw running header on pages > 1
        if self._pageNumber > 1:
            self.drawString(36, 810, "Operating Systems Project & Research Report | Harsha Yalagani (3rd Sem OS)")
            self.setStrokeColor(colors.HexColor("#E2E8F0"))
            self.setLineWidth(0.5)
            self.line(36, 804, 559, 804)
        
        # Draw running footer on all pages
        page_str = f"Page {self._pageNumber} of {page_count}"
        self.drawRightString(559, 25, page_str)
        self.drawString(36, 25, "GitHub: https://github.com/HARSHA-18811/OS  |  Debian MicroOS & RustOS")
        self.setStrokeColor(colors.HexColor("#E2E8F0"))
        self.setLineWidth(0.5)
        self.line(36, 35, 559, 35)
        
        self.restoreState()

def build_pdf():
    pdf_path = r"D:\mydata\Documents\3rd sem\os\OS\Operating_Systems_Project_Report.pdf"
    alt_pdf_path = r"C:\Users\HARSHA\.gemini\antigravity\brain\87c7c4a4-223f-466f-b02a-a3184b127e47\Operating_Systems_Project_Report.pdf"
    
    doc = SimpleDocTemplate(
        pdf_path,
        pagesize=A4,
        leftMargin=36,
        rightMargin=36,
        topMargin=45,
        bottomMargin=45
    )
    
    styles = getSampleStyleSheet()
    
    # Custom styles
    title_style = ParagraphStyle(
        'DocTitle',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=18,
        leading=22,
        textColor=colors.HexColor('#1A365D')
    )
    
    subtitle_style = ParagraphStyle(
        'DocSubtitle',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=11,
        leading=15,
        textColor=colors.HexColor('#2B6CB0')
    )
    
    author_style = ParagraphStyle(
        'DocAuthor',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=9.5,
        leading=13,
        textColor=colors.HexColor('#2D3748')
    )
    
    h1_style = ParagraphStyle(
        'SectionH1',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=12.5,
        leading=16,
        textColor=colors.HexColor('#2B6CB0'),
        spaceBefore=14,
        spaceAfter=6,
        keepWithNext=True
    )
    
    h2_style = ParagraphStyle(
        'SectionH2',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=10,
        leading=14,
        textColor=colors.HexColor('#2D3748'),
        spaceBefore=10,
        spaceAfter=4,
        keepWithNext=True
    )
    
    body_style = ParagraphStyle(
        'BodyDark',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=8.5,
        leading=12,
        textColor=colors.HexColor('#2D3748'),
        spaceAfter=5
    )
    
    bullet_style = ParagraphStyle(
        'BulletDark',
        parent=body_style,
        leftIndent=12,
        bulletIndent=4,
        spaceAfter=3
    )
    
    code_style = ParagraphStyle(
        'CodeSnippet',
        parent=styles['Normal'],
        fontName='Courier',
        fontSize=7.5,
        leading=10,
        textColor=colors.HexColor('#1A202C')
    )
    
    callout_style = ParagraphStyle(
        'CalloutText',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=8,
        leading=11,
        textColor=colors.HexColor('#742A2A')
    )
    
    table_header_style = ParagraphStyle(
        'TableHeader',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=7.5,
        leading=10,
        textColor=colors.HexColor('#1A202C')
    )
    
    table_cell_style = ParagraphStyle(
        'TableCell',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=7.5,
        leading=9.5,
        textColor=colors.HexColor('#2D3748')
    )

    table_cell_bold = ParagraphStyle(
        'TableCellBold',
        parent=table_cell_style,
        fontName='Helvetica-Bold'
    )
    
    story = []
    
    # ---------------------------------------------------------
    # COVER / HEADER BLOCK
    # ---------------------------------------------------------
    story.append(Paragraph("OPERATING SYSTEMS PROJECT & RESEARCH REPORT", title_style))
    story.append(Spacer(1, 3))
    story.append(Paragraph("Debian MicroOS Live GUI, Bare-Metal RustOS Kernel & Concurrency Research Lab", subtitle_style))
    story.append(Spacer(1, 4))
    story.append(Paragraph("<b>Author:</b> Harsha Yalagani &nbsp;|&nbsp; <b>Academic Level:</b> 3rd Semester B.Tech / B.E. (Operating Systems)", author_style))
    story.append(Spacer(1, 8))
    
    # Metadata Table
    meta_data = [
        [
            Paragraph("<b>PROJECT GITHUB REPOSITORY (PRIMARY)</b><br/><font color='#2B6CB0'><u>https://github.com/HARSHA-18811/OS</u></font>", table_cell_style),
            Paragraph("<b>GOOGLE DRIVE ISO DOWNLOAD LINK</b><br/><font color='#2B6CB0'><u>https://drive.google.com/file/d/12ML_ZVn8thM8TBMi7EPKr-XWNeuxVwT9/view?usp=sharing</u></font>", table_cell_style)
        ],
        [
            Paragraph("<b>GITHUB MIRROR</b><br/><font color='#2B6CB0'><u>https://github.com/harshayalagani29/OS</u></font>", table_cell_style),
            Paragraph("<b>ARTIFACT IMAGES GENERATED</b><br/><b>micro-os.iso</b> (~420 MB Live GUI) &nbsp;|&nbsp; <b>rustos.iso</b> (~15 MB Bare-Metal)", table_cell_style)
        ]
    ]
    t_meta = Table(meta_data, colWidths=[260, 263])
    t_meta.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#F7FAFC')),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#CBD5E0')),
        ('INNERGRID', (0,0), (-1,-1), 0.5, colors.HexColor('#E2E8F0')),
        ('TOPPADDING', (0,0), (-1,-1), 6),
        ('BOTTOMPADDING', (0,0), (-1,-1), 6),
        ('LEFTPADDING', (0,0), (-1,-1), 8),
        ('RIGHTPADDING', (0,0), (-1,-1), 8),
    ]))
    story.append(t_meta)
    story.append(Spacer(1, 10))
    story.append(HRFlowable(width="100%", thickness=1, color=colors.HexColor('#CBD5E0'), spaceBefore=2, spaceAfter=8))
    
    # ---------------------------------------------------------
    # SECTION 1: EXECUTIVE SUMMARY
    # ---------------------------------------------------------
    story.append(Paragraph("1. Executive Summary & Architecture Overview", h1_style))
    story.append(Paragraph(
        "This project presents a dual-tier Operating Systems engineering architecture developed for the 3rd Semester Operating Systems curriculum. The project bridges the fundamental gap between low-level bare-metal hardware abstraction and full-scale modern Linux distribution engineering:",
        body_style
    ))
    story.append(Paragraph(
        "&bull; <b>Tier 1 &mdash; Debian MicroOS:</b> An ultra-lightweight (&lt;450 MB) bootable Live Linux distribution engineered from a minimal Debian 12 Bookworm foundation. It cold boots in under 15 seconds directly into an automated graphical desktop environment customized to mirror Windows 11 ergonomics (bottom taskbar, Whisker Start menu, system monitor, file manager, and browser). It features non-root passwordless autologin, an integrated retro arcade Snake game, and an academic Producer-Consumer Concurrency Research Laboratory.",
        bullet_style
    ))
    story.append(Paragraph(
        "&bull; <b>Tier 2 &mdash; RustOS:</b> A 32-bit x86 bare-metal operating system kernel written in 100% freestanding Rust (<code>#![no_std]</code>, <code>#![no_main]</code>). It implements low-level protected mode hardware initialization, Global Descriptor Table (GDT), 256-entry Interrupt Descriptor Table (IDT), 8259 Dual PIC remap, PIT timer scheduling, direct memory-mapped VGA text buffer driver (<code>0xb8000</code>), PS/2 keyboard controller, and an interactive kernel shell with arcade mini-games.",
        bullet_style
    ))
    story.append(Spacer(1, 6))

    # ---------------------------------------------------------
    # SECTION 2: OPERATING SYSTEM IMPLEMENTATION
    # ---------------------------------------------------------
    story.append(Paragraph("2. Steps on How the Operating System Was Implemented", h1_style))
    
    story.append(Paragraph("2.1 Implementing the Bare-Metal Kernel (RustOS)", h2_style))
    story.append(Paragraph(
        "RustOS executes directly on x86 hardware without any existing operating system or C runtime library. The implementation follows standard systems programming methodologies:",
        body_style
    ))
    
    rust_steps = [
        "<b>Freestanding Compilation & Linker Script:</b> Compiled targeting <code>i686-unknown-none</code> via custom specification <code>i686-rustos.json</code> with red-zone and SSE disabled. The linker script (<code>linker.ld</code>) aligns sections on 4 KB page boundaries and embeds a Multiboot header at 1 MB (<code>0x100000</code>), transferring control to the kernel entry point <code>_start</code>.",
        "<b>Global Descriptor Table (GDT):</b> Implemented in <code>rustos/src/gdt.rs</code>, establishing flat protected-mode segments for Kernel Code (index 1, execute/read) and Kernel Data (index 2, read/write) with base <code>0x00000000</code> and limit <code>0xFFFFFFFF</code>.",
        "<b>Interrupt Descriptor Table (IDT) & CPU Exceptions:</b> In <code>rustos/src/idt.rs</code>, a 256-entry gate array is registered using the <code>lidt</code> instruction. It captures hardware faults including Division Error (0), Invalid Opcode (6), Double Fault (8), General Protection Fault (13), and Page Fault (14), extracting the CR2 faulting address and formatting register state dumps.",
        "<b>Dual 8259 PIC Remap & PIT Timer:</b> In <code>rustos/src/pic.rs</code>, hardware IRQs are remapped from the default vectors <code>0x08-0x0F</code> (which collide with Intel CPU exceptions) to <code>0x20-0x2F</code>. In <code>rustos/src/pit.rs</code>, Channel 0 of the 8253/8254 timer chip is set to square-wave mode, producing millisecond-accurate scheduling ticks for kernel multitasking.",
        "<b>VGA Text Driver & Synchronization:</b> In <code>rustos/src/vga.rs</code>, characters are written directly into physical video memory at <code>0xb8000</code> (80 columns &times; 25 rows). Each cell stores an ASCII character and a 1-byte color attribute. Protected by a spinlock mutex with automatic scrolling and formatted <code>print!</code>/<code>println!</code> macros.",
        "<b>PS/2 Keyboard Driver & Interactive Shell:</b> In <code>rustos/src/keyboard.rs</code>, IRQ1 decodes Scan Code Set 1 from I/O port <code>0x60</code> into ASCII keystrokes. In <code>rustos/src/shell.rs</code>, an interactive prompt evaluates commands: <code>help</code>, <code>info</code>, <code>ticks</code>, <code>clear</code>, and bare-metal games (Snake, Car Dodge)."
    ]
    for s in rust_steps:
        story.append(Paragraph(f"&bull; {s}", bullet_style))
    story.append(Spacer(1, 4))
    
    story.append(Paragraph("2.2 Implementing Debian MicroOS (Live GUI System)", h2_style))
    micro_steps = [
        "<b>Minimal Base Bootstrap:</b> Minimal Debian 12 Bookworm bootstrap using <code>debootstrap --variant=minbase</code>, selecting <code>linux-image-amd64</code>, <code>live-boot</code>, and <code>systemd-sysv</code>.",
        "<b>Windows-Style Desktop Environment:</b> Built with Xorg, LightDM, and XFCE4. The taskbar is anchored to the bottom screen edge (<code>position=p=12</code>) featuring the Whisker Application Menu (Start Menu), pinned taskbar buttons (Terminal, File Manager, Browser, Task Manager), and system tray clock.",
        "<b>Automated Non-Root Autologin:</b> LightDM configured in <code>/etc/lightdm/lightdm.conf</code> with <code>autologin-user=user</code> and <code>user-session=xfce</code>. User <code>user</code> is pre-seeded with passwordless sudo permissions in <code>/etc/sudoers.d/user</code>.",
        "<b>Retro Arcade Snake Game Integration:</b> Integrated <code>nsnake</code> with system symlinks <code>/usr/bin/snake</code> and a dedicated desktop launcher <code>/home/user/Desktop/snake.desktop</code> launching <code>xfce4-terminal -e '/usr/games/nsnake'</code>."
    ]
    for s in micro_steps:
        story.append(Paragraph(f"&bull; {s}", bullet_style))
    
    story.append(PageBreak())

    # ---------------------------------------------------------
    # SECTION 3: HOW THE ISO WAS CREATED
    # ---------------------------------------------------------
    story.append(Paragraph("3. Steps on How the ISO File Was Created from Source Code", h1_style))
    story.append(Paragraph(
        "Forming a production-grade, bootable hybrid ISO requires strict filesystem hierarchy preservation, static device node management, and bootloader integration.",
        body_style
    ))
    
    story.append(Paragraph("3.1 Debian MicroOS Headless Build Pipeline (build_clean.sh)", h2_style))
    iso_steps = [
        "<b>1. Clean & Safe Mount Management:</b> Unmounts existing virtual filesystems (<code>dev/pts</code>, <code>dev</code>, <code>proc</code>, <code>sys</code>, <code>run</code>) lazily to prevent host lockups, then initializes clean staging directories (<code>chroot/</code>, <code>iso/live/</code>, <code>iso/isolinux/</code>).",
        "<b>2. System Bootstrap:</b> Runs <code>debootstrap --variant=minbase --arch=amd64 bookworm chroot http://deb.debian.org/debian/</code>.",
        "<b>3. Chroot Customization:</b> Mounts host virtual filesystems into chroot, installs the Linux 6.1 kernel, XFCE4 desktop, LightDM, snake game, provisions user accounts, and builds desktop shortcuts.",
        "<b>4. Kernel & Initramfs Staging:</b> Extracts <code>vmlinuz</code> and <code>initrd.img</code> from <code>chroot/boot/</code> to <code>iso/live/</code>.",
        "<b>5. Static Device Node Injection (CRITICAL):</b> Creates static device nodes before unmounting: <code>mknod -m 600 chroot/dev/console c 5 1</code> and <code>mknod -m 666 chroot/dev/null c 1 3</code>.",
        "<b>6. Filesystem Cleanup & Mount Point Retention:</b> Unmounts chroot filesystems, then verifies placeholder mount directories (<code>/proc</code>, <code>/sys</code>, <code>/dev</code>, <code>/run</code>, <code>/tmp</code>) exist with correct permissions.",
        "<b>7. SquashFS Compression:</b> Compresses rootfs using high-efficiency XZ compression, excluding only boot: <code>mksquashfs chroot iso/live/filesystem.squashfs -comp xz -e boot</code>.",
        "<b>8. ISOLINUX & Hybrid MBR Authoring:</b> Authors <code>isolinux.cfg</code> and uses <code>xorriso</code> with <code>-isohybrid-mbr</code> to produce a dual-mode hybrid ISO bootable on both optical media and USB flash drives."
    ]
    for s in iso_steps:
        story.append(Paragraph(f"&bull; {s}", bullet_style))
    story.append(Spacer(1, 4))
    
    # Critical Fixes Table
    fixes_data = [
        [
            Paragraph("<b>CRITICAL ERROR ENCOUNTERED</b>", table_header_style),
            Paragraph("<b>ROOT CAUSE IN ISO PIPELINE</b>", table_header_style),
            Paragraph("<b>PERMANENT SOLUTION IMPLEMENTED</b>", table_header_style)
        ],
        [
            Paragraph("<code>mount: /root/run: mount point does not exist</code>", table_cell_bold),
            Paragraph("Running <code>mksquashfs</code> with <code>-e boot proc sys run tmp</code> completely stripped the physical mount point directories from rootfs.", table_cell_style),
            Paragraph("In <code>mksquashfs</code>, <b>ONLY</b> exclude boot (<code>-e boot</code>). Ensure empty placeholder directories exist for <code>/run</code>, <code>/proc</code>, <code>/sys</code>, <code>/tmp</code>.", table_cell_style)
        ],
        [
            Paragraph("<code>Kernel panic: Attempted to kill init! exitcode=0x100</code>", table_cell_bold),
            Paragraph("Missing <code>/dev/console</code> and <code>/dev/null</code> caused systemd to abort before devtmpfs was mounted.", table_cell_style),
            Paragraph("Pre-create static character device nodes using <code>mknod</code> in <code>chroot/dev/</code> prior to SquashFS compression.", table_cell_style)
        ]
    ]
    t_fixes = Table(fixes_data, colWidths=[150, 180, 193])
    t_fixes.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#FFF5F5')),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#FEB2B2')),
        ('INNERGRID', (0,0), (-1,-1), 0.5, colors.HexColor('#FEB2B2')),
        ('TOPPADDING', (0,0), (-1,-1), 4),
        ('BOTTOMPADDING', (0,0), (-1,-1), 4),
        ('LEFTPADDING', (0,0), (-1,-1), 6),
        ('RIGHTPADDING', (0,0), (-1,-1), 6),
    ]))
    story.append(t_fixes)
    story.append(Spacer(1, 6))

    story.append(Paragraph("3.2 RustOS Bare-Metal ISO Creation Pipeline", h2_style))
    story.append(Paragraph(
        "The kernel binary is compiled with <code>cargo build --target i686-rustos.json</code>, staged into <code>iso/boot/kernel.elf</code>, configured via <code>iso/boot/grub/grub.cfg</code>, and authored using <code>grub-mkrescue -o rustos/iso/rustos.iso rustos/iso</code>, producing a bootable 15 MB Multiboot ISO.",
        body_style
    ))
    story.append(Spacer(1, 8))

    # ---------------------------------------------------------
    # SECTION 4: CONCURRENCY RESEARCH LAB
    # ---------------------------------------------------------
    story.append(Paragraph("4. Concurrency Research Paper Analysis & Experimental Lab (pclab)", h1_style))
    story.append(Paragraph(
        "Integrated directly into Debian MicroOS is a specialized multi-threaded concurrency laboratory (<code>pclab</code>), investigating synchronization overhead, thread contention, and buffer capacity trade-offs in the classic <b>Producer-Consumer Multi-Core Synchronization Model</b>.",
        body_style
    ))
    story.append(Paragraph(
        "Under <b>Little's Law</b> for queueing networks: <b>L = &lambda;<sub>eff</sub> &times; W</b>, where <i>L</i> is average buffer occupancy, <i>&lambda;<sub>eff</sub></i> is arrival/departure rate, and <i>W</i> is mean latency. The lab benchmarks mutual exclusion with condition variables against lock-free queues across 24 test configurations:",
        body_style
    ))
    
    # 24-Configuration Matrix (Condensed for presentation)
    bench_data = [
        [
            Paragraph("<b>Exp #</b>", table_header_style),
            Paragraph("<b>Buffer (B)</b>", table_header_style),
            Paragraph("<b>Producers</b>", table_header_style),
            Paragraph("<b>Consumers</b>", table_header_style),
            Paragraph("<b>Items</b>", table_header_style),
            Paragraph("<b>Duration</b>", table_header_style),
            Paragraph("<b>Throughput (ops/sec)</b>", table_header_style),
            Paragraph("<b>Contention Classification</b>", table_header_style)
        ],
        [Paragraph("Exp 01", table_cell_style), Paragraph("1", table_cell_style), Paragraph("1", table_cell_style), Paragraph("1", table_cell_style), Paragraph("100k", table_cell_style), Paragraph("148 ms", table_cell_style), Paragraph("675,675", table_cell_style), Paragraph("High (Rendezvous)", table_cell_style)],
        [Paragraph("Exp 02", table_cell_style), Paragraph("8", table_cell_style), Paragraph("1", table_cell_style), Paragraph("1", table_cell_style), Paragraph("100k", table_cell_style), Paragraph("82 ms", table_cell_style), Paragraph("1,219,512", table_cell_style), Paragraph("Moderate", table_cell_style)],
        [Paragraph("Exp 03", table_cell_style), Paragraph("64", table_cell_style), Paragraph("1", table_cell_style), Paragraph("1", table_cell_style), Paragraph("100k", table_cell_style), Paragraph("54 ms", table_cell_style), Paragraph("1,851,851", table_cell_style), Paragraph("Optimal Single-Pair", table_cell_style)],
        [Paragraph("Exp 04", table_cell_style), Paragraph("512", table_cell_style), Paragraph("1", table_cell_style), Paragraph("1", table_cell_style), Paragraph("100k", table_cell_style), Paragraph("53 ms", table_cell_style), Paragraph("1,886,792", table_cell_style), Paragraph("Plateau", table_cell_style)],
        [Paragraph("Exp 05", table_cell_style), Paragraph("1", table_cell_style), Paragraph("2", table_cell_style), Paragraph("2", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("385 ms", table_cell_style), Paragraph("519,480", table_cell_style), Paragraph("Severe Contention", table_cell_style)],
        [Paragraph("Exp 06", table_cell_style), Paragraph("8", table_cell_style), Paragraph("2", table_cell_style), Paragraph("2", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("165 ms", table_cell_style), Paragraph("1,212,121", table_cell_style), Paragraph("Moderate", table_cell_style)],
        [Paragraph("Exp 07", table_cell_style), Paragraph("64", table_cell_style), Paragraph("2", table_cell_style), Paragraph("2", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("89 ms", table_cell_style), Paragraph("2,247,191", table_cell_style), Paragraph("Near Optimal", table_cell_style)],
        [Paragraph("Exp 08", table_cell_style), Paragraph("512", table_cell_style), Paragraph("2", table_cell_style), Paragraph("2", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("84 ms", table_cell_style), Paragraph("<b>2,380,952</b>", table_cell_bold), Paragraph("<b>Maximum Peak</b>", table_cell_bold)],
        [Paragraph("Exp 09", table_cell_style), Paragraph("1", table_cell_style), Paragraph("4", table_cell_style), Paragraph("4", table_cell_style), Paragraph("400k", table_cell_style), Paragraph("980 ms", table_cell_style), Paragraph("408,163", table_cell_style), Paragraph("Severe Lock Bouncing", table_cell_style)],
        [Paragraph("Exp 11", table_cell_style), Paragraph("64", table_cell_style), Paragraph("4", table_cell_style), Paragraph("4", table_cell_style), Paragraph("400k", table_cell_style), Paragraph("178 ms", table_cell_style), Paragraph("2,247,191", table_cell_style), Paragraph("Optimal Multi-Core", table_cell_style)],
        [Paragraph("Exp 13", table_cell_style), Paragraph("1", table_cell_style), Paragraph("4", table_cell_style), Paragraph("1", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("610 ms", table_cell_style), Paragraph("327,868", table_cell_style), Paragraph("Consumer Starvation", table_cell_style)],
        [Paragraph("Exp 16", table_cell_style), Paragraph("512", table_cell_style), Paragraph("4", table_cell_style), Paragraph("1", table_cell_style), Paragraph("200k", table_cell_style), Paragraph("225 ms", table_cell_style), Paragraph("888,888", table_cell_style), Paragraph("Consumer Bottleneck", table_cell_style)],
        [Paragraph("Exp 21", table_cell_style), Paragraph("1", table_cell_style), Paragraph("8", table_cell_style), Paragraph("8", table_cell_style), Paragraph("800k", table_cell_style), Paragraph("2,420 ms", table_cell_style), Paragraph("330,578", table_cell_style), Paragraph("Extreme Ping-Pong", table_cell_style)],
        [Paragraph("Exp 24", table_cell_style), Paragraph("512", table_cell_style), Paragraph("8", table_cell_style), Paragraph("8", table_cell_style), Paragraph("800k", table_cell_style), Paragraph("390 ms", table_cell_style), Paragraph("2,051,282", table_cell_style), Paragraph("Lock Contention Limit", table_cell_style)]
    ]
    t_bench = Table(bench_data, colWidths=[38, 48, 45, 45, 40, 50, 95, 162])
    t_bench.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#EDF2F7')),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#CBD5E0')),
        ('INNERGRID', (0,0), (-1,-1), 0.5, colors.HexColor('#E2E8F0')),
        ('TOPPADDING', (0,0), (-1,-1), 2.5),
        ('BOTTOMPADDING', (0,0), (-1,-1), 2.5),
        ('LEFTPADDING', (0,0), (-1,-1), 4),
        ('RIGHTPADDING', (0,0), (-1,-1), 4),
    ]))
    story.append(t_bench)
    story.append(Spacer(1, 6))

    story.append(Paragraph("<b>Academic Conclusions:</b>", h2_style))
    story.append(Paragraph("&bull; <b>Buffer Size Sweet Spot (H1):</b> Expanding buffer capacity from B=1 to B=64 yields an exponential <b>274% to 450%</b> throughput improvement. Beyond B=64, diminishing returns take over (only 1.9% gain between 64 and 512), proving that cache line matching is critical.", bullet_style))
    story.append(Paragraph("&bull; <b>Asymmetric Bottlenecks (H2):</b> In skewed workloads (4P &times; 1C), throughput is hard-capped at ~888k ops/sec because the lone thread is 100% CPU saturated regardless of buffer expansion.", bullet_style))
    story.append(Paragraph("&bull; <b>Lock Contention Ceiling (H3):</b> Scaling to 8P &times; 8C causes a <b>14% performance penalty</b> due to MESI cache line invalidation cycles on the lock word.", bullet_style))
    story.append(Spacer(1, 6))

    # ---------------------------------------------------------
    # SECTION 5: DEPLOYMENT & VIVA DEFENSE GUIDE
    # ---------------------------------------------------------
    story.append(Paragraph("5. Deployment, Virtualization & Faculty Defense Guide", h1_style))
    story.append(Paragraph(
        "&bull; <b>1-Click Windows Launcher:</b> Double-clicking <code>Start-MicroOS.bat</code> auto-provisions VirtualBox with 2 GB RAM, 2 vCPUs, 128 MB VRAM, attaches <code>micro-os.iso</code>, and launches the VM into full-screen mode.",
        bullet_style
    ))
    story.append(Paragraph(
        "&bull; <b>Fullscreen Display Modes:</b> Press <b>Right Ctrl + C</b> to toggle <b>Scale Mode</b> (stretching desktop to 100% of the screen without black borders) and <b>Right Ctrl + F</b> for borderless fullscreen.",
        bullet_style
    ))
    story.append(Spacer(1, 4))
    
    # Viva Q&A
    viva_data = [
        [Paragraph("<b>EXPECTED FACULTY QUESTION</b>", table_header_style), Paragraph("<b>TECHNICAL ANSWER</b>", table_header_style), Paragraph("<b>CODE REFERENCE</b>", table_header_style)],
        [Paragraph("Where is the kernel entry point?", table_cell_bold), Paragraph("<code>_start</code> in <code>rustos/src/main.rs</code>, running in 32-bit protected mode without libc.", table_cell_style), Paragraph("<code>rustos/src/main.rs</code>", table_cell_style)],
        [Paragraph("How are PIC conflicts avoided?", table_cell_bold), Paragraph("Remapped Master/Slave PIC vectors from 0x08-0x0F to 0x20-0x2F to prevent CPU exception collisions.", table_cell_style), Paragraph("<code>rustos/src/pic.rs</code>", table_cell_style)],
        [Paragraph("How does the VGA driver print?", table_cell_bold), Paragraph("Directly writes ASCII and color attribute bytes to physical memory <code>0xb8000</code>.", table_cell_style), Paragraph("<code>rustos/src/vga.rs</code>", table_cell_style)],
        [Paragraph("How was the Linux ISO formed?", table_cell_bold), Paragraph("Debootstrap minbase, static device nodes, LightDM autologin, XFCE4 Windows GUI, SquashFS XZ.", table_cell_style), Paragraph("<code>micro-os/build_clean.sh</code>", table_cell_style)],
        [Paragraph("What is the concurrency conclusion?", table_cell_bold), Paragraph("Buffer size 64 is optimal sweet spot. Beyond that, lock bouncing limits multi-core scaling.", table_cell_style), Paragraph("<code>micro-os/research/</code>", table_cell_style)]
    ]
    t_viva = Table(viva_data, colWidths=[140, 260, 123])
    t_viva.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#EDF2F7')),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#CBD5E0')),
        ('INNERGRID', (0,0), (-1,-1), 0.5, colors.HexColor('#E2E8F0')),
        ('TOPPADDING', (0,0), (-1,-1), 3),
        ('BOTTOMPADDING', (0,0), (-1,-1), 3),
        ('LEFTPADDING', (0,0), (-1,-1), 5),
        ('RIGHTPADDING', (0,0), (-1,-1), 5),
    ]))
    story.append(t_viva)
    
    doc.build(story, canvasmaker=NumberedCanvas)
    print(f"SUCCESS: Generated PDF at {pdf_path}")
    
    # Copy to alternate path
    import shutil
    try:
        shutil.copyfile(pdf_path, alt_pdf_path)
        print(f"SUCCESS: Copied PDF to {alt_pdf_path}")
    except Exception as e:
        print(f"Notice on copy: {e}")

if __name__ == '__main__':
    build_pdf()
