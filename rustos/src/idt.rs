//! Interrupt Descriptor Table (IDT) and CPU Exception Handlers

use core::arch::asm;
use core::arch::global_asm;
use core::ptr::addr_of_mut;
use crate::pic;
use crate::pit;
use crate::keyboard;
use crate::vga::{Color, VGA};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            zero: 0,
            type_attr: 0,
            offset_high: 0,
        }
    }

    pub fn set_handler(&mut self, handler: usize) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.selector = 0x08; // Kernel code segment
        self.zero = 0;
        self.type_attr = 0x8E; // 32-bit Interrupt Gate, Ring 0, Present
        self.offset_high = ((handler >> 16) & 0xFFFF) as u16;
    }
}

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u32,
}

#[repr(C)]
pub struct TrapFrame {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp_dummy: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub int_no: u32,
    pub err_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];

extern "C" {
    static isr_stub_table: [usize; 48];
}

global_asm!(
    r#"
    .macro ISR_NOERR i
    .globl isr_stub_\i
    isr_stub_\i:
        push 0
        push \i
        jmp isr_common
    .endm

    .macro ISR_ERR i
    .globl isr_stub_\i
    isr_stub_\i:
        push \i
        jmp isr_common
    .endm

    ISR_NOERR 0
    ISR_NOERR 1
    ISR_NOERR 2
    ISR_NOERR 3
    ISR_NOERR 4
    ISR_NOERR 5
    ISR_NOERR 6
    ISR_NOERR 7
    ISR_ERR   8
    ISR_NOERR 9
    ISR_ERR   10
    ISR_ERR   11
    ISR_ERR   12
    ISR_ERR   13
    ISR_ERR   14
    ISR_NOERR 15
    ISR_NOERR 16
    ISR_ERR   17
    ISR_NOERR 18
    ISR_NOERR 19
    ISR_NOERR 20
    ISR_NOERR 21
    ISR_NOERR 22
    ISR_NOERR 23
    ISR_NOERR 24
    ISR_NOERR 25
    ISR_NOERR 26
    ISR_NOERR 27
    ISR_NOERR 28
    ISR_NOERR 29
    ISR_ERR   30
    ISR_NOERR 31

    // Hardware IRQs (32..47)
    ISR_NOERR 32
    ISR_NOERR 33
    ISR_NOERR 34
    ISR_NOERR 35
    ISR_NOERR 36
    ISR_NOERR 37
    ISR_NOERR 38
    ISR_NOERR 39
    ISR_NOERR 40
    ISR_NOERR 41
    ISR_NOERR 42
    ISR_NOERR 43
    ISR_NOERR 44
    ISR_NOERR 45
    ISR_NOERR 46
    ISR_NOERR 47

    isr_common:
        pusha

        mov ax, 0x10
        mov ds, ax
        mov es, ax

        push esp
        call interrupt_dispatch
        add esp, 4

        popa
        add esp, 8
        iretd

    .section .rodata
    .globl isr_stub_table
    isr_stub_table:
        .long isr_stub_0, isr_stub_1, isr_stub_2, isr_stub_3, isr_stub_4, isr_stub_5, isr_stub_6, isr_stub_7
        .long isr_stub_8, isr_stub_9, isr_stub_10, isr_stub_11, isr_stub_12, isr_stub_13, isr_stub_14, isr_stub_15
        .long isr_stub_16, isr_stub_17, isr_stub_18, isr_stub_19, isr_stub_20, isr_stub_21, isr_stub_22, isr_stub_23
        .long isr_stub_24, isr_stub_25, isr_stub_26, isr_stub_27, isr_stub_28, isr_stub_29, isr_stub_30, isr_stub_31
        .long isr_stub_32, isr_stub_33, isr_stub_34, isr_stub_35, isr_stub_36, isr_stub_37, isr_stub_38, isr_stub_39
        .long isr_stub_40, isr_stub_41, isr_stub_42, isr_stub_43, isr_stub_44, isr_stub_45, isr_stub_46, isr_stub_47
    "#
);

#[no_mangle]
pub extern "C" fn interrupt_dispatch(frame: &TrapFrame) {
    let int_no = frame.int_no;

    if int_no < 32 {
        // CPU Exception
        handle_exception(frame);
    } else if int_no == 32 {
        // IRQ0: PIT Timer
        pit::on_tick();
        pic::send_eoi(0);
    } else if int_no == 33 {
        // IRQ1: PS/2 Keyboard
        keyboard::on_irq1();
        pic::send_eoi(1);
    } else if int_no < 48 {
        // Other hardware IRQ
        pic::send_eoi((int_no - 32) as u8);
    }
}

fn handle_exception(frame: &TrapFrame) -> ! {
    let names = [
        "Divide-by-zero Error",
        "Debug",
        "Non-maskable Interrupt",
        "Breakpoint",
        "Overflow",
        "Bound Range Exceeded",
        "Invalid Opcode",
        "Device Not Available",
        "Double Fault",
        "Coprocessor Segment Overrun",
        "Invalid TSS",
        "Segment Not Present",
        "Stack-Segment Fault",
        "General Protection Fault",
        "Page Fault",
        "Reserved",
        "x87 Floating-Point Exception",
        "Alignment Check",
        "Machine Check",
        "SIMD Floating-Point Exception",
        "Virtualization Exception",
        "Control Protection Exception",
    ];

    let name = if (frame.int_no as usize) < names.len() {
        names[frame.int_no as usize]
    } else {
        "CPU Exception"
    };

    crate::serial_println!("\n*** CPU EXCEPTION: {} (Vector {}) ***", name, frame.int_no);
    crate::serial_println!("EIP: {:#010X}  CS: {:#06X}  EFLAGS: {:#010X}", frame.eip, frame.cs, frame.eflags);
    crate::serial_println!("Error Code: {:#010X}", frame.err_code);

    {
        let mut vga = VGA.lock();
        vga.set_color(Color::LightRed, Color::Black);
        vga.new_line();
        vga.write_str("========================================\n");
        vga.write_str("           CPU EXCEPTION                \n");
        vga.write_str("========================================\n");
    }

    crate::println!("Exception: {} (Vector {})", name, frame.int_no);
    crate::println!("EIP: {:#010X}  Error: {:#010X}", frame.eip, frame.err_code);
    crate::println!("System halted.");

    loop {
        // SAFETY: Halting CPU on fatal CPU exception
        unsafe {
            asm!("cli; hlt");
        }
    }
}

pub fn init() {
    let idt_ptr = addr_of_mut!(IDT) as *mut IdtEntry;
    // Populate first 48 entries (exceptions 0..31, IRQs 32..47)
    for i in 0..48 {
        // SAFETY: Loading verified stubs into static IDT table
        unsafe {
            (*idt_ptr.add(i)).set_handler(isr_stub_table[i]);
        }
    }

    let idt_desc = IdtDescriptor {
        size: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        // SAFETY: Pointer to static IDT in kernel memory
        offset: idt_ptr as u32,
    };

    // SAFETY: Loading IDTR register and enabling hardware interrupts with sti
    unsafe {
        asm!("lidt [{desc}]", desc = in(reg) &idt_desc, options(nomem, nostack));
        asm!("sti");
    }
}
