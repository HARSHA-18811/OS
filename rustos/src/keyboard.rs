//! PS/2 Keyboard Driver (IRQ1) with Scancode Queue and Arrow Key Support

use crate::io::inb;
use crate::sync::Spinlock;

pub const KEYBOARD_DATA_PORT: u16 = 0x60;
pub const KEYBOARD_STATUS_PORT: u16 = 0x64;

/// Initializes the keyboard driver and flushes any stale BIOS/GRUB scancodes.
pub fn init() {
    // Drain PS/2 controller output buffer if dirty
    while (inb(KEYBOARD_STATUS_PORT) & 1) != 0 {
        let _ = inb(KEYBOARD_DATA_PORT);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Esc,
    Up,
    Down,
    Left,
    Right,
}

const QUEUE_SIZE: usize = 64;

struct ScancodeQueue {
    buffer: [u8; QUEUE_SIZE],
    head: usize,
    tail: usize,
}

impl ScancodeQueue {
    const fn new() -> Self {
        Self {
            buffer: [0; QUEUE_SIZE],
            head: 0,
            tail: 0,
        }
    }

    fn push(&mut self, scancode: u8) {
        let next = (self.head + 1) % QUEUE_SIZE;
        if next != self.tail {
            self.buffer[self.head] = scancode;
            self.head = next;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            None
        } else {
            let scancode = self.buffer[self.tail];
            self.tail = (self.tail + 1) % QUEUE_SIZE;
            Some(scancode)
        }
    }
}

static SCANCODE_QUEUE: Spinlock<ScancodeQueue> = Spinlock::new(ScancodeQueue::new());

struct DecoderState {
    extended: bool,
    shift: bool,
    caps_lock: bool,
}

static DECODER: Spinlock<DecoderState> = Spinlock::new(DecoderState {
    extended: false,
    shift: false,
    caps_lock: false,
});

/// Called from IRQ1 interrupt handler.
pub fn on_irq1() {
    let scancode = inb(KEYBOARD_DATA_PORT);
    SCANCODE_QUEUE.lock().push(scancode);
}

/// Decodes the next key event from the scancode queue if available.
pub fn try_get_key() -> Option<Key> {
    loop {
        let scancode = SCANCODE_QUEUE.lock().pop()?;
        let mut state = DECODER.lock();

        if scancode == 0xE0 {
            state.extended = true;
            continue;
        }

        if state.extended {
            state.extended = false;
            // Extended scancodes
            match scancode {
                0x48 => return Some(Key::Up),
                0x50 => return Some(Key::Down),
                0x4B => return Some(Key::Left),
                0x4D => return Some(Key::Right),
                _ => continue, // Ignore extended release or other keys
            }
        }

        // Check key release (bit 7 set)
        if scancode & 0x80 != 0 {
            let released = scancode & 0x7F;
            if released == 0x2A || released == 0x36 {
                // Left shift or right shift released
                state.shift = false;
            }
            continue;
        }

        // Key pressed
        match scancode {
            0x2A | 0x36 => {
                state.shift = true;
                continue;
            }
            0x3A => {
                state.caps_lock = !state.caps_lock;
                continue;
            }
            0x1C => return Some(Key::Enter),
            0x0E => return Some(Key::Backspace),
            0x0F => return Some(Key::Tab),
            0x01 => return Some(Key::Esc),
            code => {
                let shifted = state.shift ^ state.caps_lock;
                if let Some(ch) = map_scancode_to_char(code, state.shift, shifted) {
                    return Some(Key::Char(ch));
                }
            }
        }
    }
}

/// Blocking wait for the next key press.
pub fn get_key() -> Key {
    loop {
        if let Some(key) = try_get_key() {
            return key;
        }
        // SAFETY: Halt CPU until next interrupt (timer or keyboard)
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn map_scancode_to_char(scancode: u8, shift: bool, letter_upper: bool) -> Option<char> {
    let ch = match scancode {
        0x02 => if shift { '!' } else { '1' },
        0x03 => if shift { '@' } else { '2' },
        0x04 => if shift { '#' } else { '3' },
        0x05 => if shift { '$' } else { '4' },
        0x06 => if shift { '%' } else { '5' },
        0x07 => if shift { '^' } else { '6' },
        0x08 => if shift { '&' } else { '7' },
        0x09 => if shift { '*' } else { '8' },
        0x0A => if shift { '(' } else { '9' },
        0x0B => if shift { ')' } else { '0' },
        0x0C => if shift { '_' } else { '-' },
        0x0D => if shift { '+' } else { '=' },
        0x10 => if letter_upper { 'Q' } else { 'q' },
        0x11 => if letter_upper { 'W' } else { 'w' },
        0x12 => if letter_upper { 'E' } else { 'e' },
        0x13 => if letter_upper { 'R' } else { 'r' },
        0x14 => if letter_upper { 'T' } else { 't' },
        0x15 => if letter_upper { 'Y' } else { 'y' },
        0x16 => if letter_upper { 'U' } else { 'u' },
        0x17 => if letter_upper { 'I' } else { 'i' },
        0x18 => if letter_upper { 'O' } else { 'o' },
        0x19 => if letter_upper { 'P' } else { 'p' },
        0x1A => if shift { '{' } else { '[' },
        0x1B => if shift { '}' } else { ']' },
        0x1E => if letter_upper { 'A' } else { 'a' },
        0x1F => if letter_upper { 'S' } else { 's' },
        0x20 => if letter_upper { 'D' } else { 'd' },
        0x21 => if letter_upper { 'F' } else { 'f' },
        0x22 => if letter_upper { 'G' } else { 'g' },
        0x23 => if letter_upper { 'H' } else { 'h' },
        0x24 => if letter_upper { 'J' } else { 'j' },
        0x25 => if letter_upper { 'K' } else { 'k' },
        0x26 => if letter_upper { 'L' } else { 'l' },
        0x27 => if shift { ':' } else { ';' },
        0x28 => if shift { '"' } else { '\'' },
        0x29 => if shift { '~' } else { '`' },
        0x2B => if shift { '|' } else { '\\' },
        0x2C => if letter_upper { 'Z' } else { 'z' },
        0x2D => if letter_upper { 'X' } else { 'x' },
        0x2E => if letter_upper { 'C' } else { 'c' },
        0x2F => if letter_upper { 'V' } else { 'v' },
        0x30 => if letter_upper { 'B' } else { 'b' },
        0x31 => if letter_upper { 'N' } else { 'n' },
        0x32 => if letter_upper { 'M' } else { 'm' },
        0x33 => if shift { '<' } else { ',' },
        0x34 => if shift { '>' } else { '.' },
        0x35 => if shift { '?' } else { '/' },
        0x39 => ' ',
        _ => return None,
    };
    Some(ch)
}
