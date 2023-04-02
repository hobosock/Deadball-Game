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
    use crate::{characters::players::*, characters::teams::*, core::game_functions::*};

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

    #[test]
    fn test_load_team() {
        let team_file_path =
            "/home/seth/Deadball-Game/src/testfiles/detroit_steam_hammers.dbt".to_string();
        let contents = fs::read_to_string(team_file_path).unwrap();

        let test_team = load_team(contents);
        let test_name = test_team.name;
        let test_ballpark = test_team.ballpark;
        let test_manager = test_team.manager;
        let test_logo = test_team.logo;
        let test_era = test_team.era;
        let test_location = test_team.location;
        let test_mascot = test_team.mascot;
        let test_priority = test_team.priority;
        let test_makeup = test_team.makeup;
        let test_years = test_team.years;
        let test_championship = test_team.championship;
        let test_fanbase = test_team.fanbase;
        let test_manager_position = test_team.manager_position;
        let test_manager_league = test_team.manager_league;
        let test_retired = test_team.retired;
        let test_personality = test_team.personality;
        let test_daring = test_team.daring;
        let test_motto = test_team.motto;
        let test_owner_background = test_team.owner_background;
        let test_owner_personality = test_team.owner_personality;
        let test_roster = test_team.roster;

        assert!(matches!("Detroit Steam Hammers".to_string(), test_name));
        assert!(matches!(
            "/home/seth/Deadball-Game/src/testfiles/railyard.dbb".to_string(),
            test_ballpark
        ));
        assert!(matches!("none".to_string(), test_logo));
        assert!(matches!(Era::Modern, test_era));
        assert!(matches!(Location::Metropolis, test_location));
        assert!(matches!("Train".to_string(), test_mascot));
        assert!(matches!(Priority::StartingPitching, test_priority));
        assert!(matches!(Makeup::MostlyProspects, test_makeup));
        assert_eq!(11, test_years);
        assert_eq!(7, test_championship);
        assert!(matches!(Fanbase::Loyal, test_fanbase));
        assert!(matches!("Fastball Mike".to_string(), test_manager));
        assert!(matches!(Position::Pitcher, test_manager_position));
        assert!(matches!(ManagerLeague::Major, test_manager_league));
        assert_eq!(22, test_retired);
        assert!(matches!("Sincere".to_string(), test_personality));
        assert_eq!(12, test_daring);
        assert!(matches!(
            "Score more runs than the other guy.".to_string(),
            test_motto
        ));
        assert!(matches!(
            "Venture Capitalist".to_string(),
            test_owner_background
        ));
        assert!(matches!("Boastful".to_string(), test_owner_personality));
        assert!(matches!(
            vec![
                "/home/seth/Deadball-Game/src/testfiles/sample_player.dbp".to_string(),
                "/home/seth/Deadball-Game/src/testfiles/sample2.dbp".to_string(),
            ],
            test_roster
        ));
    }

    #[test]
    fn test_write_team() {
        let test_team = Team {
            name: "Test Team".to_string(),
            ballpark: "Test Ballpark".to_string(),
            manager: "Test Manager".to_string(),
            logo: "Test Logo".to_string(),
            era: Era::Modern,
            location: Location::Metropolis,
            mascot: "Test Mascot".to_string(),
            priority: Priority::Power,
            makeup: Makeup::Balanced,
            years: 10i32,
            championship: 10i32,
            fanbase: Fanbase::Loyal,
            manager_position: Position::Pitcher,
            manager_league: ManagerLeague::Major,
            retired: 10i32,
            personality: "Test Personality".to_string(),
            daring: 10i32,
            motto: "Test Motto".to_string(),
            owner_background: "Test Background".to_string(),
            owner_personality: "Test Personality".to_string(),
            roster: vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string(),
            ],
        };

        let filename = "/home/seth/Deadball-Game/src/testfiles/write_team_test.dbt";
        let write_result = write_team(test_team, filename);

        let contents = fs::read_to_string(filename).unwrap();
        let read_team = load_team(contents);
        let test_name = read_team.name;
        let test_ballpark = read_team.ballpark;
        let test_manager = read_team.manager;
        let test_logo = read_team.logo;
        let test_era = read_team.era;
        let test_location = read_team.location;
        let test_mascot = read_team.mascot;
        let test_priority = read_team.priority;
        let test_makeup = read_team.makeup;
        let test_years = read_team.years;
        let test_championship = read_team.championship;
        let test_fanbase = read_team.fanbase;
        let test_manager_position = read_team.manager_position;
        let test_manager_league = read_team.manager_league;
        let test_retired = read_team.retired;
        let test_personality = read_team.personality;
        let test_daring = read_team.daring;
        let test_motto = read_team.motto;
        let test_owner_background = read_team.owner_background;
        let test_owner_personality = read_team.owner_personality;

        assert!(matches!("Test Team".to_string(), test_name));
        assert!(matches!("Test Ballpark".to_string(), test_ballpark));
        assert!(matches!("Test Manager".to_string(), test_ballpark));
        assert!(matches!("Test Logo".to_string(), test_manager));
        assert!(matches!(Era::Modern, test_era));
        assert!(matches!(Location::Metropolis, test_location));
        assert!(matches!("Test Mascot".to_string(), test_mascot));
        assert!(matches!(Priority::Power, test_priority));
        assert!(matches!(Makeup::Balanced, test_makeup));
        assert_eq!(10i32, test_years);
        assert_eq!(10i32, test_championship);
        assert!(matches!(Fanbase::Loyal, test_fanbase));
        assert!(matches!(Position::Pitcher, test_manager_position));
        assert!(matches!(ManagerLeague::Major, test_manager_league));
        assert_eq!(10i32, test_retired);
        assert!(matches!("Test Personality".to_string(), test_personality));
        assert_eq!(10i32, test_daring);
        assert!(matches!("Test Motto".to_string(), test_motto));
        assert!(matches!(
            "Test Background".to_string(),
            test_owner_background
        ));
        assert!(matches!(
            "Test Personality".to_string(),
            test_owner_personality
        ));
        assert!(matches!(
            vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string()
            ],
            test_roster
        ));
    }
}
