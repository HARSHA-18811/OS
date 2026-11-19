//! Snake Arcade Game for RustOS
//!
//! Classic snake game with growing body, collision detection,
//! dynamic scoring, and random food generation.

use alloc::format;
use alloc::vec::Vec;
use super::Rng;
use crate::keyboard::{try_get_key, Key};
use crate::pit;
use crate::rtc;
use crate::statusbar;
use crate::vga::{Color, ColorCode, VGA, BUFFER_WIDTH};

const ARENA_LEFT: usize = 2;
const ARENA_RIGHT: usize = 77;
const ARENA_TOP: usize = 2;
const ARENA_BOTTOM: usize = 22;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn play() {
    let mut high_score = 0;

    loop {
        let final_score = run_game(high_score);
        if final_score > high_score {
            high_score = final_score;
        }

        // Show Game Over and prompt
        let restart = show_game_over(final_score, high_score);
        if !restart {
            break;
        }
    }
}

fn run_game(high_score: u32) -> u32 {
    let mut rng = Rng::new(pit::get_ticks().wrapping_add(rtc::read_datetime().second as u32));

    let mut snake: Vec<(usize, usize)> = Vec::new();
    let start_x = (ARENA_LEFT + ARENA_RIGHT) / 2;
    let start_y = (ARENA_TOP + ARENA_BOTTOM) / 2;
    snake.push((start_x, start_y));
    snake.push((start_x - 1, start_y));
    snake.push((start_x - 2, start_y));

    let mut dir = Direction::Right;
    let mut score = 0;
    let mut food = spawn_food(&snake, &mut rng);

    draw_arena();
    draw_header(score, high_score);
    statusbar::draw("Snake");

    let head_color = ColorCode::new(Color::Yellow, Color::Black);
    let body_color = ColorCode::new(Color::LightGreen, Color::Black);
    let food_color = ColorCode::new(Color::LightRed, Color::Black);
    let clear_color = ColorCode::new(Color::White, Color::Black);

    // Initial draw
    let vga = VGA.lock();
    vga.write_char_at(food.0, food.1, b'*', food_color);
    vga.write_char_at(snake[0].0, snake[0].1, b'O', head_color);
    for pt in &snake[1..] {
        vga.write_char_at(pt.0, pt.1, b'o', body_color);
    }
    drop(vga);

    let mut last_tick = pit::get_ticks();
    let move_delay_ticks = 8; // ~80ms per step

    loop {
        // Handle input non-blocking
        while let Some(key) = try_get_key() {
            match key {
                Key::Up if dir != Direction::Down => dir = Direction::Up,
                Key::Down if dir != Direction::Up => dir = Direction::Down,
                Key::Left if dir != Direction::Right => dir = Direction::Left,
                Key::Right if dir != Direction::Left => dir = Direction::Right,
                Key::Esc => return score,
                _ => {}
            }
        }

        let now = pit::get_ticks();
        if now.wrapping_sub(last_tick) >= move_delay_ticks {
            last_tick = now;

            // Compute new head
            let (hx, hy) = snake[0];
            let new_head = match dir {
                Direction::Up => (hx, hy.saturating_sub(1)),
                Direction::Down => (hx, hy + 1),
                Direction::Left => (hx.saturating_sub(1), hy),
                Direction::Right => (hx + 1, hy),
            };

            // Check wall collision
            if new_head.0 <= ARENA_LEFT || new_head.0 >= ARENA_RIGHT || new_head.1 <= ARENA_TOP || new_head.1 >= ARENA_BOTTOM {
                return score;
            }

            // Check self collision
            for pt in &snake {
                if new_head == *pt {
                    return score;
                }
            }

            // Check food collision
            let ate_food = new_head == food;
            snake.insert(0, new_head);

            let vga = VGA.lock();
            // Draw new head and update old head to body
            vga.write_char_at(new_head.0, new_head.1, b'O', head_color);
            if snake.len() > 1 {
                vga.write_char_at(snake[1].0, snake[1].1, b'o', body_color);
            }

            if ate_food {
                score += 10;
                food = spawn_food(&snake, &mut rng);
                vga.write_char_at(food.0, food.1, b'*', food_color);
                drop(vga);
                draw_header(score, high_score);
            } else {
                // Remove tail
                if let Some(tail) = snake.pop() {
                    vga.write_char_at(tail.0, tail.1, b' ', clear_color);
                }
                drop(vga);
            }
        }

        // Halting CPU until next interrupt (e.g. PIT or Keyboard)
        unsafe { core::arch::asm!("hlt") };
    }
}

fn spawn_food(snake: &[(usize, usize)], rng: &mut Rng) -> (usize, usize) {
    loop {
        let x = rng.range((ARENA_LEFT + 1) as u32, (ARENA_RIGHT - 1) as u32) as usize;
        let y = rng.range((ARENA_TOP + 1) as u32, (ARENA_BOTTOM - 1) as u32) as usize;
        if !snake.iter().any(|pt| pt.0 == x && pt.1 == y) {
            return (x, y);
        }
    }
}

fn draw_arena() {
    let mut vga = VGA.lock();
    vga.clear_screen();

    let border_color = ColorCode::new(Color::Cyan, Color::Black);

    // Horizontal borders
    for x in ARENA_LEFT..=ARENA_RIGHT {
        vga.write_char_at(x, ARENA_TOP, b'#', border_color);
        vga.write_char_at(x, ARENA_BOTTOM, b'#', border_color);
    }
    // Vertical borders
    for y in ARENA_TOP..=ARENA_BOTTOM {
        vga.write_char_at(ARENA_LEFT, y, b'#', border_color);
        vga.write_char_at(ARENA_RIGHT, y, b'#', border_color);
    }
}

fn draw_header(score: u32, high: u32) {
    let header_color = ColorCode::new(Color::Yellow, Color::Blue);
    let s = format!(" SNAKE ARCADE -- Score: {:<4} | High Score: {:<4} | Controls: Arrows | Esc: Exit ", score, high);
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

    vga.write_string_at(box_left + 12, box_top + 1, "GAME OVER!", ColorCode::new(Color::White, Color::Red));
    let score_str = format!("Final Score: {:>4}", score);
    vga.write_string_at(box_left + 10, box_top + 3, &score_str, text_color);
    let high_str = format!("High Score:  {:>4}", high);
    vga.write_string_at(box_left + 10, box_top + 4, &high_str, text_color);

    vga.write_string_at(box_left + 4, box_top + 6, "[Enter] Play Again  |  [Esc] Menu", ColorCode::new(Color::LightCyan, Color::Red));
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
