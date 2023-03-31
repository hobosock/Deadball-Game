/*========================================================
MODULE INCLUSIONS
========================================================*/
pub mod characters; // includes player, team, and era code
pub mod core; // includes core functions like dice rolling

#[allow(unused_imports)]
use crate::core::game_functions;
#[allow(unused_imports)]
use crate::core::*;

/*========================================================
ENUM DEFINITIONS
========================================================*/

/*========================================================
STRUCT DEFINITIONS
========================================================*/

/*========================================================
TESTS
========================================================*/
#[allow(unused_imports)]
#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use std::fs;

    //use crate::core::gameFunctions::atBatResults;
    use crate::{characters::players::*, core::game_functions::*};

    use super::*;

    #[test]
    fn dice_roll_check() {
        // kind of hard to test that the dice rolls are random, but this should at least test that they are within expected range
        let side = 100;
        let test_roll = roll(side);
        assert!(
            test_roll <= side && test_roll >= 1,
            "dice roll is outside of expected bounds"
        );
    }

    #[test]
    fn at_bat_hit_check() {
        let on_base_target = 40;
        let bat_target = 32;
        let pitch_result = 20;
        let at_bat_result = game_functions::at_bat(bat_target, on_base_target, pitch_result);
        assert!(matches!(game_functions::AtBatResults::Hit, at_bat_result));
    }

    #[test]
    fn at_bat_out_check() {
        let on_base_target = 40;
        let bat_target = 32;
        let pitch_result = 78;
        let at_bat_result = game_functions::at_bat(bat_target, on_base_target, pitch_result);
        assert!(matches!(game_functions::AtBatResults::Out, at_bat_result));
    }

    #[test]
    fn load_player_file() {
        let player_file_path =
            "/home/seth/Deadball-Game/src/testfiles/sample_player.dbp".to_string();
        let contents = fs::read_to_string(player_file_path).unwrap();
        let test_player = load_player(contents);
        let test_name = test_player.first_name;
        let test_last = test_player.last_name;
        let test_pos = test_player.position;
        let test_hand = test_player.handedness;
        let test_bt = test_player.batter_target;
        let test_obt = test_player.on_base_target;
        let test_pd = test_player.pitch_die;
        let test_trait = test_player.traits;
        let test_loc = test_player.injury_location;
        let test_sev = test_player.injury_severity;

        assert!(matches!("Seth".to_string(), test_name));
        assert!(matches!("Loveall".to_string(), test_last));
        assert!(matches!(Position::Shortstop, test_pos));
        assert!(matches!(Handedness::Right, test_hand));
        assert_eq!(32, test_bt);
        assert_eq!(40, test_obt);
        assert_eq!(-8, test_pd);
        assert!(matches!(vec![Traits::ContactHitter], test_trait));
        assert!(matches!(vec![InjuryLocation::None], test_loc));
        assert!(matches!(vec![InjurySeverity::Uninjured], test_sev));

        let player2_file_path = "/home/seth/Deadball-Game/src/testfiles/sample2.dbp".to_string();
        let contents2 = fs::read_to_string(player2_file_path).unwrap();
        let test_player2 = load_player(contents2);
        let test_nick2 = test_player2.nickname;
        let test_loc2 = test_player2.injury_location;
        let test_sev2 = test_player2.injury_severity;
        assert!(matches!("Bruh".to_string(), test_nick2));
        assert!(matches!(vec![InjuryLocation::Shoulder], test_loc2));
        assert!(matches!(vec![InjurySeverity::Minor], test_sev2));
    }

    #[test]
    fn write_player_file() {
        let test_player = Player {
            first_name: "Seth".to_string(),
            last_name: "Loveall".to_string(),
            nickname: "Seth Loveall".to_string(),
            position: Position::Shortstop,
            handedness: Handedness::Right,
            batter_target: 32,
            on_base_target: 40,
            pitch_die: -8,
            traits: vec![
                Traits::ContactHitter,
                Traits::PowerHitter,
                Traits::GreatDefender,
            ],
            injury_location: vec![
                InjuryLocation::Shoulder,
                InjuryLocation::Wrist,
                InjuryLocation::Hamstring,
            ],
            injury_severity: vec![
                InjurySeverity::Uninjured,
                InjurySeverity::Minor,
                InjurySeverity::Superficial,
            ],
        };
        let filename = "/home/seth/Deadball-Game/src/testfiles/write_test.dbp";
        let write_result = write_player(test_player, filename);

        let contents = fs::read_to_string(filename).unwrap();
        let test_player = load_player(contents);
        let test_first = test_player.first_name;
        let test_last = test_player.last_name;
        let test_nick = test_player.nickname;
        let test_pos = test_player.position;
        let test_hand = test_player.handedness;
        let test_bt = test_player.batter_target;
        let test_obt = test_player.on_base_target;
        let test_pd = test_player.pitch_die;
        let test_trait = test_player.traits;
        let test_loc = test_player.injury_location;
        let test_sev = test_player.injury_severity;

        assert!(matches!("Seth".to_string(), test_first));
        assert!(matches!("Loveall".to_string(), test_last));
        assert!(matches!("Seth Loveall".to_string(), test_nick));
        assert!(matches!(Position::Shortstop, test_pos));
        assert!(matches!(Handedness::Right, test_hand));
        assert_eq!(32, test_bt);
        assert_eq!(40, test_obt);
        assert_eq!(-8, test_pd);
        assert!(matches!(
            vec![
                Traits::ContactHitter,
                Traits::PowerHitter,
                Traits::GreatDefender
            ],
            test_trait
        ));
        assert!(matches!(
            vec![
                InjuryLocation::Shoulder,
                InjuryLocation::Wrist,
                InjuryLocation::Hamstring
            ],
            test_loc
        ));
        assert!(matches!(
            vec![
                InjurySeverity::Uninjured,
                InjurySeverity::Minor,
                InjurySeverity::Superficial
            ],
            test_sev
        ));
    }
}
