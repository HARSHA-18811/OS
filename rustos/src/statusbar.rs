//! Status Bar for RustOS
//!
//! Renders a dedicated status line at row 24 displaying the active application,
//! system hints, and real-time CMOS clock.

use alloc::format;
use crate::rtc;
use crate::vga::{Color, ColorCode, VGA, BUFFER_WIDTH};

/// Draws the status bar at row 24.
pub fn draw(app_name: &str) {
    let dt = rtc::read_datetime();
    let left_text = format!(" [RustOS] App: {:<14}", app_name);
    let right_text = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} ", dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second);
    let center_text = "[Esc: Menu | Enter: Select]";

    let mut line = [b' '; BUFFER_WIDTH];

    // Copy left text
    let left_bytes = left_text.as_bytes();
    let left_len = left_bytes.len().min(26);
    line[..left_len].copy_from_slice(&left_bytes[..left_len]);

    // Copy center text
    let center_bytes = center_text.as_bytes();
    let center_pos = 28;
    if center_pos + center_bytes.len() < BUFFER_WIDTH {
        line[center_pos..center_pos + center_bytes.len()].copy_from_slice(center_bytes);
    }

    // Copy right text
    let right_bytes = right_text.as_bytes();
    let right_pos = BUFFER_WIDTH.saturating_sub(right_bytes.len());
    line[right_pos..right_pos + right_bytes.len()].copy_from_slice(right_bytes);

    let bar_color = ColorCode::new(Color::Black, Color::LightGray);
    let vga = VGA.lock();
    for col in 0..BUFFER_WIDTH {
        vga.write_char_at(col, 24, line[col], bar_color);
    }
}
