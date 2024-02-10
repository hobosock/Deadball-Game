/*==============================================================================================
 * IMPORTS
 * ===========================================================================================*/
// LOCAL IMPORTS
mod characters;
mod core;
mod gui;
use gui::app::*;

// EXTERNAL IMPORTS
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Deadball",
        options,
        Box::new(|_cc| Box::<DeadballApp>::default()),
    )
    /* SEGMENT OF CODE TO GENERATE TEAMS, DON'T NEED IT EVERY TIME
    // need to load in databases for generating names, etc.
    let mut first_names: Vec<String> = vec![];
    let read_result = load_csv("src/databases/firstname.csv", "\n");
    match read_result {
        Ok(mut a) => first_names.append(&mut a),
        Err(_) => {}
    }

    let mut last_names: Vec<String> = vec![];
    let read_result = load_csv("src/databases/lastname.csv", "\n");
    match read_result {
        Ok(mut a) => last_names.append(&mut a),
        Err(_) => {}
    }

    let mut logos: Vec<String> = vec![];
    let read_result = load_csv("src/databases/logo.csv", "\n");
    match read_result {
        Ok(mut a) => logos.append(&mut a),
        Err(_) => {}
    }

    let mut mascots: Vec<String> = vec![];
    let read_result = load_csv("src/databases/mascot.csv", "\n");
    match read_result {
        Ok(mut a) => mascots.append(&mut a),
        Err(_) => {}
    }

    let mut mottos: Vec<String> = vec![];
    let read_result = load_csv("src/databases/motto.csv", "\n");
    match read_result {
        Ok(mut a) => mottos.append(&mut a),
        Err(_) => {}
    }

    let mut personalities: Vec<String> = vec![];
    let read_result = load_csv("src/databases/personality.csv", "\n");
    match read_result {
        Ok(mut a) => personalities.append(&mut a),
        Err(_) => {}
    }

    let mut backgrounds: Vec<String> = vec![];
    let read_result = load_csv("src/databases/background.csv", "\n");
    match read_result {
        Ok(mut a) => backgrounds.append(&mut a),
        Err(_) => {}
    }

    let mut name1: Vec<String> = vec![];
    let read_result = load_csv("src/databases/park1.csv", "\n");
    match read_result {
        Ok(mut a) => name1.append(&mut a),
        Err(_) => {}
    }

    let mut name2: Vec<String> = vec![];
    let read_result = load_csv("src/databases/park2.csv", "\n");
    match read_result {
        Ok(mut a) => name2.append(&mut a),
        Err(_) => {}
    }

    // quick test, just print out each step of a game and see if it makes sense
    // 1. generate 2 new teams
    let team1 = generate_team(
        Era::Modern,
        8,
        4,
        1,
        5,
        "Red Team",
        &first_names,
        &last_names,
        &logos,
        &mascots,
        &mottos,
        &personalities,
        &backgrounds,
        &name1,
        &name2,
    );
    let _ = write_team(team1, "src/testfiles/game/teams/red_team.dbt");

    let team2 = generate_team(
        Era::Modern,
        8,
        4,
        1,
        5,
        "Blue Team",
        &first_names,
        &last_names,
        &logos,
        &mascots,
        &mottos,
        &personalities,
        &backgrounds,
        &name1,
        &name2,
    );
    let _ = write_team(team2, "src/testfiles/game/teams/blue_team.dbt");
    */

    /*
    let contents1 = fs::read_to_string("src/testfiles/game/teams/red_team.dbt").unwrap();
    let contents2 = fs::read_to_string("src/testfiles/game/teams/blue_team.dbt").unwrap();
    let team1 = load_team(contents1);
    let team2 = load_team(contents2);
    let (mut roster1, mut bench1, mut pitcher1, mut bullpen1) = load_roster(&team1);
    let (mut roster2, mut bench2, mut pitcher2, mut bullpen2) = load_roster(&team2);
    let contents3 = fs::read_to_string(&team1.ballpark).unwrap();
    let ballpark = load_park_modern(contents3);

    let game = create_modern_game(&team1, &team2, &ballpark).unwrap();
    let mut game_state = init_new_game_state(&pitcher1[0], &pitcher2[0]);
    modern_game_flow(&game, game_state);
    */
}

