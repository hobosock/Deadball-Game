/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/
pub mod characters; // includes player, team, and era code
pub mod core; // includes core functions like dice rolling

use crate::core::*;
use crate::core::gameFunctions;

/*========================================================
ENUM DEFINITIONS
========================================================*/

/*========================================================
STRUCT DEFINITIONS
========================================================*/

/*========================================================
TESTS
========================================================*/
#[cfg(test)]
mod tests {
    //use crate::core::gameFunctions::atBatResults;

    use crate::{core::gameFunctions::*, characters::players::*};

    use super::*;

    #[test]
    fn dice_roll_check() {
        // kind of hard to test that the dice rolls are random, but this should at least test that they are within expected range
        let side = 100;
        let test_roll = roll(side);
        assert!(test_roll <= side && test_roll >= 1, "dice roll is outside of expected bounds");
    }

    #[test]
    fn at_bat_hit_check() {
        let onBaseTarget = 40;
        let batTarget = 32;
        let pitchResult = 20;
        let atBatResult = gameFunctions::atBat(batTarget, onBaseTarget, pitchResult);
        assert!(matches!(gameFunctions::atBatResults::hit, atBatResult));
    }

    #[test]
    fn at_bat_out_check() {
        let onBaseTarget = 40;
        let batTarget = 32;
        let pitchResult = 78;
        let atBatResult = gameFunctions::atBat(batTarget, onBaseTarget, pitchResult);
        assert!(matches!(gameFunctions::atBatResults::out, atBatResult));
    }

    #[test]
    fn load_roster_file() {
        let playerFilePath = "/home/seth/Documents/rust_projects/deadball/src/testfiles/sample_player.dbp".to_string();
        let testPlayer = loadPlayer(playerFilePath);
        assert!(matches!("Seth".to_string(), testPlayer.firstName));
    }
}