/*==========================================
MODULE INCLUSIONS
==========================================*/
use std::fs;
use text_colorizer::*;

use crate::core::{
    file_locations::{BALLPARK_LOCATION, PLAYER_LOCATION},
    //game_functions::modern_game_flow,
    *,
};

use super::{ballparks::*, players::*};

/*==========================================
ENUM DEFINITIONS
==========================================*/
// TEAM ENUMS
#[derive(Clone, PartialEq)]
pub enum Era {
    Ancient,
    Modern,
    None,
}

#[derive(Clone)]
pub enum Location {
    MiddleOfNowhere,
    SmallTown,
    SmallCity,
    MediumSizedCity,
    Metropolis,
    None,
}

#[derive(Clone)]
pub enum Priority {
    Power,
    Average,
    StartingPitching,
    Bullpen,
    Speed,
    Defense,
    None,
}

#[derive(Clone)]
pub enum Makeup {
    MostlyProspects,
    Balanced,
    MostlyVeterans,
    None,
}

/*
pub enum Personality {
    Baffled,
    Boastful,
    Combative,
    Cowardly,
    Destructive,
    Elegant,
    EvenKeeled,
    Giddy,
    Gossipy,
    Gregarious,
    Hedonistic,
    Humble,
    Lovable,
    Miserly,
    Noble,
    Quixotic,
    Sadistic,
    Slovenly,
    Tempermental,
    Unbalanced,
}
*/

/*
pub enum Background {
    CaptainofIndustry,
    EccentricInventor,
    Entertainer,
    FormerPlayer,
    HeirtoPreviousOwner,
    LocalGovernment,
    LocalMagnate,
    MediaPersonality,
    MillionaireRecluse,
    MultinationalCorporation,
    NewspaperSyndicate,
    OilMan,
    PlayersCooperative,
    Politician,
    RailroadBaron,
    RealEstateDeveloper,
    RiverboatGambler,
    RollerCoasterTycoon,
    VentureCapitalist,
    WarHero,
}
*/

// championship - I don't think this is needed
// mascot
// years in league
// owner background
// fanbase
// park name
// park Location

#[derive(Clone)]
pub enum Fanbase {
    Nonexistent,
    Indifferent,
    FairWeather,
    Loyal,
    Obsessive,
    None,
}

// Manager
#[derive(Clone)]
pub enum ManagerLeague {
    Major,
    Minor,
    None,
}

/*==========================================
STRUCTURES
==========================================*/
#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub ballpark: String, // file name to *.DBB file
    pub manager: String,  //name
    pub logo: String,     // image file?
    pub era: Era,
    pub location: Location,
    pub mascot: String,
    pub priority: Priority,
    pub makeup: Makeup,
    pub years: i32,
    pub championship: i32,
    pub fanbase: Fanbase,
    pub manager_position: Position,
    pub manager_league: ManagerLeague,
    pub retired: i32,
    pub personality: String, // could be an ENUM if you want
    pub daring: i32,
    pub motto: String,
    pub owner_background: String,
    pub owner_personality: String,
    pub roster: Vec<String>,
    pub bench: Vec<String>,
    pub pitcher: Vec<String>,
    pub bullpen: Vec<String>,
}

/// struct for teams in a game - loads player files into Player structs for easier reference
#[derive(Clone)]
pub struct ActiveTeam {
    pub roster: Vec<Player>,
    pub bench: Vec<Player>,
    pub pitching: Vec<Player>,
    pub bullpen: Vec<Player>,
    pub batting_order: Vec<Player>,
}

/// struct for tracking innings pitched, current batter, etc. for a team
/// this might help with repeating logic for top/bottom innings
#[derive(Clone, Debug)]
pub struct TeamState {
    pub current_batter: u32,
    pub current_pitcher: Player,
    pub innings_pitched: u32,
    pub runs: u32,
    pub hits: u32,
    pub errors: u32,
}
// TODO: make runs/hits/errors vectors with a value for each inning
// TODO: make fields to track temp bonuses

/*==========================================
FUNCTIONS
==========================================*/

