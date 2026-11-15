//! Desktop Application Menu for RustOS
//!
//! Provides a visual desktop menu to launch applications, games,
//! research laboratory, text editor, and system information.

use crate::keyboard::{get_key, Key};
use crate::statusbar;
use crate::vga::{Color, ColorCode, VGA, BUFFER_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppChoice {
    Shell,
    Snake,
    SnakesLadders,
    CarDodge,
    ResearchLab,
    Files,
    About,
}

struct MenuItem {
    title: &'static str,
    desc: &'static str,
    choice: AppChoice,
}

const MENU_ITEMS: [MenuItem; 7] = [
    MenuItem {
        title: "1. Shell                  ",
        desc: "Interactive command-line shell (27 commands)",
        choice: AppChoice::Shell,
    },
    MenuItem {
        title: "2. Snake                  ",
        desc: "Classic arcade snake game with score & food",
        choice: AppChoice::Snake,
    },
    MenuItem {
        title: "3. Snake and Ladders      ",
        desc: "10x10 board game vs Computer AI",
        choice: AppChoice::SnakesLadders,
    },
    MenuItem {
        title: "4. Car Dodge              ",
        desc: "3-lane highway reflex survival game",
        choice: AppChoice::CarDodge,
    },
    MenuItem {
        title: "5. Producer-Consumer Lab  ",
        desc: "Bounded-buffer performance research lab",
        choice: AppChoice::ResearchLab,
    },
    MenuItem {
        title: "6. Files (Text Editor)    ",
        desc: "Browse and edit in-memory files (vi)",
        choice: AppChoice::Files,
    },
    MenuItem {
        title: "7. About RustOS           ",
        desc: "System architecture, authors & specs",
        choice: AppChoice::About,
    },
];

/// Renders the desktop menu and blocks until an application is selected.
pub fn show_desktop_menu() -> AppChoice {
    let mut selected: usize = 0;

    loop {
        render_menu(selected);
        statusbar::draw("Desktop");
        crate::serial_println!("Desktop menu rendered (selected={}), waiting for key...", selected);

        let key = get_key();
        crate::serial_println!("Desktop menu got key: {:?}", key);
        match key {
            Key::Up => {
                if selected == 0 {
                    selected = MENU_ITEMS.len() - 1;
                } else {
                    selected -= 1;
                }
            }
            Key::Down => {
                if selected + 1 >= MENU_ITEMS.len() {
                    selected = 0;
                } else {
                    selected += 1;
                }
            }
            Key::Char(c) => match c {
                '1' => return AppChoice::Shell,
                '2' => return AppChoice::Snake,
                '3' => return AppChoice::SnakesLadders,
                '4' => return AppChoice::CarDodge,
                '5' => return AppChoice::ResearchLab,
                '6' => return AppChoice::Files,
                '7' => return AppChoice::About,
                _ => {}
            },
            Key::Enter => {
                return MENU_ITEMS[selected].choice;
            }
            Key::Esc => {
                // Esc from desktop menu opens default screen (Shell)
                return AppChoice::Shell;
            }
            _ => {}
        }
    }
}

/// Draws the menu box and items onto the VGA text buffer.
fn render_menu(selected: usize) {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let title_color = ColorCode::new(Color::Yellow, Color::Blue);
    let border_color = ColorCode::new(Color::Cyan, Color::Black);
    let item_color = ColorCode::new(Color::White, Color::Black);
    let sel_color = ColorCode::new(Color::Black, Color::LightCyan);
    let desc_color = ColorCode::new(Color::LightGray, Color::Black);

    // Header banner
    for col in 0..BUFFER_WIDTH {
        vga.write_char_at(col, 1, b' ', title_color);
        vga.write_char_at(col, 2, b' ', title_color);
    }
    vga.write_string_at(22, 1, "RUSTOS DESKTOP ENVIRONMENT", title_color);
    vga.write_string_at(16, 2, "Educational x86 OS -- Oracle VirtualBox Sandbox", title_color);

    // Outer frame box
    let box_left = 6;
    let box_right = 73;
    let box_top = 4;
    let box_bottom = 21;

    // Top and bottom borders
    for col in box_left..=box_right {
        vga.write_char_at(col, box_top, b'=', border_color);
        vga.write_char_at(col, box_bottom, b'=', border_color);
    }
    for row in box_top..=box_bottom {
        vga.write_char_at(box_left, row, b'|', border_color);
        vga.write_char_at(box_right, row, b'|', border_color);
    }

    vga.write_string_at(box_left + 2, box_top, "[ Applications Menu ]", ColorCode::new(Color::Yellow, Color::Black));

    // Draw menu items
    let start_row = 6;
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let row = start_row + i * 2;
        let is_sel = i == selected;

        if is_sel {
            // Draw highlight bar
            for col in (box_left + 2)..(box_right - 2) {
                vga.write_char_at(col, row, b' ', sel_color);
            }
            vga.write_string_at(box_left + 4, row, "> ", sel_color);
            vga.write_string_at(box_left + 6, row, item.title, sel_color);
            vga.write_string_at(box_left + 30, row, item.desc, ColorCode::new(Color::Black, Color::LightCyan));
        } else {
            vga.write_string_at(box_left + 4, row, "  ", item_color);
            vga.write_string_at(box_left + 6, row, item.title, item_color);
            vga.write_string_at(box_left + 30, row, item.desc, desc_color);
        }
    }

    // Navigation hint at bottom of box
    let hint = "Navigate: [Up/Down] or [1-7]  |  Open: [Enter]  |  Shell: [Esc]";
    vga.write_string_at(box_left + 4, box_bottom - 1, hint, ColorCode::new(Color::LightGreen, Color::Black));
}

