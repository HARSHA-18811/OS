//! 8259A Programmable Interrupt Controller (PIC) Driver

use crate::io::{inb, io_wait, outb};

pub const PIC1_COMMAND: u16 = 0x20;
pub const PIC1_DATA: u16 = 0x21;
pub const PIC2_COMMAND: u16 = 0xA0;
pub const PIC2_DATA: u16 = 0xA1;

pub const PIC_EOI: u8 = 0x20;

// Re-map PIC IRQs to interrupt vectors 0x20..0x2F (32..47)
pub const PIC1_OFFSET: u8 = 0x20;
pub const PIC2_OFFSET: u8 = 0x28;

pub fn init() {
    // Save current masks
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    // ICW1: Start initialization in cascade mode
    outb(PIC1_COMMAND, 0x11);
    io_wait();
    outb(PIC2_COMMAND, 0x11);
    io_wait();

    // ICW2: Vector offsets
    outb(PIC1_DATA, PIC1_OFFSET);
    io_wait();
    outb(PIC2_DATA, PIC2_OFFSET);
    io_wait();

    // ICW3: Cascade wiring
    outb(PIC1_DATA, 0x04); // Master has slave at IRQ2 (0000 0100b)
    io_wait();
    outb(PIC2_DATA, 0x02); // Slave attached to master's IRQ2
    io_wait();

    // ICW4: 8086/88 (MCS-80/85) mode
    outb(PIC1_DATA, 0x01);
    io_wait();
    outb(PIC2_DATA, 0x01);
    io_wait();

    // Restore masks, enabling only IRQ0 (timer) and IRQ1 (keyboard) initially
    // IRQ0 = bit 0 (0), IRQ1 = bit 1 (0) -> mask = 0xFC (1111 1100b)
    outb(PIC1_DATA, 0xFC);
    outb(PIC2_DATA, 0xFF); // Disable all slave IRQs for now
    let _ = (mask1, mask2);
}

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI);
    }
    outb(PIC1_COMMAND, PIC_EOI);
}

pub fn unmask_irq(irq: u8) {
    if irq < 8 {
        let mask = inb(PIC1_DATA) & !(1 << irq);
        outb(PIC1_DATA, mask);
    } else {
        let mask = inb(PIC2_DATA) & !(1 << (irq - 8));
        outb(PIC2_DATA, mask);
    }
}