/*========================================================
TESTS
========================================================*/
#[allow(unused_imports)]
#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use std::{fs, vec};

    //use crate::core::gameFunctions::atBatResults;
    use crate::core::{game_functions, roll};
    use crate::gui::debug::{debug_roll, DebugConfig};
    use crate::{
        characters::players::*, characters::teams::*, core::file_locations::*,
        core::game_functions::*,
    };
    use characters::players::{
        Handedness, InjuryLocation, InjurySeverity, Player, Position, Traits,
    };

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
        let write_result = write_player(&test_player, filename);

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

        let test_result = create_modern_game(team1.clone(), team2.clone(), ballpark.clone());
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

        let test_result2 = create_modern_game(team1, team2.clone(), ballpark);
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
            runner1: None,
            runner2: None,
            runner3: None,
            batting_team1: 1,
            batting_team2: 1,
            current_pitcher_team1: test_player.clone(),
            current_pitcher_team2: test_player.clone(),
            pitched_team1: 1,
            pitched_team2: 1,
            runs_team1: 0,
            runs_team2: 0,
            hits_team1: 0,
            hits_team2: 0,
            errors_team1: 0,
            errors_team2: 0,
            game_text: "test".to_string(),
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
            runner1: None,
            runner2: None,
            runner3: None,
            batting_team1: 1,
            batting_team2: 1,
            current_pitcher_team1: test_player.clone(),
            current_pitcher_team2: test_player.clone(),
            pitched_team1: 1,
            pitched_team2: 1,
            runs_team1: 0,
            runs_team2: 0,
            hits_team1: 0,
            hits_team2: 0,
            errors_team1: 0,
            errors_team2: 0,
            game_text: "test".to_string(),
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

    // add_runner test function
    #[test]
    fn test_add_runners() {
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
            runner1: None,
            runner2: None,
            runner3: None,
            batting_team1: 1,
            batting_team2: 1,
            current_pitcher_team1: test_player.clone(),
            current_pitcher_team2: test_player.clone(),
            pitched_team1: 1,
            pitched_team2: 1,
            runs_team1: 0,
            runs_team2: 0,
            hits_team1: 0,
            hits_team2: 0,
            errors_team1: 0,
            errors_team2: 0,
            game_text: "test".to_string(),
        };

        let player1 = Player {
            first_name: "Seth".to_string(),
            nickname: "".to_string(),
            last_name: "Loveall".to_string(),
            position: Position::Firstbase,
            handedness: Handedness::Right,
            batter_target: 30,
            on_base_target: 30,
            pitch_die: -12,
            traits: vec![Traits::GreatDefender],
            injury_location: vec![InjuryLocation::None],
            injury_severity: vec![],
        };
        state.runners = RunnersOn::Runner011;
        state = add_runner(state, &1, player1.clone());
        assert!(matches!(state.runners, RunnersOn::Runner111));

        state.runners = RunnersOn::Runner101;
        state = add_runner(state, &2, player1.clone());
        assert!(matches!(state.runners, RunnersOn::Runner111));

        state.runners = RunnersOn::Runner000;
        state = add_runner(state, &1, player1.clone());
        assert!(matches!(state.runners, RunnersOn::Runner100));

        state = add_runner(state, &2, player1.clone());
        assert!(matches!(state.runners, RunnersOn::Runner110));
    }

    // load_csv function test
    #[test]
    fn test_load_csv() {
        let filename = "src/testfiles/csv_test.csv";
        let delimiter = "\n";
        let result = load_csv(filename, delimiter).unwrap();
        assert_eq!(result.len(), 4);
        assert!(result[0] == "this");
        assert!(result[1] == "is");
        assert!(result[2] == "a");
        assert!(result[3] == "test");
    }

    // generate player function test
    #[test]
    fn test_generate_player() {
        // use same file name every time so test directory isn't mindlessly spammed
        // make basic first name and last name vectors to keep things simple
        // make them vector of vectors so you can take randomness out of player names for test
        let firstnames = vec![
            vec!["Seth".to_string(), "Seth".to_string()],
            vec!["Ben".to_string(), "Ben".to_string()],
            vec!["Chuck".to_string(), "Chuck".to_string()],
        ];
        let lastnames = vec![
            vec!["Loveall".to_string(), "Loveall".to_string()],
            vec!["Smith".to_string(), "Smith".to_string()],
            vec!["Schuldiner".to_string(), "Schuldiner".to_string()],
        ];
        for i in 0..3 as usize {
            let test_player = generate_player(
                PlayerClass::StartingHitter,
                Position::Firstbase,
                &firstnames[i],
                &lastnames[i],
            );
            let mut filename = "src/testfiles/game_test/test_player".to_string();
            filename.push_str(&i.to_string());
            filename.push_str(".dbp");
            _ = write_player(&test_player, &filename);
            let contents = fs::read_to_string(filename).unwrap();
            let read_player = load_player(contents);

            let position = read_player.position;
            let handedness = read_player.handedness;
            let raits = read_player.traits;
            assert!(matches!(test_player.position, position));
            assert!(test_player.first_name == read_player.first_name);
            assert!(test_player.last_name == read_player.last_name);
            assert!(matches!(test_player.handedness, handedness));
            assert_eq!(test_player.batter_target, read_player.batter_target);
            assert_eq!(test_player.on_base_target, read_player.on_base_target);
            assert_eq!(test_player.pitch_die, read_player.pitch_die);
            assert!(matches!(test_player.traits, traits));
        }
    }

    // find player by position test
    #[test]
    fn test_find_by_position() {
        let filename = "src/testfiles/game/teams/blue_team.dbt";
        let contents = fs::read_to_string(filename).unwrap();
        let team = load_team(contents);
        let (roster, _, _, _) = load_roster(&team);
        let second_baseman = find_by_position(Position::Secondbase, &roster).unwrap();
        assert_eq!(second_baseman.batter_target, 26); // this was easier than actually comparing
                                                      // name strings or something, lol
    }

    // debug_roll test
    #[test]
    fn test_debug_roll() {
        let mut config = DebugConfig {
            mode: true,
            rolls: vec![12, 15, 20],
            roll_index: 0,
        };
        let result1 = debug_roll(&mut config, 10);
        let result2 = debug_roll(&mut config, 10);
        let result3 = debug_roll(&mut config, 10);
        assert_eq!(result1, 12);
        assert_eq!(result2, 15);
        assert_eq!(result3, 20);
    }

    /*
    // oddity test
    #[test]
    fn test_oddity() {
        let odd = oddity(&1, &1, &game, state);
    }
    */

    /*
    // hit_table test
    #[test]
    fn test_hit_table() {
        let mut state = GameState { status: GameStatus::Ongoing, inning: 1, inning_half: InningTB::Top, outs: Outs::One, runners: RunnersOn::Runner000, batting_team1: (), batting_team2: (), current_pitcher_team1: (), current_pitcher_team2: (), pitched_team1: (), pitched_team2: (), runs_team1: (), runs_team2: (), hits_team1: (), hits_team2: (), errors_team1: (), errors_team2: () };
        let hit_result = hit_table(&1, state);
    }
    */

    // get_swing_position()
    #[test]
    fn test_get_swing_position() {
        let mut position = get_swing_position(&31);
        assert_eq!(position, 1);
        position = get_swing_position(&42);
        assert_eq!(position, 2);
        position = get_swing_position(&53);
        assert_eq!(position, 3);
        position = get_swing_position(&64);
        assert_eq!(position, 4);
        position = get_swing_position(&75);
        assert_eq!(position, 5);
        position = get_swing_position(&86);
        assert_eq!(position, 6);
        position = get_swing_position(&17);
        assert_eq!(position, 7);
        position = get_swing_position(&28);
        assert_eq!(position, 8);
        position = get_swing_position(&39);
        assert_eq!(position, 9);
    }

    #[test]
    fn test_trait_check() {
        let mut player1 = Player {
            first_name: "Seth".to_string(),
            nickname: "".to_string(),
            last_name: "Loveall".to_string(),
            position: Position::Firstbase,
            handedness: Handedness::Right,
            batter_target: 30,
            on_base_target: 30,
            pitch_die: -12,
            traits: vec![Traits::GreatDefender],
            injury_location: vec![InjuryLocation::None],
            injury_severity: vec![],
        };
        let mut player2 = player1.clone();
        let mut player3 = player1.clone();
        let mut player4 = player1.clone();

        // defense
        player2.traits = vec![Traits::None];
        player3.traits = vec![Traits::PoorDefender];
        assert_eq!(player1.defense(), 1);
        assert_eq!(player2.defense(), 0);
        assert_eq!(player3.defense(), -1);

        // power hitter
        player1.traits = vec![Traits::PowerHitter];
        player2.traits = vec![Traits::ExtraWeakHitter];
        player3.traits = vec![Traits::ElitePowerHitter];
        player4.traits = vec![Traits::WeakHitter];
        assert_eq!(player1.power(), 1);
        assert_eq!(player2.power(), -2);
        assert_eq!(player3.power(), 2);
        assert_eq!(player4.power(), -1);

        player1.traits = vec![Traits::ContactHitter];
        assert_eq!(player1.contact_hit(), true);

        player1.traits = vec![Traits::FreeSwinger];
        assert_eq!(player1.free_swing(), true);

        player1.traits = vec![Traits::SpeedyRunner];
        assert_eq!(player1.speedy(), true);

        player1.traits = vec![Traits::SlowRunner];
        assert_eq!(player1.slow(), true);

        player1.traits = vec![Traits::ToughPlayer];
        assert_eq!(player1.tough(), true);

        player1.traits = vec![Traits::StrikeoutArtist];
        assert_eq!(player1.strikeout(), true);

        player1.traits = vec![Traits::GroundballMachine];
        assert_eq!(player1.groundball(), true);

        player1.traits = vec![Traits::GreatStamina];
        assert_eq!(player1.stamina(), true);

        player1.traits = vec![Traits::ControlPitcher];
        player2.traits = vec![Traits::Wild];
        assert_eq!(player1.control(), -2);
        assert_eq!(player2.control(), 3);
    }

    // TODO: make test function names uniform

    #[test]
    fn increment_out_check() {
        let mut current = Outs::None;
        current = increment_out(current, 1);
        assert_eq!(current, Outs::One);
        current = increment_out(current, 1);
        assert_eq!(current, Outs::Two);
        current = increment_out(current, 1);
        assert_eq!(current, Outs::Three);
        current = increment_out(current, 1);
        assert_eq!(current, Outs::Three);
        current = Outs::None;
        current = increment_out(current, 2);
        assert_eq!(current, Outs::Two);
        current = increment_out(current, 2);
        assert_eq!(current, Outs::Three);
        current = Outs::None;
        current = increment_out(current, 3);
        assert_eq!(current, Outs::Three);
    }

    #[test]
    fn test_process_steals() {
        // create GameState, GameModern, DebugConfig, Player
        let red_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/red_team.dbt").unwrap());
        let blue_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/blue_team.dbt").unwrap());
        let ballpark = load_park_modern(
            fs::read_to_string("src/testfiles/game/ballparks/Nightside Field.dbb").unwrap(),
        );
        let game = create_modern_game(red_team, blue_team, ballpark).unwrap();
        let mut state = init_new_game_state(
            game.home_active.pitching[0].clone(),
            game.away_active.pitching[0].clone(),
        );
        let mut debug = DebugConfig {
            mode: true,
            rolls: vec![3],
            roll_index: 0,
        };
        let mut stealer = game.home_active.batting_order[2].clone();
        let mut catcher = find_by_position(Position::Catcher, &game.away_active.roster).unwrap();
        stealer.traits = vec![Traits::SpeedyRunner];
        catcher.traits = vec![Traits::None];
        state.inning_half = InningTB::Bottom;
        state.status = GameStatus::Ongoing;
        state.runners = RunnersOn::Runner100;
        state.runner1 = Some(stealer.clone());
        state.batting_team1 = 3;

        let mut new_state =
            process_steals(StealType::Second, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner010);

        stealer.traits = vec![Traits::SlowRunner];
        state.runner1 = Some(stealer.clone());
        debug.rolls = vec![4];
        new_state = process_steals(StealType::Second, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner000);

        stealer.traits = vec![Traits::SpeedyRunner];
        state.runner1 = None;
        state.runner2 = Some(stealer.clone());
        state.runners = RunnersOn::Runner010;
        debug.rolls = vec![4];
        new_state = process_steals(StealType::Third, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner001);

        debug.rolls = vec![2];
        catcher.traits = vec![Traits::GreatDefender];
        new_state = process_steals(StealType::Third, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner000);

        debug.rolls = vec![8];
        state.runners = RunnersOn::Runner001;
        state.runner3 = state.runner2.clone();
        state.runner2 = None;
        new_state = process_steals(StealType::Home, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner000);
        assert_eq!(new_state.runs_team1, 1);

        debug.rolls = vec![1];
        state.runners = RunnersOn::Runner110;
        state.runner2 = state.runner3.clone();
        state.runner1 = state.runner2.clone();
        state.runner3 = None;
        new_state = process_steals(StealType::Double, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner010);

        debug.rolls = vec![4];
        new_state = process_steals(StealType::Double, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner001);

        debug.rolls = vec![7];
        new_state = process_steals(StealType::Double, state.clone(), debug.clone(), &catcher);
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner011);
    }

    #[test]
    fn test_bunt() {
        // create GameState, GameModern, DebugConfig, Player
        let red_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/red_team.dbt").unwrap());
        let blue_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/blue_team.dbt").unwrap());
        let ballpark = load_park_modern(
            fs::read_to_string("src/testfiles/game/ballparks/Nightside Field.dbb").unwrap(),
        );
        let game = create_modern_game(red_team, blue_team, ballpark).unwrap();
        let mut state = init_new_game_state(
            game.home_active.pitching[0].clone(),
            game.away_active.pitching[0].clone(),
        );
        let mut debug = DebugConfig {
            mode: true,
            rolls: vec![1],
            roll_index: 0,
        };
        let mut batter = game.home_active.batting_order[3].clone();
        state.inning_half = InningTB::Bottom;
        state.status = GameStatus::Ongoing;
        state.runners = RunnersOn::Runner100;
        state.runner1 = Some(game.home_active.batting_order[2].clone());
        state.batting_team1 = 3;
        batter.traits = vec![Traits::ContactHitter];

        // bunt_result = 2
        let mut new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner100);

        // bunt_result = 3
        batter.traits = vec![Traits::FreeSwinger];
        debug.rolls = vec![4];
        new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner010);
        state.runners = RunnersOn::Runner001;
        state.runner3 = state.runner1.clone();
        state.runner1 = None;
        new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner100);

        // bunt_result = 4/5
        batter.traits = vec![Traits::None];
        debug.rolls = vec![5];
        new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner000);

        // bunt_result = 6
        debug.rolls = vec![6];
        new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner000);
        debug.rolls = vec![6, 4];
        batter.traits = vec![Traits::SpeedyRunner];
        new_state = bunt(state.clone(), &game, debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner100);
    }

    #[test]
    fn test_hit_and_run() {
        // create GameState, GameModern, DebugConfig, Player
        let red_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/red_team.dbt").unwrap());
        let blue_team =
            load_team(fs::read_to_string("src/testfiles/game/teams/blue_team.dbt").unwrap());
        let ballpark = load_park_modern(
            fs::read_to_string("src/testfiles/game/ballparks/Nightside Field.dbb").unwrap(),
        );
        let game = create_modern_game(red_team, blue_team, ballpark).unwrap();
        let mut state = init_new_game_state(
            game.home_active.pitching[0].clone(),
            game.away_active.pitching[0].clone(),
        );
        let mut debug = DebugConfig {
            mode: true,
            rolls: vec![8, 1, 37],
            roll_index: 0,
        };
        let mut stealer = game.home_active.batting_order[2].clone();
        let mut batter = game.home_active.batting_order[3].clone();
        batter.batter_target = 30;
        batter.on_base_target = 30;
        batter.traits = vec![Traits::ContactHitter];
        stealer.traits = vec![Traits::SpeedyRunner];
        state.inning_half = InningTB::Bottom;
        state.status = GameStatus::Ongoing;
        state.runners = RunnersOn::Runner100;
        state.runner1 = Some(stealer.clone());
        state.batting_team1 = 3;

        let mut new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner101);

        batter.traits = vec![Traits::FreeSwinger];
        debug.rolls = vec![1, 1, 10];
        new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::None);
        assert_eq!(new_state.runners, RunnersOn::Runner110);

        debug.rolls = vec![8, 1, 70];
        new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner100);

        debug.rolls = vec![1, 1, 37];
        new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::Two);
        assert_eq!(new_state.runners, RunnersOn::Runner000);

        debug.rolls = vec![8, 4, 70];
        new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::One);
        assert_eq!(new_state.runners, RunnersOn::Runner010);

        debug.rolls = vec![1, 4, 70];
        new_state = hit_and_run(state.clone(), &game, &mut debug.clone(), batter.clone());
        assert_eq!(new_state.outs, Outs::Two);
        assert_eq!(new_state.runners, RunnersOn::Runner000);
    }
}
