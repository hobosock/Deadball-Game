/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
//#[allow(non_camel_case_types)]
//#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/
pub mod game_functions; // include things like at bat functinos, defense rolls, etc.

use rand::Rng;

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn roll(side: i32) -> i32 {
    let roll = rand::thread_rng().gen_range(1..side);
    roll
}
