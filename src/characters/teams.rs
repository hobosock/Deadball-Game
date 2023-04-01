/*==========================================
MODULE INCLUSIONS
==========================================*/
use text_colorizer::*;

use super::players::Position;

/*==========================================
ENUM DEFINITIONS
==========================================*/
// TEAM ENUMS
pub enum Era {
    Ancient,
    Modern,
    None,
}

pub enum Location {
    MiddleOfNowhere,
    SmallTown,
    SmallCity,
    MediumSizedCity,
    Metropolis,
    None,
}

pub enum Priority {
    Power,
    Average,
    StartingPitching,
    Bullpen,
    Speed,
    Defense,
    None,
}

pub enum Makeup {
    MostlyProspects,
    Balanced,
    MostlyVeterans,
    None,
}

// championship - I don't think this is needed
// mascot
// years in league
// owner background
// owner personality
// fanbase
// park name
// park Location

pub enum StadiumTypeModern {
    JewelBox,
    BaseballPalace,
    SpaceAge,
    ConcreteDonut,
    Retro,
    None,
}

pub enum StadiumTypeAncient {
    WoodFramePavilion,
    JewelBox,
    BaseballPalace,
    None,
}

pub enum Turf {
    Ragged,
    Good,
    Artificial,
    None,
}

pub enum Roof {
    NoRoof,
    PermanentRoof,
    RetractableRoof,
    None,
}

pub enum Condition {
    FallingApart,
    Decrepit,
    WellWorn,
    Sparkling,
    None,
}

pub enum Quirks {
    CozyOutfield,
    ExpansiveOutfield,
    ShortLeft,
    ShortRight,
    OddLeft,
    OddCenter,
    OddRight,
    FastInfield,
    SlowInfield,
    HighMound,
    Beautiful,
    Hideous,
    None,
}

pub enum Fanbase {
    Nonexistent,
    Indifferent,
    FairWeather,
    Loyal,
    Obsessive,
    None,
}

// Manager
pub enum ManagerLeague {
    Major,
    Minor,
    None,
}

/*==========================================
STRUCTURES
==========================================*/
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
}

pub struct BallparkModern {
    pub name: String,
    pub location: String,
    pub park_type: StadiumTypeModern,
    pub capacity: i32,
    pub turf: Turf,
    pub roof: Roof,
    pub condition: Condition,
    pub quirks: Quirks,
}

pub struct BallparkAncient {
    pub name: String,
    pub location: String,
    pub park_type: StadiumTypeAncient,
    pub capacity: i32,
    pub condition: Condition,
    pub quirks: Quirks,
}

/*==========================================
FUNCTIONS
==========================================*/

// load team file *.DBT
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
    let mut owner_background = String::new();
    let mut owner_personality = String::new();

    // sort text into relevant fields
    let rows: Vec<&str> = contents.split("\n").collect();
    for i in 0..rows.len() - 1 {
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
        }
    }

    let team_data = Team {
        name: name,
        ballpark: ballpark,
        manager: manager,
        logo: logo,
        era: era,
        location: location,
        mascot: mascot,
        priority: priority,
        makeup: makeup,
        years: years,
        championship: championship,
        fanbase: fanbase,
        manager_position: manager_position,
        manager_league: manager_league,
        retired: retired,
        personality: personality,
        daring: daring,
        motto: motto,
        owner_background: owner_background,
        owner_personality: owner_personality,
        roster: roster,
    };

    team_data
}