/// Displays the About information screen.
pub fn show_about_screen() {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let header_color = ColorCode::new(Color::Yellow, Color::Blue);
    let text_color = ColorCode::new(Color::White, Color::Black);
    let highlight_color = ColorCode::new(Color::LightCyan, Color::Black);

    for col in 0..BUFFER_WIDTH {
        vga.write_char_at(col, 1, b' ', header_color);
    }
    vga.write_string_at(26, 1, "ABOUT RUSTOS VERSION 1.0.0", header_color);

    vga.write_string_at(4, 4, "Operating System:", highlight_color);
    vga.write_string_at(24, 4, "RustOS (32-bit x86 bare-metal ELF)", text_color);

    vga.write_string_at(4, 6, "Architecture:", highlight_color);
    vga.write_string_at(24, 6, "i686 Protected Mode, Flat 4GB Segmentation", text_color);

    vga.write_string_at(4, 8, "Sandbox Environment:", highlight_color);
    vga.write_string_at(24, 8, "Oracle VirtualBox Virtual Machine (BIOS)", text_color);

    vga.write_string_at(4, 10, "Memory Management:", highlight_color);
    vga.write_string_at(24, 10, "2 MiB Linked-List Allocator with Block Coalescing", text_color);

    vga.write_string_at(4, 12, "Hardware Drivers:", highlight_color);
    vga.write_string_at(24, 12, "VGA 80x25, 8259 PIC, 8254 PIT (100Hz), PS/2, CMOS RTC", text_color);

    vga.write_string_at(4, 14, "Research Lab:", highlight_color);
    vga.write_string_at(24, 14, "Producer-Consumer Performance Bounded Buffer Study", text_color);

    vga.write_string_at(4, 16, "Input Note:", highlight_color);
    vga.write_string_at(24, 16, "Keyboard-driven UI. Mouse support is a planned future step.", text_color);

    vga.write_string_at(4, 19, "Press [Esc] or [Enter] to return to Desktop Menu...", ColorCode::new(Color::LightGreen, Color::Black));
    drop(vga);

    statusbar::draw("About");

    loop {
        let key = get_key();
        if key == Key::Esc || key == Key::Enter {
            break;
        }
    }
}
