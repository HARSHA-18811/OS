//! Programmable Interval Timer (PIT 8254) Driver (Channel 0 at 100 Hz)

use core::sync::atomic::{AtomicU32, Ordering};
use crate::io::outb;

pub const PIT_CHANNEL_0: u16 = 0x40;
pub const PIT_COMMAND: u16 = 0x43;
pub const PIT_BASE_FREQUENCY: u32 = 1193182;
pub const TARGET_HZ: u32 = 100; // 100 Hz -> 10ms per tick

static TICKS: AtomicU32 = AtomicU32::new(0);

pub fn init() {
    let divisor = (PIT_BASE_FREQUENCY / TARGET_HZ) as u16; // 11932 (0x2E9C)

    // Command byte: 0x36
    // Channel 0 (00b), Access Mode low/high byte (11b), Mode 3 square wave (011b), Binary (0b)
    outb(PIT_COMMAND, 0x36);
    // Write low byte
    outb(PIT_CHANNEL_0, (divisor & 0xFF) as u8);
    // Write high byte
    outb(PIT_CHANNEL_0, ((divisor >> 8) & 0xFF) as u8);
}

/// Called from IRQ0 interrupt handler.
pub fn on_tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

/// Returns the total ticks since boot (100 Hz).
pub fn get_ticks() -> u32 {
    TICKS.load(Ordering::Relaxed)
}

/// Sleep for the specified number of milliseconds using PIT ticks.
pub fn sleep_ms(ms: u32) {
    let ticks_needed = (ms + 9) / 10;
    let target = get_ticks().wrapping_add(ticks_needed);
    while (get_ticks().wrapping_sub(target) as i32) < 0 {
        // SAFETY: Halting the CPU waiting for the next timer interrupt
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Returns system uptime in total seconds.
pub fn uptime_seconds() -> u32 {
    get_ticks() / TARGET_HZ
}

/// Returns uptime broken down into (hours, minutes, seconds).
pub fn uptime_hms() -> (u32, u32, u32) {
    let total = uptime_seconds();
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;
    (hours, minutes, seconds)
}
