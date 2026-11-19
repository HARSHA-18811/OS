//! Real-Time Clock (RTC / CMOS) Driver
//!
//! Reads system time and date from CMOS registers via I/O ports 0x70 and 0x71.
//! Handles BCD/Binary format and 12/24-hour mode correctly.

use crate::io::{inb, outb};

const CMOS_ADDR: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

const REG_SEC: u8 = 0x00;
const REG_MIN: u8 = 0x02;
const REG_HOUR: u8 = 0x04;
const REG_DAY: u8 = 0x07;
const REG_MONTH: u8 = 0x08;
const REG_YEAR: u8 = 0x09;
const REG_CENTURY: u8 = 0x32;
const REG_STATUS_A: u8 = 0x0A;
const REG_STATUS_B: u8 = 0x0B;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub day_name: &'static str,
}

impl DateTime {
    /// Formats the time as "HH:MM:SS" into the provided byte buffer.
    /// Returns the number of bytes written (always 8).
    pub fn format_time(&self, buf: &mut [u8]) -> usize {
        if buf.len() < 8 {
            return 0;
        }
        buf[0] = b'0' + (self.hour / 10);
        buf[1] = b'0' + (self.hour % 10);
        buf[2] = b':';
        buf[3] = b'0' + (self.minute / 10);
        buf[4] = b'0' + (self.minute % 10);
        buf[5] = b':';
        buf[6] = b'0' + (self.second / 10);
        buf[7] = b'0' + (self.second % 10);
        8
    }

    /// Formats the date as "YYYY-MM-DD" into the provided byte buffer.
    /// Returns the number of bytes written (always 10).
    pub fn format_date(&self, buf: &mut [u8]) -> usize {
        if buf.len() < 10 {
            return 0;
        }
        let y = self.year;
        buf[0] = b'0' + ((y / 1000) % 10) as u8;
        buf[1] = b'0' + ((y / 100) % 10) as u8;
        buf[2] = b'0' + ((y / 10) % 10) as u8;
        buf[3] = b'0' + (y % 10) as u8;
        buf[4] = b'-';
        buf[5] = b'0' + (self.month / 10);
        buf[6] = b'0' + (self.month % 10);
        buf[7] = b'-';
        buf[8] = b'0' + (self.day / 10);
        buf[9] = b'0' + (self.day % 10);
        10
    }
}

/// Reads a single CMOS register.
fn read_register(reg: u8) -> u8 {
    // Port 0x70 bit 7 is NMI disable; keeping it 0 leaves NMI enabled.
    outb(CMOS_ADDR, reg);
    inb(CMOS_DATA)
}

/// Checks if CMOS update is currently in progress.
fn is_updating() -> bool {
    (read_register(REG_STATUS_A) & 0x80) != 0
}

/// Converts Binary Coded Decimal (BCD) to standard binary integer.
fn bcd_to_binary(val: u8) -> u8 {
    (val & 0x0F) + ((val >> 4) * 10)
}

/// Calculates the day of the week using Sakamoto's algorithm.
fn calculate_day_of_week(year: u16, month: u8, day: u8) -> &'static str {
    static T: [u16; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y = y.saturating_sub(1);
    }
    let m_idx = (month.saturating_sub(1) as usize).min(11);
    let dow = (y + y / 4 - y / 100 + y / 400 + T[m_idx] + day as u16) % 7;
    match dow {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Sunday",
    }
}

/// Reads current system date and time from the RTC CMOS chip.
pub fn read_datetime() -> DateTime {
    // Wait until RTC is not updating to avoid reading transient values
    while is_updating() {}

    // Read registers twice to ensure consistency against rollover
    let mut sec = read_register(REG_SEC);
    let mut min = read_register(REG_MIN);
    let mut hour = read_register(REG_HOUR);
    let mut day = read_register(REG_DAY);
    let mut month = read_register(REG_MONTH);
    let mut year = read_register(REG_YEAR);
    let mut century = read_register(REG_CENTURY);

    loop {
        while is_updating() {}
        let sec2 = read_register(REG_SEC);
        let min2 = read_register(REG_MIN);
        let hour2 = read_register(REG_HOUR);
        let day2 = read_register(REG_DAY);
        let month2 = read_register(REG_MONTH);
        let year2 = read_register(REG_YEAR);
        let century2 = read_register(REG_CENTURY);

        if sec == sec2 && min == min2 && hour == hour2 && day == day2 && month == month2 && year == year2 {
            break;
        }

        sec = sec2;
        min = min2;
        hour = hour2;
        day = day2;
        month = month2;
        year = year2;
        century = century2;
    }

    let status_b = read_register(REG_STATUS_B);
    let is_binary = (status_b & 0x04) != 0;
    let is_24h = (status_b & 0x02) != 0;

    // Convert BCD to binary if needed
    if !is_binary {
        sec = bcd_to_binary(sec);
        min = bcd_to_binary(min);
        hour = bcd_to_binary(hour & 0x7F) | (hour & 0x80);
        day = bcd_to_binary(day);
        month = bcd_to_binary(month);
        year = bcd_to_binary(year);
        century = bcd_to_binary(century);
    }

    // Convert 12h format to 24h format if needed
    if !is_24h {
        let is_pm = (hour & 0x80) != 0;
        let mut h = hour & 0x7F;
        if is_pm && h < 12 {
            h += 12;
        } else if !is_pm && h == 12 {
            h = 0;
        }
        hour = h;
    }

    // Calculate full 4-digit year
    let full_year = if century >= 19 && century <= 21 {
        (century as u16) * 100 + (year as u16)
    } else if year < 80 {
        2000 + (year as u16)
    } else {
        1900 + (year as u16)
    };

    let day_name = calculate_day_of_week(full_year, month, day);

    DateTime {
        year: full_year,
        month,
        day,
        hour,
        minute: min,
        second: sec,
        day_name,
    }
}
