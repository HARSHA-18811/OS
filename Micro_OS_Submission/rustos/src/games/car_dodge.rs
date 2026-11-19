//! Car Dodge Highway Survival Game for RustOS
//!
//! 3-lane reflex racing game with player car steering, falling obstacles,
//! dynamic speed acceleration, and high-score tracking.

use alloc::format;
use alloc::vec::Vec;
use super::Rng;
use crate::keyboard::{try_get_key, Key};
use crate::pit;
use crate::rtc;
use crate::statusbar;
use crate::vga::{Color, ColorCode, VGA, BUFFER_WIDTH};

const ROAD_LEFT: usize = 25;
const ROAD_RIGHT: usize = 55;
const PLAYER_ROW: usize = 21;

// Lane center columns
const LANE_COLS: [usize; 3] = [30, 40, 50];

struct Obstacle {
    lane: usize,
    row: usize,
}

pub fn play() {
    let mut high_score = 0;

    loop {
        let final_score = run_game(high_score);
        if final_score > high_score {
            high_score = final_score;
        }

        let restart = show_game_over(final_score, high_score);
        if !restart {
            break;
        }
    }
}

fn run_game(high_score: u32) -> u32 {
    let mut rng = Rng::new(pit::get_ticks().wrapping_add(rtc::read_datetime().second as u32));
    let mut player_lane: usize = 1; // Middle lane
    let mut obstacles: Vec<Obstacle> = Vec::new();
    let mut score: u32 = 0;

    draw_road();
    draw_header(score, high_score);
    statusbar::draw("Car Dodge");

    let player_color = ColorCode::new(Color::LightCyan, Color::Black);
    let obstacle_color = ColorCode::new(Color::LightRed, Color::Black);
    let clear_color = ColorCode::new(Color::White, Color::Black);

    draw_player(player_lane, player_color);

    let mut last_tick = pit::get_ticks();
    let mut spawn_counter = 0;

    loop {
        // Non-blocking keyboard input
        while let Some(key) = try_get_key() {
            match key {
                Key::Left => {
                    if player_lane > 0 {
                        erase_player(player_lane, clear_color);
                        player_lane -= 1;
                        draw_player(player_lane, player_color);
                    }
                }
                Key::Right => {
                    if player_lane < 2 {
                        erase_player(player_lane, clear_color);
                        player_lane += 1;
                        draw_player(player_lane, player_color);
                    }
                }
                Key::Esc => return score,
                _ => {}
            }
        }

        let now = pit::get_ticks();
        // Speed accelerates as score increases (delay decreases from 9 down to 3)
        let delay_ticks = 9u32.saturating_sub(score / 150).max(3);

        if now.wrapping_sub(last_tick) >= delay_ticks {
            last_tick = now;
            score += 1;

            // Spawn obstacles
            spawn_counter += 1;
            if spawn_counter >= 5 {
                spawn_counter = 0;
                let new_lane = rng.range(0, 2) as usize;
                obstacles.push(Obstacle {
                    lane: new_lane,
                    row: 2,
                });
            }

            // Move and draw road dividers
            animate_road(score);

            // Update obstacles
            let mut i = 0;
            let mut collided = false;

            while i < obstacles.len() {
                // Erase old position
                erase_obstacle(&obstacles[i], clear_color);

                obstacles[i].row += 1;

                // Check collision with player car
                if obstacles[i].lane == player_lane && obstacles[i].row >= PLAYER_ROW {
                    collided = true;
                }

                if obstacles[i].row > PLAYER_ROW {
                    // Obstacle passed off bottom of screen
                    score += 10;
                    obstacles.swap_remove(i);
                } else {
                    // Draw new position
                    draw_obstacle(&obstacles[i], obstacle_color);
                    i += 1;
                }
            }

            // Redraw player car
            draw_player(player_lane, player_color);
            draw_header(score, high_score);

            if collided {
                return score;
            }
        }

        // Halting CPU until next interrupt
        unsafe { core::arch::asm!("hlt") };
    }
}

