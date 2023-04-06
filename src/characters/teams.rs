/*==========================================
MODULE INCLUSIONS
==========================================*/
use std::fs;
use text_colorizer::*;

use super::players::{load_player, Player, Position};

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
    pub bench: Vec<String>,
    pub pitcher: Vec<String>,
    pub bullpen: Vec<String>,
}

pub struct BallparkModern {
    pub name: String,
    pub location: Location,
    pub park_type: StadiumTypeModern,
    pub capacity: i32,
    pub turf: Turf,
    pub roof: Roof,
    pub condition: Condition,
    pub quirks: Vec<Quirks>,
}

pub struct BallparkAncient {
    pub name: String,
    pub location: Location,
    pub park_type: StadiumTypeAncient,
    pub capacity: i32,
    pub condition: Condition,
    pub quirks: Vec<Quirks>,
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
        bench: bench,
        pitcher: pitcher,
        bullpen: bullpen,
    };

    team_data
}

// write team file *.DBT
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

pub fn load_park_modern(contents: String) -> BallparkModern {
    // initialize fields
    let mut name = String::new();
    let mut location = Location::None;
    let mut park_type = StadiumTypeModern::None;
    let mut capacity: i32 = 0;
    let mut turf = Turf::None;
    let mut roof = Roof::None;
    let mut condition = Condition::None;
    let mut quirks = vec![Quirks::None];

    let rows: Vec<&str> = contents.split("\n").collect();
    for i in 0..rows.len() - 1 {
        // last line is usually just a new line character
        let rowline: Vec<&str> = rows[i].split(":").collect();
        if rowline[0].trim().eq("NAME") {
            name = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("LOCATION") {
            if rowline[1].trim().eq("Middle Of Nowhere") {
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
        } else if rowline[0].trim().eq("TYPE") {
            if rowline[1].trim().eq("Jewel Box") {
                park_type = StadiumTypeModern::JewelBox;
            } else if rowline[1].trim().eq("Baseball Palace") {
                park_type = StadiumTypeModern::BaseballPalace;
            } else if rowline[1].trim().eq("Space Age") {
                park_type = StadiumTypeModern::SpaceAge;
            } else if rowline[1].trim().eq("Concrete Donut") {
                park_type = StadiumTypeModern::ConcreteDonut;
            } else if rowline[1].trim().eq("Retro") {
                park_type = StadiumTypeModern::Retro;
            }
        } else if rowline[0].trim().eq("CAPACITY") {
            let capacity_result = rowline[1].trim().parse();
            match capacity_result {
                Ok(cap) => capacity = cap,
                Err(_err) => println!("{}", "Failed to convert 'capacity' number.".red().bold()),
            }
        } else if rowline[0].trim().eq("TURF") {
            if rowline[1].trim().eq("Good") {
                turf = Turf::Good;
            } else if rowline[1].trim().eq("Ragged") {
                turf = Turf::Ragged;
            } else if rowline[1].trim().eq("Artificial") {
                turf = Turf::Artificial;
            }
        } else if rowline[0].trim().eq("ROOF") {
            if rowline[1].trim().eq("No Roof") {
                roof = Roof::NoRoof;
            } else if rowline[1].trim().eq("Permanent Roof") {
                roof = Roof::PermanentRoof;
            } else if rowline[1].trim().eq("Retractable Roof") {
                roof = Roof::RetractableRoof;
            }
        } else if rowline[0].trim().eq("CONDITION") {
            if rowline[1].trim().eq("Well Worn") {
                condition = Condition::WellWorn;
            } else if rowline[1].trim().eq("Decrepit") {
                condition = Condition::Decrepit;
            } else if rowline[1].trim().eq("Sparkling") {
                condition = Condition::Sparkling;
            } else if rowline[1].trim().eq("Falling Apart") {
                condition = Condition::FallingApart;
            }
        } else if rowline[0].trim().eq("QUIRKS") {
            let quirk_string: Vec<&str> = rowline[1].split(",").collect();
            for i in 0..quirk_string.len() {
                if quirk_string[i].trim().eq("Cozy Outfield") {
                    quirks.push(Quirks::CozyOutfield);
                } else if quirk_string[i].trim().eq("Expansive Outfield") {
                    quirks.push(Quirks::ExpansiveOutfield);
                } else if quirk_string[i].trim().eq("Short Left") {
                    quirks.push(Quirks::ShortLeft);
                } else if quirk_string[i].trim().eq("Short Right") {
                    quirks.push(Quirks::ShortRight);
                } else if quirk_string[i].trim().eq("Odd Left") {
                    quirks.push(Quirks::OddLeft);
                } else if quirk_string[i].trim().eq("Odd Center") {
                    quirks.push(Quirks::OddCenter);
                } else if quirk_string[i].trim().eq("Odd Right") {
                    quirks.push(Quirks::OddRight);
                } else if quirk_string[i].trim().eq("Fast Infield") {
                    quirks.push(Quirks::FastInfield);
                } else if quirk_string[i].trim().eq("Slow Infield") {
                    quirks.push(Quirks::SlowInfield);
                } else if quirk_string[i].trim().eq("High Mound") {
                    quirks.push(Quirks::HighMound);
                } else if quirk_string[i].trim().eq("Beautiful") {
                    quirks.push(Quirks::Beautiful);
                } else if quirk_string[i].trim().eq("Hideous") {
                    quirks.push(Quirks::Hideous);
                }
            }
        }
    }

    let park_data = BallparkModern {
        name: name,
        location: location,
        park_type: park_type,
        capacity: capacity,
        turf: turf,
        roof: roof,
        condition: condition,
        quirks: quirks,
    };

    park_data
}

pub fn load_park_ancient(contents: String) -> BallparkAncient {
    // initialize fields
    let mut name = String::new();
    let mut location = Location::None;
    let mut park_type = StadiumTypeAncient::None;
    let mut capacity: i32 = 0;
    let mut condition = Condition::None;
    let mut quirks = vec![Quirks::None];

    let rows: Vec<&str> = contents.split("\n").collect();
    for i in 0..rows.len() - 1 {
        let rowline: Vec<&str> = rows[i].split(":").collect();
        if rowline[0].trim().eq("NAME") {
            name = rowline[1].trim().to_string();
        } else if rowline[0].trim().eq("LOCATION") {
            if rowline[1].trim().eq("Middle Of Nowhere") {
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
        } else if rowline[0].trim().eq("TYPE") {
            if rowline[1].trim().eq("Jewel Box") {
                park_type = StadiumTypeAncient::JewelBox;
            } else if rowline[1].trim().eq("Baseball Palace") {
                park_type = StadiumTypeAncient::BaseballPalace;
            } else if rowline[1].trim().eq("Wood Frame Pavilion") {
                park_type = StadiumTypeAncient::WoodFramePavilion;
            }
        } else if rowline[0].trim().eq("CAPACITY") {
            let capacity_result = rowline[1].trim().parse();
            match capacity_result {
                Ok(cap) => capacity = cap,
                Err(_err) => println!("{}", "Failed to convert 'capacity' number.".red().bold()),
            }
        } else if rowline[0].trim().eq("CONDITION") {
            if rowline[1].trim().eq("Well Worn") {
                condition = Condition::WellWorn;
            } else if rowline[1].trim().eq("Decrepit") {
                condition = Condition::Decrepit;
            } else if rowline[1].trim().eq("Sparkling") {
                condition = Condition::Sparkling;
            } else if rowline[1].trim().eq("Falling Apart") {
                condition = Condition::FallingApart;
            }
        } else if rowline[0].trim().eq("QUIRKS") {
            let quirk_string: Vec<&str> = rowline[1].split(",").collect();
            for i in 0..quirk_string.len() {
                if quirk_string[1].trim().eq("Cozy Outfield") {
                    quirks.push(Quirks::CozyOutfield);
                } else if quirk_string[i].trim().eq("Expansive Outfield") {
                    quirks.push(Quirks::ExpansiveOutfield);
                } else if quirk_string[i].trim().eq("Short Left") {
                    quirks.push(Quirks::ShortLeft);
                } else if quirk_string[i].trim().eq("Short Right") {
                    quirks.push(Quirks::ShortRight);
                } else if quirk_string[i].trim().eq("Odd Left") {
                    quirks.push(Quirks::OddLeft);
                } else if quirk_string[i].trim().eq("Odd Center") {
                    quirks.push(Quirks::OddCenter);
                } else if quirk_string[i].trim().eq("Odd Right") {
                    quirks.push(Quirks::OddRight);
                } else if quirk_string[i].trim().eq("Fast Infield") {
                    quirks.push(Quirks::FastInfield);
                } else if quirk_string[i].trim().eq("Slow Infield") {
                    quirks.push(Quirks::SlowInfield);
                } else if quirk_string[i].trim().eq("High Mound") {
                    quirks.push(Quirks::HighMound);
                } else if quirk_string[i].trim().eq("Beautiful") {
                    quirks.push(Quirks::Beautiful);
                } else if quirk_string[i].trim().eq("Hideous") {
                    quirks.push(Quirks::Hideous);
                }
            }
        }
    }

    let park_data = BallparkAncient {
        name: name,
        location: location,
        park_type: park_type,
        capacity: capacity,
        condition: condition,
        quirks: quirks,
    };

    park_data
}

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
