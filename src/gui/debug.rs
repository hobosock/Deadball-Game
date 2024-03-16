use rand::Rng;

use crate::core::roll;

#[derive(Clone)]
pub struct DebugConfig {
    pub mode: bool,
    pub rolls: Vec<i32>,
    pub roll_index: usize,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            mode: false,
            rolls: vec![0],
            roll_index: 0,
        }
    }
}

pub fn debug_roll(config: &mut DebugConfig, side: i32) -> i32 {
    let roll: i32;
    if side == 1 {
        roll = 1;
    } else {
        // make sure enough rolls were specified, if not rng like normal
        if config.rolls.len() > config.roll_index {
            roll = config.rolls[config.roll_index];
            config.roll_index += 1; // increment index for next roll
        } else {
            roll = rand::thread_rng().gen_range(1..side);
        }
    }

    roll
}

pub fn combined_roll(debug: &mut DebugConfig, side: i32) -> i32 {
    if debug.mode {
        debug_roll(debug, side)
    } else {
        roll(side)
    }
}
