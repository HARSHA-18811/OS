//! Snake and Ladders Board Game for RustOS
//!
//! Two player turn-based board game (Human vs Computer AI) on a 10x10 grid (1..100)
//! with ladders, snakes, dice rolling, and victory detection.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use super::Rng;
use crate::keyboard::{get_key, Key};
use crate::pit;
use crate::rtc;
use crate::statusbar;
use crate::vga::{Color, ColorCode, VGA};

// Fixed list of Ladders (start -> end)
const LADDERS: [(u8, u8); 6] = [
    (4, 14),
    (9, 31),
    (20, 38),
    (28, 84),
    (40, 59),
    (63, 81),
];

// Fixed list of Snakes (head -> tail)
const SNAKES: [(u8, u8); 6] = [
    (17, 7),
    (54, 34),
    (62, 19),
    (64, 60),
    (87, 24),
    (95, 75),
];

pub fn play() {
    loop {
        let restart = run_game();
        if !restart {
            break;
        }
    }
}

fn run_game() -> bool {
    let mut rng = Rng::new(pit::get_ticks().wrapping_add(rtc::read_datetime().second as u32));
    let mut human_pos: u8 = 1;
    let mut comp_pos: u8 = 1;
    let mut log_messages: Vec<String> = Vec::new();
    log_messages.push(String::from("Game started! Press [Enter] to roll."));

    loop {
        draw_board(human_pos, comp_pos);
        draw_sidebar(&log_messages, true, 0);
        statusbar::draw("Snakes & Ladders");

        // Human Turn
        let roll = wait_for_roll();
        if roll == 0 {
            return false; // Esc pressed
        }

        let roll_val = rng.range(1, 6) as u8;
        let mut new_pos = (human_pos + roll_val).min(100);
        let mut msg = format!("You rolled a {} -> square {}", roll_val, new_pos);

        // Check ladder or snake
        if let Some(&(_, dst)) = LADDERS.iter().find(|&&(src, _)| src == new_pos) {
            new_pos = dst;
            msg = format!("Rolled {}! LADDER up to {}!", roll_val, dst);
        } else if let Some(&(_, dst)) = SNAKES.iter().find(|&&(src, _)| src == new_pos) {
            new_pos = dst;
            msg = format!("Rolled {}! BITTEN! Down to {}!", roll_val, dst);
        }
        human_pos = new_pos;
        add_log(&mut log_messages, msg);

        draw_board(human_pos, comp_pos);
        draw_sidebar(&log_messages, false, roll_val);

        if human_pos >= 100 {
            return show_winner("HUMAN WON! Congratulations!");
        }

        // Computer AI Turn
        pit::sleep_ms(600);
        let comp_roll = rng.range(1, 6) as u8;
        let mut new_comp = (comp_pos + comp_roll).min(100);
        let mut comp_msg = format!("Computer rolled {} -> square {}", comp_roll, new_comp);

        if let Some(&(_, dst)) = LADDERS.iter().find(|&&(src, _)| src == new_comp) {
            new_comp = dst;
            comp_msg = format!("Computer got {}! LADDER to {}!", comp_roll, dst);
        } else if let Some(&(_, dst)) = SNAKES.iter().find(|&&(src, _)| src == new_comp) {
            new_comp = dst;
            comp_msg = format!("Computer got {}! BITTEN to {}!", comp_roll, dst);
        }
        comp_pos = new_comp;
        add_log(&mut log_messages, comp_msg);

        draw_board(human_pos, comp_pos);
        draw_sidebar(&log_messages, true, comp_roll);

        if comp_pos >= 100 {
            return show_winner("COMPUTER WON! Better luck next time!");
        }
    }
}

fn add_log(logs: &mut Vec<String>, msg: String) {
    if logs.len() >= 6 {
        logs.remove(0);
    }
    logs.push(msg);
}

fn wait_for_roll() -> u8 {
    loop {
        let key = get_key();
        match key {
            Key::Enter | Key::Char(' ') => return 1,
            Key::Esc => return 0,
            _ => {}
        }
    }
}

fn square_to_screen(sq: u8) -> (usize, usize) {
    let sq0 = sq.saturating_sub(1).min(99);
    let row_idx = (sq0 / 10) as usize; // 0..9 (0 is bottom)
    let col_idx = (sq0 % 10) as usize; // 0..9

    // Serpentine order
    let actual_col = if row_idx % 2 == 1 {
        9 - col_idx
    } else {
        col_idx
    };

    let screen_row = 2 + (9 - row_idx) * 2;
    let screen_col = 2 + actual_col * 5;
    (screen_col, screen_row)
}

