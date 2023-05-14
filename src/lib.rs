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
        let player_file_path = "src/testfiles/sample_player.dbp".to_string();
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
        let temp = test_name.clone();
        assert!(matches!("Seth".to_string(), temp));
        assert!(matches!("Loveall".to_string(), test_last));
        assert!(matches!(Position::Shortstop, test_pos));
        assert!(matches!(Handedness::Right, test_hand));
        assert_eq!(32, test_bt);
        assert_eq!(40, test_obt);
        assert_eq!(-8, test_pd);
        assert!(matches!(vec![Traits::ContactHitter], test_trait));
        assert!(matches!(vec![InjuryLocation::None], test_loc));
        assert!(matches!(vec![InjurySeverity::Uninjured], test_sev));

        let player2_file_path = "src/testfiles/sample2.dbp".to_string();
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
        let filename = "src/testfiles/write_test.dbp";
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
        let team_file_path = "src/testfiles/detroit_steam_hammers.dbt".to_string();
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
            "src/testfiles/railyard.dbb".to_string(),
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
                "src/testfiles/sample_player.dbp".to_string(),
                "src/testfiles/sample2.dbp".to_string(),
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
            bench: vec!["test4".to_string()],
            pitcher: vec!["test5".to_string()],
            bullpen: vec!["test6".to_string()],
        };

        let filename = "src/testfiles/write_team_test.dbt";
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
        let test_roster = read_team.roster;
        let test_bench = read_team.bench;
        let test_pitcher = read_team.pitcher;
        let test_bullpen = read_team.bullpen;

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
        assert!(matches!(vec!["test4".to_string()], test_bench));
        assert!(matches!(vec!["test5".to_string()], test_pitcher));
        let temp = &test_bullpen[0].trim();
        assert!(matches!("test6".to_string(), temp));
    }

    #[test]
    fn test_load_park() {
        let park_file_path = "src/testfiles/railyard.dbb".to_string();
        let contents = fs::read_to_string(park_file_path).unwrap();

        let test_park_modern = load_park_modern(contents);
        let modern_name = test_park_modern.name;
        let modern_location = test_park_modern.location;
        let modern_type = test_park_modern.park_type;
        let modern_capacity = test_park_modern.capacity;
        let modern_turf = test_park_modern.turf;
        let modern_roof = test_park_modern.roof;
        let modern_condition = test_park_modern.condition;
        let modern_quirks = test_park_modern.quirks;

        assert!(matches!("The Railyard".to_string(), modern_name));
        assert!(matches!(Location::Metropolis, modern_location));
        assert!(matches!(StadiumTypeModern::Retro, modern_type));
        assert_eq!(43000i32, modern_capacity);
        assert!(matches!(Turf::Good, modern_turf));
        assert!(matches!(Roof::PermanentRoof, modern_roof));
        assert!(matches!(Condition::Decrepit, modern_condition));
        assert!(matches!(vec![Quirks::ExpansiveOutfield], modern_quirks));

        let ancient_file_path = "src/testfiles/mayfair_park.dbb".to_string();
        let ancient_contents = fs::read_to_string(ancient_file_path).unwrap();

        let test_park_ancient = load_park_ancient(ancient_contents);
        let ancient_name = test_park_ancient.name;
        let ancient_location = test_park_ancient.location;
        let ancient_type = test_park_ancient.park_type;
        let ancient_capacity = test_park_ancient.capacity;
        let ancient_condition = test_park_ancient.condition;
        let ancient_quirks = test_park_ancient.quirks;

        assert!(matches!("Mayfair Park".to_string(), ancient_name));
        assert!(matches!(Location::SmallCity, ancient_location));
        assert!(matches!(
            StadiumTypeAncient::WoodFramePavilion,
            ancient_type
        ));
        assert_eq!(25000i32, ancient_capacity);
        assert!(matches!(Condition::WellWorn, ancient_condition));
        assert!(matches!(
            vec![Quirks::ShortRight, Quirks::Beautiful],
            ancient_quirks
        ));
    }

    #[test]
    fn test_write_park() {
        //
    }

    #[test]
    fn test_create_modern_game() {
        let mut team1 = Team {
            name: "Test 1".to_string(),
            ballpark: "test".to_string(),
            manager: "test".to_string(),
            logo: "test".to_string(),
            era: Era::Modern,
            location: Location::SmallTown,
            mascot: "test".to_string(),
            priority: Priority::Power,
            makeup: Makeup::Balanced,
            years: 1i32,
            championship: 1i32,
            fanbase: Fanbase::Loyal,
            manager_position: Position::Shortstop,
            manager_league: ManagerLeague::Major,
            retired: 1i32,
            personality: "test".to_string(),
            daring: 1i32,
            motto: "test".to_string(),
            owner_background: "test".to_string(),
            owner_personality: "test".to_string(),
            roster: vec!["test".to_string(), "test".to_string(), "test".to_string()],
            bench: vec!["test".to_string()],
            pitcher: vec!["test".to_string()],
            bullpen: vec!["test".to_string()],
        };

        let mut team2 = Team {
            name: "Test 2".to_string(),
            ballpark: "test".to_string(),
            manager: "test".to_string(),
            logo: "test".to_string(),
            era: Era::Ancient,
            location: Location::SmallTown,
            mascot: "test".to_string(),
            priority: Priority::Power,
            makeup: Makeup::Balanced,
            years: 1i32,
            championship: 1i32,
            fanbase: Fanbase::Loyal,
            manager_position: Position::Shortstop,
            manager_league: ManagerLeague::Major,
            retired: 1i32,
            personality: "test".to_string(),
            daring: 1i32,
            motto: "test".to_string(),
            owner_background: "test".to_string(),
            owner_personality: "test".to_string(),
            roster: vec![
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            ],
            bench: vec!["test".to_string()],
            pitcher: vec!["test".to_string()],
            bullpen: vec!["test".to_string()],
        };

        let ballpark = BallparkModern {
            name: "test".to_string(),
            location: Location::SmallTown,
            park_type: StadiumTypeModern::Retro,
            capacity: 1i32,
            turf: Turf::Good,
            roof: Roof::None,
            condition: Condition::WellWorn,
            quirks: vec![Quirks::OddLeft],
        };

        let test_result = create_modern_game(&team1, &team2, &ballpark);
        assert!(matches!(
            Err::<GameModern, core::game_functions::TeamError>(TeamError {
                message: "Home team does not have a complete roster".to_string(),
                team: "Test 1".to_string()
            }),
            test_result
        ));

        team1.roster = vec![
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
        ];

        let test_result2 = create_modern_game(&team1, &team2, &ballpark);
        assert!(matches!(
            Err::<GameModern, core::game_functions::TeamError>(TeamError {
                message: "Away team is not for the modern era".to_string(),
                team: "Test 2".to_string()
            }),
            test_result2
        ));

        team2.era = Era::Modern;

        /*
        let test_result3 = create_modern_game(&team1, &team2, &ballpark).unwrap();
        assert!(matches!(
            GameModern {
                home: &team1,
                away: &team2,
                ballpark: &ballpark,
                home_active: _,
                away_active: _,
            },
            test_result3
        ));
        */
    }

    #[test]
    fn test_load_roster() {
        let filename = "src/testfiles/detroit_steam_hammers.dbt";
        let contents = fs::read_to_string(filename).unwrap();
        let read_team = load_team(contents);
        let test_roster = &read_team.roster;
        let test_bench = &read_team.bench;
        let test_pitcher = &read_team.pitcher;
        let test_bullpen = &read_team.bullpen;
    }

    // critical hit function test
    #[test]
    fn test_crit_hit() {
        let r1 = crit_hit(&1);
        assert_eq!(r1, 18);

        let r2 = crit_hit(&8);
        assert_eq!(r2, 18);

        let r3 = crit_hit(&5);
        assert_eq!(r3, 15);

        let r4 = crit_hit(&3);
        assert_eq!(r4, 17);

        let r5 = crit_hit(&4);
        assert_eq!(r5, 16);

        let r6 = crit_hit(&16);
        assert_eq!(r6, 19);

        let r7 = crit_hit(&19);
        assert_eq!(r7, 19);
    }

    // runnerson function test
    #[test]
    fn test_runnerson() {
        // create pitcher to fill in game state for test
        let test_player = Player {
            first_name: "".to_string(),
            last_name: "".to_string(),
            nickname: "".to_string(),
            position: Position::Pitcher,
            handedness: Handedness::Right,
            batter_target: 12,
            on_base_target: 18,
            pitch_die: 4,
            traits: vec![Traits::None],
            injury_location: vec![InjuryLocation::None],
            injury_severity: vec![InjurySeverity::Uninjured],
        };
        let mut state = GameState {
            status: GameStatus::Ongoing,
            inning: 1,
            inning_half: InningTB::Bottom,
            outs: Outs::Two,
            runners: RunnersOn::Runner000,
            batting_team1: 1,
            batting_team2: 1,
            current_pitcher_team1: &test_player,
            current_pitcher_team2: &test_player,
            pitched_team1: 1,
            pitched_team2: 1,
            runs_team1: 0,
            runs_team2: 0,
            hits_team1: 0,
            hits_team2: 0,
            errors_team1: 0,
            errors_team2: 0,
        };

        let r1 = runnerson(&state);
        assert_eq!(r1, 0);

        state.runners = RunnersOn::Runner100;
        let r2 = runnerson(&state);
        assert_eq!(r2, 1);

        state.runners = RunnersOn::Runner010;
        let r3 = runnerson(&state);
        assert_eq!(r3, 1);

        state.runners = RunnersOn::Runner001;
        let r4 = runnerson(&state);
        assert_eq!(r4, 1);

        state.runners = RunnersOn::Runner110;
        let r5 = runnerson(&state);
        assert_eq!(r5, 2);

        state.runners = RunnersOn::Runner101;
        let r6 = runnerson(&state);
        assert_eq!(r6, 2);

        state.runners = RunnersOn::Runner011;
        let r7 = runnerson(&state);
        assert_eq!(r7, 2);

        state.runners = RunnersOn::Runner111;
        let r8 = runnerson(&state);
        assert_eq!(r8, 3);
    }

    // runners_advance test function
    #[test]
    fn test_runners_advance() {
        // create test structures
        let test_player = Player {
            first_name: "".to_string(),
            last_name: "".to_string(),
            nickname: "".to_string(),
            position: Position::Pitcher,
            handedness: Handedness::Right,
            batter_target: 12,
            on_base_target: 18,
            pitch_die: 4,
            traits: vec![Traits::None],
            injury_location: vec![InjuryLocation::None],
            injury_severity: vec![InjurySeverity::Uninjured],
        };
        let mut state = GameState {
            status: GameStatus::Ongoing,
            inning: 1,
            inning_half: InningTB::Bottom,
            outs: Outs::Two,
            runners: RunnersOn::Runner100,
            batting_team1: 1,
            batting_team2: 1,
            current_pitcher_team1: &test_player,
            current_pitcher_team2: &test_player,
            pitched_team1: 1,
            pitched_team2: 1,
            runs_team1: 0,
            runs_team2: 0,
            hits_team1: 0,
            hits_team2: 0,
            errors_team1: 0,
            errors_team2: 0,
        };

        state = runners_advance(state, &1);
        assert!(matches!(state.runners, RunnersOn::Runner010));

        state = runners_advance(state, &1);
        assert!(matches!(state.runners, RunnersOn::Runner001));

        state = runners_advance(state, &1);
        assert!(matches!(state.runners, RunnersOn::Runner000));
        assert_eq!(state.runs_team1, 1);

        state.runners = RunnersOn::Runner100;
        state = runners_advance(state, &2);
        assert!(matches!(state.runners, RunnersOn::Runner001));

        state = runners_advance(state, &2);
        assert!(matches!(state.runners, RunnersOn::Runner000));
        assert_eq!(state.runs_team1, 2);

        state.runners = RunnersOn::Runner011;
        state = runners_advance(state, &2);
        assert!(matches!(state.runners, RunnersOn::Runner000));
        assert_eq!(state.runs_team1, 4);

        state.runners = RunnersOn::Runner110;
        state = runners_advance(state, &3);
        assert!(matches!(state.runners, RunnersOn::Runner000));
        assert_eq!(state.runs_team1, 6);
    }
}
