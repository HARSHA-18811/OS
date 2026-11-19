//! COM1 Serial Port Driver (Port 0x3F8)

use core::fmt;
use crate::io::{inb, outb};
use crate::sync::Spinlock;

pub const COM1_PORT: u16 = 0x3F8;

pub struct SerialPort {
    port: u16,
    initialized: bool,
}

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            initialized: false,
        }
    }

    /// Initializes COM1 with 38400 baud, 8N1, FIFO enabled.
    pub fn init(&mut self) {
        let p = self.port;
        // Disable all interrupts
        outb(p + 1, 0x00);
        // Enable DLAB (set baud rate divisor)
        outb(p + 3, 0x80);
        // Set divisor to 3 (38400 baud) or 1 (115200 baud); 3 is standard for VirtualBox
        outb(p + 0, 0x03);
        outb(p + 1, 0x00);
        // 8 bits, no parity, one stop bit (8N1)
        outb(p + 3, 0x03);
        // Enable FIFO, clear them, with 14-byte threshold
        outb(p + 2, 0xC7);
        // IRQs enabled, RTS/DSR set
        outb(p + 4, 0x0B);
        self.initialized = true;
    }

    fn line_status(&self) -> u8 {
        inb(self.port + 5)
    }

    fn is_transmit_empty(&self) -> bool {
        (self.line_status() & 0x20) != 0
    }

    pub fn write_byte(&mut self, byte: u8) {
        if !self.initialized {
            self.init();
        }
        while !self.is_transmit_empty() {
            core::hint::spin_loop();
        }
        outb(self.port, byte);
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub static SERIAL1: Spinlock<SerialPort> = Spinlock::new(SerialPort::new(COM1_PORT));

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    use fmt::Write;
    let mut serial = SERIAL1.lock();
    let _ = serial.write_fmt(args);
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_serial_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