/// load team file *.DBT
pub fn load_team(contents: String) -> Team {
    // initialize variables for all the different fields
    let mut name = String::new();
    let mut ballpark = String::new();
    let mut manager = String::new();
    let mut logo = String::new();
    let mut era = Era::None;
    let mut location = Location::None;
    let mut mascot = String::new();
    let mut priority = Priority::None;
    let mut makeup = Makeup::None;
    let mut years: i32 = 0;
    let mut championship: i32 = 0;
    let mut fanbase = Fanbase::None;
    let mut manager_position = Position::None;
    let mut manager_league = ManagerLeague::None;
    let mut retired: i32 = 0;
    let mut personality = String::new();
    let mut daring: i32 = 0;
    let mut motto = String::new();
    let mut roster = Vec::new();
    let mut bench = Vec::new();
    let mut pitcher = Vec::new();
    let mut bullpen = Vec::new();
    let mut owner_background = String::new();
    let mut owner_personality = String::new();

    // sort text into relevant fields
    let rows: Vec<&str> = contents.split("\n").collect();
    for i in 0..rows.len() {
        // last line is usually just a new line character
        let rowline: Vec<&str> = rows[i].split(":").collect();
        if rowline[0].trim().eq("TEAM") {
            name = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("BALLPARK") {
            ballpark = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("MANAGER") {
            manager = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("LOGO") {
            logo = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("ERA") {
            if rowline[1].trim().eq("Modern") {
                era = Era::Modern;
            } else if rowline[1].trim().eq("Ancient") {
                era = Era::Ancient;
            }
        } else if rowline[0].trim().eq("LOCATION") {
            if rowline[1].trim().eq("Middle of Nowhere") {
                location = Location::MiddleOfNowhere;
            } else if rowline[1].trim().eq("Small Town") {
                location = Location::SmallTown;
            } else if rowline[1].trim().eq("Small City") {
                location = Location::SmallCity;
            } else if rowline[1].trim().eq("Medium Sized City") {
                location = Location::MediumSizedCity;
            } else if rowline[1].trim().eq("Metropolis") {
                location = Location::Metropolis;
            }
        } else if rowline[0].trim().eq("MASCOT") {
            mascot = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("TEAM PRIORITY") {
            if rowline[1].trim().eq("Power") {
                priority = Priority::Power;
            } else if rowline[1].trim().eq("Average") {
                priority = Priority::Average;
            } else if rowline[1].trim().eq("Starting Pitching") {
                priority = Priority::StartingPitching;
            } else if rowline[1].trim().eq("Bullpen") {
                priority = Priority::Bullpen;
            } else if rowline[1].trim().eq("Speed") {
                priority = Priority::Speed;
            } else if rowline[1].trim().eq("Defense") {
                priority = Priority::Defense;
            }
        } else if rowline[0].trim().eq("TEAM MAKEUP") {
            if rowline[1].trim().eq("Mostly Prospects") {
                makeup = Makeup::MostlyProspects;
            } else if rowline[1].trim().eq("Balanced") {
                makeup = Makeup::Balanced;
            } else if rowline[1].trim().eq("Mostly Veterans") {
                makeup = Makeup::MostlyVeterans;
            }
        } else if rowline[0].trim().eq("YEARS IN LEAGUE") {
            let years_result = rowline[1].trim().parse();
            match years_result {
                Ok(yr) => years = yr,
                Err(_err) => println!(
                    "{}",
                    "Failed to convert 'years in league' number".red().bold()
                ),
            }
        } else if rowline[0].trim().eq("MOST RECENT CHAMPIONSHIP") {
            let champ_result = rowline[1].trim().parse();
            match champ_result {
                Ok(champ) => championship = champ,
                Err(_err) => println!(
                    "{}",
                    "Failed to convert 'most recent championship' number"
                        .red()
                        .bold()
                ),
            }
        } else if rowline[0].trim().eq("FANBASE") {
            if rowline[1].trim().eq("Non-existent") {
                fanbase = Fanbase::Nonexistent;
            } else if rowline[1].trim().eq("Indifferent") {
                fanbase = Fanbase::Indifferent;
            } else if rowline[1].trim().eq("Fair Weather") {
                fanbase = Fanbase::FairWeather;
            } else if rowline[1].trim().eq("Loyal") {
                fanbase = Fanbase::Loyal;
            } else if rowline[1].trim().eq("Obsessive") {
                fanbase = Fanbase::Obsessive;
            }
        } else if rowline[0].trim().eq("POSITION") {
            if rowline[1].trim().eq("P") {
                manager_position = Position::Pitcher;
            } else if rowline[1].trim().eq("C") {
                manager_position = Position::Catcher;
            } else if rowline[1].trim().eq("1B") {
                manager_position = Position::Firstbase;
            } else if rowline[1].trim().eq("2B") {
                manager_position = Position::Secondbase;
            } else if rowline[1].trim().eq("SS") {
                manager_position = Position::Shortstop;
            } else if rowline[1].trim().eq("3B") {
                manager_position = Position::Thirdbase;
            } else if rowline[1].trim().eq("RF") {
                manager_position = Position::Rightfield;
            } else if rowline[1].trim().eq("CF") {
                manager_position = Position::Centerfield;
            } else if rowline[1].trim().eq("LF") {
                manager_position = Position::Leftfield;
            } else if rowline[1].trim().eq("None") {
                manager_position = Position::None;
            }
        } else if rowline[0].trim().eq("LEAGUE") {
            if rowline[1].trim().eq("Majors") {
                manager_league = ManagerLeague::Major;
            } else if rowline[1].trim().eq("Minors") {
                manager_league = ManagerLeague::Minor;
            } else if rowline[1].trim().eq("None") {
                manager_league = ManagerLeague::None;
            }
        } else if rowline[0].trim().eq("RETIRED") {
            let retired_result = rowline[1].trim().parse();
            match retired_result {
                Ok(rtr) => retired = rtr,
                Err(_err) => println!("{}", "Failed to convert 'retired' to number".red().bold()),
            }
        } else if rowline[0].trim().eq("PERSONALITY") {
            personality = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("DARING") {
            let daring_result = rowline[1].trim().parse();
            match daring_result {
                Ok(dare) => daring = dare,
                Err(_err) => println!("{}", "Failed to convert 'daring' to number.".red().bold()),
            }
        } else if rowline[0].trim().eq("MOTTO") {
            motto = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("BACKGROUND") {
            owner_background = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("PERSONALITY") {
            owner_personality = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("PLAYER") {
            roster.push(rowline[1].trim().to_string());
        } else if rowline[0].trim().eq("BENCH") {
            bench.push(rowline[1].trim().to_string());
        } else if rowline[0].trim().eq("PITCHER") {
            pitcher.push(rowline[1].trim().to_string());
        } else if rowline[0].trim().eq("BULLPEN") {
            bullpen.push(rowline[1].trim().to_string());
        }
    }

    let team_data = Team {
        name,
        ballpark,
        manager,
        logo,
        era,
        location,
        mascot,
        priority,
        makeup,
        years,
        championship,
        fanbase,
        manager_position,
        manager_league,
        retired,
        personality,
        daring,
        motto,
        owner_background,
        owner_personality,
        roster,
        bench,
        pitcher,
        bullpen,
    };

    team_data
}

/// write team file *.DBT
pub fn write_team(data: Team, filename: &str) -> Result<(), std::io::Error> {
    let mut file_text = String::new();
    file_text.push_str("TEAM: ");
    file_text.push_str(&data.name);
    file_text.push_str("\nBALLPARK: ");
    file_text.push_str(&data.ballpark);
    file_text.push_str("\nLOGO: ");
    file_text.push_str(&data.logo);
    file_text.push_str("\nERA: ");
    match data.era {
        Era::None => file_text.push_str("None"),
        Era::Modern => file_text.push_str("Modern"),
        Era::Ancient => file_text.push_str("Ancient"),
    }
    file_text.push_str("\nLocation: ");
    match data.location {
        Location::None => file_text.push_str("None"),
        Location::SmallTown => file_text.push_str("Small Town"),
        Location::SmallCity => file_text.push_str("Small City"),
        Location::Metropolis => file_text.push_str("Metropolis"),
        Location::MiddleOfNowhere => file_text.push_str("Middle Of Nowhere"),
        Location::MediumSizedCity => file_text.push_str("Medium Sized City"),
    }
    file_text.push_str("\nMASCOT: ");
    file_text.push_str(&data.mascot);
    file_text.push_str("\nTEAM PRIORITY: ");
    match data.priority {
        Priority::None => file_text.push_str("None"),
        Priority::Power => file_text.push_str("Power"),
        Priority::Speed => file_text.push_str("Speed"),
        Priority::Average => file_text.push_str("Average"),
        Priority::Bullpen => file_text.push_str("Bullpen"),
        Priority::Defense => file_text.push_str("Defense"),
        Priority::StartingPitching => file_text.push_str("StartingPitching"),
    }
    file_text.push_str("\nTEAM MAKEUP: ");
    match data.makeup {
        Makeup::None => file_text.push_str("None"),
        Makeup::Balanced => file_text.push_str("Balanced"),
        Makeup::MostlyVeterans => file_text.push_str("Mostly Veterans"),
        Makeup::MostlyProspects => file_text.push_str("Mostly Prospects"),
    }
    file_text.push_str("\nYEARS IN LEAGUE: ");
    file_text.push_str(&data.years.to_string());
    file_text.push_str("\nMOST RECENT CHAMPIONSHIP: ");
    file_text.push_str(&data.championship.to_string());
    file_text.push_str("\nFANBASE: ");
    match data.fanbase {
        Fanbase::None => file_text.push_str("None"),
        Fanbase::Loyal => file_text.push_str("Loyal"),
        Fanbase::Obsessive => file_text.push_str("Obsessive"),
        Fanbase::Nonexistent => file_text.push_str("Non-existent"),
        Fanbase::Indifferent => file_text.push_str("Indifferent"),
        Fanbase::FairWeather => file_text.push_str("Fair Weather"),
    }
    file_text.push_str("/n/n## MANAGER INFO\nMANAGER: ");
    file_text.push_str(&data.manager);
    file_text.push_str("\nPOSITION: ");
    match data.manager_position {
        Position::None => file_text.push_str("None"),
        Position::Bench => file_text.push_str("Bench"),
        Position::Pitcher => file_text.push_str("P"),
        Position::Catcher => file_text.push_str("C"),
        Position::Firstbase => file_text.push_str("1B"),
        Position::Shortstop => file_text.push_str("2B"),
        Position::Thirdbase => file_text.push_str("3B"),
        Position::Leftfield => file_text.push_str("LF"),
        Position::Secondbase => file_text.push_str("2B"),
        Position::Rightfield => file_text.push_str("RF"),
        Position::Centerfield => file_text.push_str("CF"),
    }
    file_text.push_str("\nLEAGUE: ");
    match data.manager_league {
        ManagerLeague::None => file_text.push_str("None"),
        ManagerLeague::Major => file_text.push_str("Majors"),
        ManagerLeague::Minor => file_text.push_str("Minors"),
    }
    file_text.push_str("\nRETIRED: ");
    file_text.push_str(&data.retired.to_string());
    file_text.push_str("\nPERSONALITY: ");
    file_text.push_str(&data.personality);
    file_text.push_str("\nDARING: ");
    file_text.push_str(&data.daring.to_string());
    file_text.push_str("\nMOTTO: ");
    file_text.push_str(&data.motto);
    file_text.push_str("\n\n## OWNER INFO\nBACKGROUND: ");
    file_text.push_str(&data.owner_background);
    file_text.push_str("\nOWNER PERSONALITY: ");
    file_text.push_str(&data.owner_personality);
    file_text.push_str("\n\n## ROSTER");
    for i in 0..data.roster.len() {
        file_text.push_str("\nPLAYER: ");
        file_text.push_str(&data.roster[i]);
    }
    for i in 0..data.bench.len() {
        file_text.push_str("\nBENCH: ");
        file_text.push_str(&data.bench[i]);
    }
    for i in 0..data.pitcher.len() {
        file_text.push_str("\nPITCHER: ");
        file_text.push_str(&data.pitcher[i]);
    }
    for i in 0..data.bullpen.len() {
        file_text.push_str("\nBULLPEN: ");
        file_text.push_str(&data.bullpen[i]);
    }

    let write_result = fs::write(filename, &file_text);
    write_result
}

/// turns a Team struct into separate vectors for fielders/bench/pitcher/bullpen
pub fn load_roster(team: &Team) -> (Vec<Player>, Vec<Player>, Vec<Player>, Vec<Player>) {
    let mut roster = Vec::new();
    let mut bench = Vec::new();
    let mut pitcher = Vec::new();
    let mut bullpen = Vec::new();

    for i in 0..team.roster.len() {
        let read_results = fs::read_to_string(&team.roster[i]);
        match read_results {
            Ok(content) => roster.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &team.roster[i]
            ),
        }
    }
    for i in 0..team.bench.len() {
        let read_results = fs::read_to_string(&team.bench[i]);
        match read_results {
            Ok(content) => bench.push(load_player(content)),
            Err(_err) => println!("{}: {}", "failed to load file".red().bold(), &team.bench[i]),
        }
    }
    for i in 0..team.pitcher.len() {
        let read_results = fs::read_to_string(&team.pitcher[i]);
        match read_results {
            Ok(content) => pitcher.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &team.pitcher[i]
            ),
        }
    }
    for i in 0..team.bullpen.len() {
        let read_results = fs::read_to_string(&team.bullpen[i]);
        match read_results {
            Ok(content) => bullpen.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &team.bullpen[i]
            ),
        }
    }

    return (roster, bench, pitcher, bullpen);
}

/// generate manager - can borrow a lot from player gen function
pub fn generate_manager(firstnames: &Vec<String>, lastnames: &Vec<String>) -> String {
    let (first_name, last_name) = generate_name(firstnames, lastnames);
    let name = first_name + &last_name;
    return name;
}

/// generate logo
pub fn generate_logo(logos: &Vec<String>) -> String {
    let len1 = logos.len();
    let roll1 = roll(len1 as i32) - 1;
    let logo = logos[roll1 as usize].clone();
    return logo;
}

/// generate location
pub fn generate_location() -> Location {
    let result = roll(5);
    let location: Location;
    if result == 1 {
        location = Location::MiddleOfNowhere;
    } else if result == 2 {
        location = Location::MediumSizedCity;
    } else if result == 3 {
        location = Location::SmallTown;
    } else if result == 4 {
        location = Location::SmallCity;
    } else if result == 5 {
        location = Location::Metropolis;
    } else {
        location = Location::None;
    }
    return location;
}
/*
pub fn generate_location(locations: Vec<String>) -> String {
    let len1 = locations.len();
    let roll1 = roll(len1 as i32);
    let location = locations[roll1 as usize].clone();
    return location;
}
*/

/// generate mascot
pub fn generate_mascot(mascots: &Vec<String>) -> String {
    let len1 = mascots.len();
    let roll1 = roll(len1 as i32) - 1;
    let mascot = mascots[roll1 as usize].clone();
    return mascot;
}

//TODO: make it impact player ages and traits?
/// generate team priority
pub fn generate_priority() -> Priority {
    let result = roll(7);
    let priority: Priority;
    if result == 1 {
        priority = Priority::Power;
    } else if result == 2 {
        priority = Priority::None;
    } else if result == 3 {
        priority = Priority::Speed;
    } else if result == 4 {
        priority = Priority::Average;
    } else if result == 5 {
        priority = Priority::Bullpen;
    } else if result == 6 {
        priority = Priority::Defense;
    } else if result == 7 {
        priority = Priority::StartingPitching;
    } else {
        priority = Priority::None;
    }

    return priority;
}

/// generate makeup - same TODO
pub fn generate_makeup() -> Makeup {
    let result = roll(4);
    let makeup: Makeup;
    if result == 1 {
        makeup = Makeup::MostlyProspects;
    } else if result == 2 {
        makeup = Makeup::MostlyVeterans;
    } else if result == 3 {
        makeup = Makeup::Balanced
    } else if result == 4 {
        makeup = Makeup::None;
    } else {
        makeup = Makeup::None;
    }

    return makeup;
}

/// generate fanbase
pub fn generate_fanbase() -> Fanbase {
    let result = roll(5);
    let fanbase: Fanbase;
    if result == 1 {
        fanbase = Fanbase::Loyal;
    } else if result == 2 {
        fanbase = Fanbase::Obsessive;
    } else if result == 3 {
        fanbase = Fanbase::Nonexistent;
    } else if result == 4 {
        fanbase = Fanbase::Indifferent;
    } else if result == 5 {
        fanbase = Fanbase::FairWeather;
    } else {
        fanbase = Fanbase::None;
    }

    return fanbase;
}

/// generate manager position
pub fn generate_manager_position() -> Position {
    let result = roll(10);
    let position: Position;
    if result == 1 {
        position = Position::Pitcher;
    } else if result == 2 {
        position = Position::Catcher;
    } else if result == 3 {
        position = Position::Firstbase;
    } else if result == 4 {
        position = Position::Secondbase;
    } else if result == 5 {
        position = Position::Shortstop;
    } else if result == 6 {
        position = Position::Thirdbase;
    } else if result == 7 {
        position = Position::Rightfield;
    } else if result == 8 {
        position = Position::Centerfield;
    } else if result == 9 {
        position = Position::Leftfield;
    } else {
        position = Position::None;
    }
    return position;
}

/// generate manager league
pub fn generate_league(position: &Position) -> ManagerLeague {
    let league: ManagerLeague;
    match position {
        Position::None => {
            league = ManagerLeague::None;
        }
        _ => {
            let result = roll(2);
            if result == 1 {
                league = ManagerLeague::Major;
            } else {
                league = ManagerLeague::Minor;
            }
        }
    }
    return league;
}

/// generate retired - just roll
pub fn generate_retired() -> i32 {
    let result = roll(30);
    return result;
}

/// generate personality
pub fn generate_personality(personalities: &Vec<String>) -> String {
    let len1 = personalities.len();
    let result = roll(len1 as i32) - 1;
    let personality = personalities[result as usize].clone();
    return personality;
}

/*
pub fn generate_personality() -> Personality {
    let result = roll(20);
    let personality: Personality;
    if result == 1 {
        personality = Personality::Giddy;
    } else if result == 2 {
        personality = Personality::Noble;
    } else if result == 3 {
        personality = Personality::Humble;
    } else if result == 4 {
        personality = Personality::Baffled;
    } else if result == 5 {
        personality = Personality::Elegant;
    } else if result == 6 {
        personality = Personality::Gossipy;
    } else if result == 7 {
        personality = Personality::Lovable;
    } else if result == 8 {
        personality = Personality::Miserly;
    } else if result == 9 {
        personality = Personality::Boastful;
    } else if result == 10 {
        personality = Personality::Cowardly;
    } else if result == 11 {
        personality = Personality::Quixotic;
    } else if result == 12 {
        personality = Personality::Sadistic;
    } else if result == 13 {
        personality = Personality::Slovenly;
    } else if result == 14 {
        personality = Personality::Combative;
    } else if result == 15 {
        personality = Personality::EvenKeeled;
    } else if result == 16 {
        personality = Personality::Gregarious;
    } else if result == 17 {
        personality = Personality::Hedonistic;
    } else if result == 18 {
        personality = Personality::Unbalanced;
    } else if result == 19 {
        personality = Personality::Destructive;
    } else if result == 20 {
        personality = Personality::Tempermental;
    } else {
        personality = Personality::EvenKeeled;
    }

    return personality;
}
*/

/// generate motto???
pub fn generate_motto(mottos: &Vec<String>) -> String {
    let len1 = mottos.len();
    let roll1 = roll(len1 as i32) - 1;
    let motto = mottos[roll1 as usize].clone();
    return motto;
}

/// generate owner background
pub fn generate_background(backgrounds: &Vec<String>) -> String {
    let len1 = backgrounds.len();
    let roll1 = roll(len1 as i32) - 1;
    let background = backgrounds[roll1 as usize].clone();
    return background;
}
/*
pub fn generate_background() -> Background {
    let result = roll(20);
    let background: Background;
    if result == 1 {
        background = Background::PlayersCooperative;
    } else if result == 2 {
        background = Background::LocalGovernment;
    } else if result == 3 {
        background = Background::EccentricInventor;
    } else if result == 4 {
        background = Background::OilMan;
    } else if result == 5 {
        background = Background::WarHero;
    } else if result == 6 {
        background = Background::Politician;
    } else if result == 7 {
        background = Background::Entertainer;
    } else if result == 8 {
        background = Background::FormerPlayer;
    } else if result == 9 {
        background = Background::LocalMagnate;
    } else if result == 10 {
        background = Background::RailroadBaron;
    } else if result == 11 {
        background = Background::MediaPersonality;
    } else if result == 12 {
        background = Background::RiverboatGambler;
    } else if result == 13 {
        background = Background::CaptainofIndustry;
    } else if result == 14 {
        background = Background::VentureCapitalist;
    } else if result == 15 {
        background = Background::MillionaireRecluse;
    } else if result == 16 {
        background = Background::NewspaperSyndicate;
    } else if result == 17 {
        background = Background::HeirtoPreviousOwner;
    } else if result == 18 {
        background = Background::RealEstateDeveloper;
    } else if result == 19 {
        background = Background::RollerCoasterTycoon;
    } else if result == 20 {
        background = Background::MultinationalCorporation;
    } else {
        background = Background::MillionaireRecluse;
    }

    return background;
}
*/

/// generate team function
// TODO: combine inputs - load all the csv databases into a vector or array, makes it easier to pass into functions
// probably need to be references as well
pub fn generate_team(
    era: Era,
    starters_num: u32,
    bench_num: u32,
    pitchers_num: u32,
    bullpen_num: u32,
    name: &str,
    firstnames: &Vec<String>,
    lastnames: &Vec<String>,
    logos: &Vec<String>,
    mascots: &Vec<String>,
    mottos: &Vec<String>,
    personalities: &Vec<String>,
    backgrounds: &Vec<String>,
    //locations: Vec<String>, // honestly I forget why this was here in the first place
    name1: &Vec<String>,
    name2: &Vec<String>,
) -> Team {
    // iterate over number of players
    let mut roster_raw: Vec<Player> = vec![];
    let mut bench_raw: Vec<Player> = vec![];
    let mut pitcher_raw: Vec<Player> = vec![];
    let mut bullpen_raw: Vec<Player> = vec![];
    let mut roster: Vec<String> = vec![];
    let mut bench: Vec<String> = vec![];
    let mut pitcher: Vec<String> = vec![];
    let mut bullpen: Vec<String> = vec![];
    let mut position: Position;
    // create player structs, then write to files - it's the filenames that need to be stored in
    // the team struct
    for i in 0..starters_num {
        // TODO: IDK what to do here, should always be 8 position players so I'm just going to hard
        // code for now
        if i == 0 {
            position = Position::Catcher;
        } else if i == 1 {
            position = Position::Firstbase;
        } else if i == 2 {
            position = Position::Secondbase;
        } else if i == 3 {
            position = Position::Shortstop;
        } else if i == 4 {
            position = Position::Thirdbase;
        } else if i == 5 {
            position = Position::Leftfield;
        } else if i == 6 {
            position = Position::Centerfield;
        } else if i == 7 {
            position = Position::Rightfield;
        } else {
            position = Position::Firstbase;
        }
        roster_raw.push(generate_player(
            super::players::PlayerClass::StartingHitter,
            //&era, // uncomment when reintroducing Era
            position,
            &firstnames,
            &lastnames,
        ));
        // write player struct, if file write is successful add it to the filename struct
        let mut file_name_str = PLAYER_LOCATION.to_owned();
        file_name_str.push_str(&roster_raw[i as usize].first_name);
        file_name_str.push_str("_");
        file_name_str.push_str(&roster_raw[i as usize].last_name);
        file_name_str.push_str(".dbp");
        let write_result = write_player(&roster_raw[i as usize], &file_name_str);
        match write_result {
            Ok(()) => roster.push(file_name_str),
            Err(_err) => println!("Error writing file: {}", file_name_str),
        }
    }

    for i in 0..bench_num as usize {
        bench_raw.push(generate_player(
            super::players::PlayerClass::PinchHitter,
            Position::None,
            &firstnames,
            &lastnames,
        ));
        // write player struct, if file write is successful add it to the filename struct
        let mut file_name_str = PLAYER_LOCATION.to_owned();
        file_name_str.push_str(&bench_raw[i].first_name);
        file_name_str.push_str("_");
        file_name_str.push_str(&bench_raw[i].last_name);
        file_name_str.push_str(".dbp");
        let write_result = write_player(&bench_raw[i], &file_name_str);
        match write_result {
            Ok(()) => bench.push(file_name_str),
            Err(_err) => println!("Error writing file: {}", file_name_str),
        }
    }

    for i in 0..pitchers_num as usize {
        pitcher_raw.push(generate_player(
            super::players::PlayerClass::Pitchers,
            Position::Pitcher,
            &firstnames,
            &lastnames,
        ));
        // write player struct, if file write is successful add it to the filename struct
        let mut file_name_str = PLAYER_LOCATION.to_owned();
        file_name_str.push_str(&pitcher_raw[i].first_name);
        file_name_str.push_str("_");
        file_name_str.push_str(&pitcher_raw[i].last_name);
        file_name_str.push_str(".dbp");
        let write_result = write_player(&pitcher_raw[i], &file_name_str);
        match write_result {
            Ok(()) => pitcher.push(file_name_str),
            Err(_err) => println!("Error writing file: {}", file_name_str),
        }
    }

    for i in 0..bullpen_num as usize {
        bullpen_raw.push(generate_player(
            super::players::PlayerClass::Pitchers,
            Position::Pitcher,
            &firstnames,
            &lastnames,
        ));
        // write player struct, if file write is successful add it to the filename struct
        let mut file_name_str = PLAYER_LOCATION.to_owned();
        file_name_str.push_str(&bullpen_raw[i].first_name);
        file_name_str.push_str("_");
        file_name_str.push_str(&bullpen_raw[i].last_name);
        file_name_str.push_str(".dbp");
        let write_result = write_player(&bullpen_raw[i], &file_name_str);
        match write_result {
            Ok(()) => bullpen.push(file_name_str),
            Err(_err) => println!("Error writing file: {}", file_name_str),
        }
    }

    // manager details
    let manager_name = generate_manager(firstnames, lastnames);
    let manager_position = generate_manager_position();
    let manager_league = generate_league(&manager_position);

    // team details
    let years_in_league = roll(100);
    let years_since_championship = roll(years_in_league);

    // generate ballpark structure then write it to file
    let mut ballpark_string: String = String::new();

    match era {
        Era::Modern => {
            // ballpark details
            let ballpark = generate_modern_ballpark(name1, name2);
            // build file name string
            let mut file_name_str = BALLPARK_LOCATION.to_owned();
            file_name_str.push_str(&ballpark.name);
            file_name_str.push_str(".dbb");
            let write_result = write_ballpark_modern(&ballpark, &file_name_str);
            match write_result {
                Ok(()) => ballpark_string.push_str(&file_name_str),
                Err(_err) => ballpark_string.push_str("src/testfiles/railyard.dbb"),
            }
        }
        Era::Ancient => {
            let ballpark = generate_ancient_ballpark(name1, name2);
            // build file name string
            let mut file_name_str = BALLPARK_LOCATION.to_owned();
            file_name_str.push_str(&ballpark.name);
            file_name_str.push_str(".dbb");
            let write_result = write_ballpark_ancient(&ballpark, &file_name_str);
            match write_result {
                Ok(()) => ballpark_string.push_str(&file_name_str),
                Err(_err) => ballpark_string.push_str("src/testfiles/mayfair_park.dbb"),
            }
        }
        Era::None => {
            let ballpark = generate_ancient_ballpark(name1, name2);
            // build file_name_str
            let mut file_name_str = BALLPARK_LOCATION.to_owned();
            file_name_str.push_str(&ballpark.name);
            file_name_str.push_str(".dbb");
            let write_result = write_ballpark_ancient(&ballpark, &file_name_str);
            match write_result {
                Ok(()) => ballpark_string.push_str(&file_name_str),
                Err(_err) => ballpark_string.push_str("src/testfiles/mayfair_park.dbb"),
            }
        }
    }

    // build team struct
    let new_team = Team {
        name: name.to_string(),
        ballpark: ballpark_string, // TODO: auto generate or user define
        manager: manager_name,
        logo: generate_logo(logos),
        era,
        location: generate_location(),
        mascot: generate_mascot(mascots),
        priority: generate_priority(),
        makeup: generate_makeup(),
        years: years_in_league,
        championship: years_since_championship,
        fanbase: generate_fanbase(),
        manager_position,
        manager_league,
        retired: generate_retired(),
        personality: generate_personality(&personalities),
        daring: roll(20),
        motto: generate_motto(mottos),
        owner_background: generate_background(backgrounds),
        owner_personality: generate_personality(&personalities),
        roster,
        bench,
        pitcher,
        bullpen,
    };

    return new_team;
}