fn draw_board(human_pos: u8, comp_pos: u8) {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let header_color = ColorCode::new(Color::Yellow, Color::Blue);
    vga.write_string_at(0, 0, " SNAKE AND LADDERS -- 10x10 Board Game vs Computer AI | [Esc] Exit to Menu    ", header_color);

    // Draw all 100 squares
    for sq in 1..=100 {
        let (col, row) = square_to_screen(sq);
        let is_ladder = LADDERS.iter().any(|&(s, _)| s == sq);
        let is_snake = SNAKES.iter().any(|&(s, _)| s == sq);

        let sq_color = if is_ladder {
            ColorCode::new(Color::LightGreen, Color::Black)
        } else if is_snake {
            ColorCode::new(Color::LightRed, Color::Black)
        } else if (sq / 10 + sq % 10) % 2 == 0 {
            ColorCode::new(Color::LightGray, Color::Black)
        } else {
            ColorCode::new(Color::White, Color::Black)
        };

        let sq_str = format!("{:>3}", sq);
        vga.write_string_at(col, row, &sq_str, sq_color);

        // Player indicators
        if sq == human_pos && sq == comp_pos {
            vga.write_string_at(col, row + 1, "H&C", ColorCode::new(Color::Yellow, Color::Red));
        } else if sq == human_pos {
            vga.write_string_at(col, row + 1, "[H]", ColorCode::new(Color::Yellow, Color::Blue));
        } else if sq == comp_pos {
            vga.write_string_at(col, row + 1, "[C]", ColorCode::new(Color::LightCyan, Color::Magenta));
        } else if is_ladder {
            vga.write_string_at(col, row + 1, "^L^", ColorCode::new(Color::LightGreen, Color::Black));
        } else if is_snake {
            vga.write_string_at(col, row + 1, "vSv", ColorCode::new(Color::LightRed, Color::Black));
        } else {
            vga.write_string_at(col, row + 1, "   ", sq_color);
        }
    }
}

fn draw_sidebar(logs: &[String], is_human_turn: bool, last_roll: u8) {
    let vga = VGA.lock();
    let left = 54;
    let label_color = ColorCode::new(Color::Cyan, Color::Black);
    let text_color = ColorCode::new(Color::White, Color::Black);

    vga.write_string_at(left, 2, "=== GAME PANEL ===", label_color);

    let turn_str = if is_human_turn {
        "Turn: >> HUMAN <<"
    } else {
        "Turn: .. COMPUTER .."
    };
    let turn_color = if is_human_turn {
        ColorCode::new(Color::Yellow, Color::Blue)
    } else {
        ColorCode::new(Color::LightCyan, Color::Black)
    };
    vga.write_string_at(left, 4, turn_str, turn_color);

    let roll_str = if last_roll > 0 {
        format!("Last Roll: [{}]", last_roll)
    } else {
        format!("Last Roll: [-]")
    };
    vga.write_string_at(left, 6, &roll_str, text_color);

    vga.write_string_at(left, 8, "Ladders: 4,9,20,28,40,63", ColorCode::new(Color::LightGreen, Color::Black));
    vga.write_string_at(left, 9, "Snakes:  17,54,62,64,87,95", ColorCode::new(Color::LightRed, Color::Black));

    vga.write_string_at(left, 11, "--- Move History ---", label_color);
    for (i, msg) in logs.iter().enumerate() {
        let truncated = if msg.len() > 24 { &msg[..24] } else { msg };
        vga.write_string_at(left, 13 + i, truncated, text_color);
    }

    vga.write_string_at(left, 21, "[Space/Enter] Roll Dice", ColorCode::new(Color::LightGreen, Color::Black));
}

fn show_winner(msg: &str) -> bool {
    let vga = VGA.lock();
    let box_left = 18;
    let box_top = 8;
    let box_right = 62;
    let box_bottom = 14;

    for y in box_top..=box_bottom {
        for x in box_left..=box_right {
            vga.write_char_at(x, y, b' ', ColorCode::new(Color::Yellow, Color::Blue));
        }
    }

    vga.write_string_at(box_left + 6, box_top + 2, msg, ColorCode::new(Color::Yellow, Color::Blue));
    vga.write_string_at(box_left + 4, box_top + 4, "[Enter] Play Again  |  [Esc] Menu", ColorCode::new(Color::White, Color::Blue));
    drop(vga);

    loop {
        let key = get_key();
        match key {
            Key::Enter => return true,
            Key::Esc => return false,
            _ => {}
        }
    }
}