fn draw_player(lane: usize, color: ColorCode) {
    let col = LANE_COLS[lane];
    let vga = VGA.lock();
    vga.write_string_at(col - 2, PLAYER_ROW - 1, " /==\\ ", color);
    vga.write_string_at(col - 2, PLAYER_ROW,     "|####|", color);
}

fn erase_player(lane: usize, color: ColorCode) {
    let col = LANE_COLS[lane];
    let vga = VGA.lock();
    vga.write_string_at(col - 2, PLAYER_ROW - 1, "      ", color);
    vga.write_string_at(col - 2, PLAYER_ROW,     "      ", color);
}

fn draw_obstacle(obs: &Obstacle, color: ColorCode) {
    let col = LANE_COLS[obs.lane];
    let vga = VGA.lock();
    vga.write_string_at(col - 2, obs.row, "[XXXX]", color);
}

fn erase_obstacle(obs: &Obstacle, color: ColorCode) {
    let col = LANE_COLS[obs.lane];
    let vga = VGA.lock();
    vga.write_string_at(col - 2, obs.row, "      ", color);
}

fn draw_road() {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let border_color = ColorCode::new(Color::White, Color::DarkGray);
    for row in 1..=22 {
        vga.write_string_at(ROAD_LEFT - 1, row, "||", border_color);
        vga.write_string_at(ROAD_RIGHT, row, "||", border_color);
    }
}

fn animate_road(tick: u32) {
    let divider_color = ColorCode::new(Color::Yellow, Color::Black);
    let clear_color = ColorCode::new(Color::Black, Color::Black);
    let vga = VGA.lock();

    for row in 2..22 {
        let is_dash = ((row as u32) + tick) % 3 == 0;
        let char_to_draw = if is_dash { b'|' } else { b' ' };
        let color = if is_dash { divider_color } else { clear_color };
        vga.write_char_at(35, row, char_to_draw, color);
        vga.write_char_at(45, row, char_to_draw, color);
    }
}

fn draw_header(score: u32, high: u32) {
    let header_color = ColorCode::new(Color::Yellow, Color::Blue);
    let s = format!(" CAR DODGE HIGHWAY -- Score: {:<5} | High: {:<5} | Left/Right: Steer | Esc: Exit ", score, high);
    let vga = VGA.lock();
    vga.write_string_at(0, 0, &s, header_color);
    for col in s.len()..BUFFER_WIDTH {
        vga.write_char_at(col, 0, b' ', header_color);
    }
}

fn show_game_over(score: u32, high: u32) -> bool {
    let box_left = 22;
    let box_top = 8;
    let box_right = 58;
    let box_bottom = 15;

    let box_color = ColorCode::new(Color::White, Color::Red);
    let text_color = ColorCode::new(Color::Yellow, Color::Red);

    let vga = VGA.lock();
    for y in box_top..=box_bottom {
        for x in box_left..=box_right {
            vga.write_char_at(x, y, b' ', box_color);
        }
    }

    vga.write_string_at(box_left + 12, box_top + 1, "CAR CRASHED!", ColorCode::new(Color::White, Color::Red));
    let score_str = format!("Final Score: {:>5}", score);
    vga.write_string_at(box_left + 10, box_top + 3, &score_str, text_color);
    let high_str = format!("High Score:  {:>5}", high);
    vga.write_string_at(box_left + 10, box_top + 4, &high_str, text_color);

    vga.write_string_at(box_left + 4, box_top + 6, "[Enter] Race Again  |  [Esc] Menu", ColorCode::new(Color::LightCyan, Color::Red));
    drop(vga);

    loop {
        let key = crate::keyboard::get_key();
        match key {
            Key::Enter => return true,
            Key::Esc => return false,
            _ => {}
        }
    }
}
