/*========================================================
MODULE INCLUSIONS
========================================================*/
pub mod file_locations;
pub mod game_functions; // include things like at bat functinos, defense rolls, etc. // default file locations for teams/players/ballparks

use rand::Rng;

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn roll(side: i32) -> i32 {
    let roll = rand::thread_rng().gen_range(1..side);
    roll
}
