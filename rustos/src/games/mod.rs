//! RustOS Games Suite
//!
//! Provides three playable games accessible from the Desktop Menu:
//! 1. Snake: Classic arcade snake with growing body, score, and random food.
//! 2. Snake and Ladders: 10x10 board game (Human vs Computer AI) with snakes & ladders.
//! 3. Car Dodge: 3-lane reflex highway driving game with increasing obstacle speed.

pub mod snake;
pub mod snakes_ladders;
pub mod car_dodge;

/// Pseudo-random number generator (Xorshift32)
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Rng {
            state: if seed == 0 { 0xA5A5A5A5 } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    /// Returns a pseudo-random integer in the inclusive range [min, max].
    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        min + (self.next_u32() % (max - min + 1))
    }
}
