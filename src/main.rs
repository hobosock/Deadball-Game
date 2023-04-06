use deadball::characters::players::*;
use deadball::characters::teams::*;
use deadball::core::*;

use std::fs;

fn main() {
    /*
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

    println!("{:?}", test_team.bench[0]);
    println!("{:?}", test_team.pitcher[0]);
    */

    let filename = "src/testfiles/detroit_steam_hammers.dbt";

    let contents = fs::read_to_string(filename).unwrap();

    let read_team = load_team(contents);

    let test_roster = &read_team.roster;
    let test_bench = &read_team.bench;
    let test_pitcher = &read_team.pitcher;
    let test_bullpen = &read_team.bullpen;

    println!("{:?}", test_roster);
    println!("{:?}", test_bench);
    println!("{:?}", test_pitcher);
    println!("{:?}", test_bullpen);

    println!("======================");

    let (roster, bench, pitcher, bullpen) = load_roster(&read_team);
    println!("{:?}", roster[0].first_name);
    println!("{:?}", bench[0].first_name);
    println!("{:?}", pitcher[0].first_name);
    println!("{:?}", bullpen[0].first_name);
    println!("======================");

    let read_results = fs::read_to_string(&test_roster[0]);
    println!("{:?}", read_results);
    let results = read_results.unwrap();
    let player_check = load_player(results);
    let name_check = player_check.first_name;
    println!("{}", name_check);
}
