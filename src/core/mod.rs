/*========================================================
MODULE INCLUSIONS
========================================================*/
pub mod file_locations;
pub mod game_functions; // include things like at bat functinos, defense rolls, etc. // default file locations for teams/players/ballparks

use rand::Rng;

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
// TODO: this should probably be u32 right???
/// returns a random integer between 1 and [side], used to simulate dice rolls with threed_rng()
pub fn roll(side: i32) -> i32 {
    if side == 1 {
        return 1;
    }
    rand::thread_rng().gen_range(1..side)
}

// TODO: function to wrap indexing batting order (0-8)
/// wraps addition/subtraction to values between 0-8 for indexing batting order in Team structs
pub fn bo_wrap(original: u32, adjust: u32, subtract: bool) -> usize {
    // no need to clamp initial value, should always be between 0-8 unless screwed up elsewhere
    if subtract && original < adjust {
        (9 + original - adjust) as usize
    } else if subtract && original >= adjust {
        (original - adjust) as usize
    } else {
        let sum = original + adjust;
        (sum % 9) as usize
    }
}
