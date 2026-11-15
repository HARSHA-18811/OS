//! Low-level x86 Port I/O Helpers

use core::arch::asm;

/// Writes an 8-bit byte to the specified I/O port.
#[inline]
pub fn outb(port: u16, val: u8) {
    // SAFETY: Executing out instruction to write a byte to an x86 hardware port
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
    }
}

/// Reads an 8-bit byte from the specified I/O port.
#[inline]
pub fn inb(port: u16) -> u8 {
    let val: u8;
    // SAFETY: Executing in instruction to read a byte from an x86 hardware port
    unsafe {
        asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    val
}

/// Writes a 16-bit word to the specified I/O port.
#[inline]
pub fn outw(port: u16, val: u16) {
    // SAFETY: Executing out instruction to write a 16-bit word to an x86 hardware port
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack, preserves_flags));
    }
}

/// Reads a 16-bit word from the specified I/O port.
#[inline]
pub fn inw(port: u16) -> u16 {
    let val: u16;
    // SAFETY: Executing in instruction to read a 16-bit word from an x86 hardware port
    unsafe {
        asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    val
}

/// Brief delay for slow hardware I/O operations (writing to unused port 0x80).
#[inline]
pub fn io_wait() {
    outb(0x80, 0);
}
