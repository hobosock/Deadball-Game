use rand::Rng;

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
        } else {
            roll = rand::thread_rng().gen_range(1..side);
        }
    }

    return roll;
}
