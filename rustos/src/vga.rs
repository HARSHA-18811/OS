//! VGA Text Mode Driver (80x25 characters at 0xB8000)

use core::fmt;
use crate::io::outb;
use crate::sync::Spinlock;

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_ADDR: usize = 0xB8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii: u8,
    pub color_code: ColorCode,
}

pub struct VgaWriter {
    pub col: usize,
    pub row: usize,
    pub color_code: ColorCode,
    pub max_scroll_row: usize, // allows reserving row 24 for status bar if needed
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            max_scroll_row: BUFFER_HEIGHT - 2, // row 23 (row 24 reserved for status bar)
        }
    }

    fn buffer_ptr(&self) -> *mut ScreenChar {
        VGA_BUFFER_ADDR as *mut ScreenChar
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color_code = ColorCode::new(fg, bg);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.col = 0,
            0x08 => {
                // Backspace
                if self.col > 0 {
                    self.col -= 1;
                    self.write_char_at(self.col, self.row, b' ', self.color_code);
                }
            }
            0x09 => {
                // Tab (advance to next 4-space tab stop)
                let next_stop = (self.col + 4) & !3;
                let spaces = next_stop.saturating_sub(self.col);
                for _ in 0..spaces {
                    self.write_byte(b' ');
                }
            }
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.write_char_at(self.col, self.row, byte, self.color_code);
                self.col += 1;
            }
        }
        self.update_hardware_cursor();
    }

    pub fn write_char(&mut self, c: char) {
        self.write_byte(c as u8);
    }

    pub fn backspace(&mut self) {
        self.write_byte(0x08);
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_char_at(&self, col: usize, row: usize, ascii: u8, color_code: ColorCode) {
        if col < BUFFER_WIDTH && row < BUFFER_HEIGHT {
            let offset = row * BUFFER_WIDTH + col;
            // SAFETY: Valid coordinates within 80x25 VGA buffer at 0xB8000
            unsafe {
                self.buffer_ptr().add(offset).write_volatile(ScreenChar {
                    ascii,
                    color_code,
                });
            }
        }
    }

    pub fn write_string_at(&self, col: usize, row: usize, s: &str, color_code: ColorCode) {
        for (i, byte) in s.bytes().enumerate() {
            if col + i < BUFFER_WIDTH {
                self.write_char_at(col + i, row, byte, color_code);
            }
        }
    }

    pub fn new_line(&mut self) {
        if self.row < self.max_scroll_row {
            self.row += 1;
        } else {
            self.scroll_up();
        }
        self.col = 0;
        self.update_hardware_cursor();
    }

    pub fn scroll_up(&mut self) {
        let max_r = self.max_scroll_row;
        // Shift lines 1..=max_r up by one line
        for r in 1..=max_r {
            for c in 0..BUFFER_WIDTH {
                let src_offset = r * BUFFER_WIDTH + c;
                let dst_offset = (r - 1) * BUFFER_WIDTH + c;
                // SAFETY: Reading and writing valid VGA buffer positions
                unsafe {
                    let character = self.buffer_ptr().add(src_offset).read_volatile();
                    self.buffer_ptr().add(dst_offset).write_volatile(character);
                }
            }
        }
        // Clear bottom row (max_r)
        self.clear_row(max_r, self.color_code);
    }

    pub fn clear_row(&self, row: usize, color_code: ColorCode) {
        if row < BUFFER_HEIGHT {
            let blank = ScreenChar {
                ascii: b' ',
                color_code,
            };
            for c in 0..BUFFER_WIDTH {
                let offset = row * BUFFER_WIDTH + c;
                // SAFETY: Valid index in VGA buffer
                unsafe {
                    self.buffer_ptr().add(offset).write_volatile(blank);
                }
            }
        }
    }

    pub fn clear_screen(&mut self) {
        let blank = ScreenChar {
            ascii: b' ',
            color_code: self.color_code,
        };
        for offset in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            // SAFETY: Clearing 80x25 characters
            unsafe {
                self.buffer_ptr().add(offset).write_volatile(blank);
            }
        }
        self.col = 0;
        self.row = 0;
        self.update_hardware_cursor();
    }

    pub fn set_cursor(&mut self, col: usize, row: usize) {
        self.col = col.min(BUFFER_WIDTH - 1);
        self.row = row.min(BUFFER_HEIGHT - 1);
        self.update_hardware_cursor();
    }

    pub fn update_hardware_cursor(&self) {
        let pos = (self.row * BUFFER_WIDTH + self.col) as u16;
        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }

    pub fn enable_cursor(&self, cursor_start: u8, cursor_end: u8) {
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb_helper(0x3D5) & 0xC0) | cursor_start);
        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb_helper(0x3D5) & 0xE0) | cursor_end);
    }

    pub fn disable_cursor(&self) {
        outb(0x3D4, 0x0A);
        outb(0x3D5, 0x20); // Bit 5 disables cursor
    }
}

fn inb_helper(port: u16) -> u8 {
    crate::io::inb(port)
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub static VGA: Spinlock<VgaWriter> = Spinlock::new(VgaWriter::new());

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let mut vga = VGA.lock();
    let _ = vga.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
