use std::fs;

use text_colorizer::Colorize;

use crate::core::roll;

use super::teams::{generate_location, Era, Location};

/*==========================================
ENUM DEFINITIONS
==========================================*/
#[derive(Clone)]
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

#[derive(Clone)]
pub enum Turf {
    Ragged,
    Good,
    Artificial,
    None,
}

#[derive(Clone)]
pub enum Roof {
    No,
    Permanent,
    Retractable,
    None,
}

#[derive(Clone)]
pub enum Condition {
    FallingApart,
    Decrepit,
    WellWorn,
    Sparkling,
    None,
}

#[derive(Clone)]
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

/*==========================================
STRUCTURES
==========================================*/
#[derive(Clone)]
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

/// load modern park baseball file
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

    let rows: Vec<&str> = contents.split('\n').collect();
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
                roof = Roof::No;
            } else if rowline[1].trim().eq("Permanent Roof") {
                roof = Roof::Permanent;
            } else if rowline[1].trim().eq("Retractable Roof") {
                roof = Roof::Retractable;
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

    BallparkModern {
        name,
        location,
        park_type,
        capacity,
        turf,
        roof,
        condition,
        quirks,
    }
}

/// writes modern park struct to text file
pub fn write_ballpark_modern(data: &BallparkModern, filename: &str) -> Result<(), std::io::Error> {
    let mut file_text = String::new();
    file_text.push_str("NAME: ");
    file_text.push_str(&data.name);
    file_text.push_str("\nLOCATION: ");
    match data.location {
        Location::None => file_text.push_str("None"),
        Location::SmallTown => file_text.push_str("Small Town"),
        Location::SmallCity => file_text.push_str("Small City"),
        Location::Metropolis => file_text.push_str("Metropolis"),
        Location::MiddleOfNowhere => file_text.push_str("Middle Of Nowhere"),
        Location::MediumSizedCity => file_text.push_str("Medium Sized City"),
    }
    file_text.push_str("\nTYPE: ");
    match data.park_type {
        StadiumTypeModern::None => file_text.push_str("None"),
        StadiumTypeModern::BaseballPalace => file_text.push_str("Baseball Palace"),
        StadiumTypeModern::Retro => file_text.push_str("Retro"),
        StadiumTypeModern::JewelBox => file_text.push_str("Jewel Box"),
        StadiumTypeModern::SpaceAge => file_text.push_str("Space Age"),
        StadiumTypeModern::ConcreteDonut => file_text.push_str("Concrete Donot"),
    }
    file_text.push_str("\nCAPACITY: ");
    file_text.push_str(&data.capacity.to_string());
    file_text.push_str("\nTURF: ");
    match data.turf {
        Turf::None => file_text.push_str("None"),
        Turf::Good => file_text.push_str("Good"),
        Turf::Ragged => file_text.push_str("Ragged"),
        Turf::Artificial => file_text.push_str("Artificial"),
    }
    file_text.push_str("\nROOF: ");
    match data.roof {
        Roof::None => file_text.push_str("None"),
        Roof::No => file_text.push_str("No Roof"),
        Roof::Permanent => file_text.push_str("Permanent Roof"),
        Roof::Retractable => file_text.push_str("Retractable Roof"),
    }
    file_text.push_str("\nCONDITION: ");
    match data.condition {
        Condition::None => file_text.push_str("None"),
        Condition::Decrepit => file_text.push_str("Decrepit"),
        Condition::WellWorn => file_text.push_str("Well Worn"),
        Condition::Sparkling => file_text.push_str("Sparkling"),
        Condition::FallingApart => file_text.push_str("Falling Apart"),
    }
    file_text.push_str("\nQUIRKS: ");
    for i in 0..data.quirks.len() {
        match data.quirks[i] {
            Quirks::None => {}
            Quirks::SlowInfield => file_text.push_str(" Slow Infield,"),
            Quirks::OddLeft => file_text.push_str(" Odd Left,"),
            Quirks::Hideous => file_text.push_str(" Hideous,"),
            Quirks::OddRight => file_text.push_str(" Odd Right,"),
            Quirks::ShortLeft => file_text.push_str(" Short Left,"),
            Quirks::OddCenter => file_text.push_str(" Odd Center,"),
            Quirks::HighMound => file_text.push_str(" High Mound,"),
            Quirks::Beautiful => file_text.push_str(" Beautiful,"),
            Quirks::ShortRight => file_text.push_str(" Short Right,"),
            Quirks::FastInfield => file_text.push_str(" Fast Infield,"),
            Quirks::CozyOutfield => file_text.push_str(" Cozy Outfield,"),
            Quirks::ExpansiveOutfield => file_text.push_str(" Expansive Outfield,"),
        }
    }

    let write_result = fs::write(filename, &file_text);
    return write_result;
}

/// loads ancient era park from text file to struct
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
        name,
        location,
        park_type,
        capacity,
        condition,
        quirks,
    };

    park_data
}

/// writes ancient era ballpark struct to text file
pub fn write_ballpark_ancient(
    data: &BallparkAncient,
    filename: &str,
) -> Result<(), std::io::Error> {
    let mut file_text = String::new();
    file_text.push_str("NAME: ");
    file_text.push_str(&data.name);
    file_text.push_str("\nLOCATION: ");
    match data.location {
        Location::None => file_text.push_str("None"),
        Location::SmallTown => file_text.push_str("Small Town"),
        Location::SmallCity => file_text.push_str("Small City"),
        Location::Metropolis => file_text.push_str("Metropolis"),
        Location::MiddleOfNowhere => file_text.push_str("Middle Of Nowhere"),
        Location::MediumSizedCity => file_text.push_str("Medium Sized City"),
    }
    file_text.push_str("\nTYPE: ");
    match data.park_type {
        StadiumTypeAncient::None => file_text.push_str("None"),
        StadiumTypeAncient::JewelBox => file_text.push_str("Jewel Box"),
        StadiumTypeAncient::BaseballPalace => file_text.push_str("Baseball Palace"),
        StadiumTypeAncient::WoodFramePavilion => file_text.push_str("Wood Frame Pavilion"),
    }
    file_text.push_str("\nCAPACITY: ");
    file_text.push_str(&data.capacity.to_string());
    file_text.push_str("\nCONDITION: ");
    match data.condition {
        Condition::None => file_text.push_str("None"),
        Condition::Decrepit => file_text.push_str("Decrepit"),
        Condition::WellWorn => file_text.push_str("Well Worn"),
        Condition::Sparkling => file_text.push_str("Sparkling"),
        Condition::FallingApart => file_text.push_str("Falling Apart"),
    }
    file_text.push_str("\nQUIRKS: ");
    for i in 0..data.quirks.len() {
        match data.quirks[i] {
            Quirks::None => {}
            Quirks::SlowInfield => file_text.push_str(" Slow Infield,"),
            Quirks::OddLeft => file_text.push_str(" Odd Left,"),
            Quirks::Hideous => file_text.push_str(" Hideous,"),
            Quirks::OddRight => file_text.push_str(" Odd Right,"),
            Quirks::ShortLeft => file_text.push_str(" Short Left,"),
            Quirks::OddCenter => file_text.push_str(" Odd Center,"),
            Quirks::HighMound => file_text.push_str(" High Mound,"),
            Quirks::Beautiful => file_text.push_str(" Beautiful,"),
            Quirks::ShortRight => file_text.push_str(" Short Right,"),
            Quirks::FastInfield => file_text.push_str(" Fast Infield,"),
            Quirks::CozyOutfield => file_text.push_str(" Cozy Outfield,"),
            Quirks::ExpansiveOutfield => file_text.push_str(" Expansive Outfield,"),
        }
    }

    let write_result = fs::write(filename, &file_text);
    return write_result;
}

/// generate ballpark names - two words, CSV for each? some kind of name and then park type
pub fn generate_ballpark_name(name1: &Vec<String>, name2: &Vec<String>) -> String {
    let len1 = name1.len();
    let len2 = name2.len();
    let roll1 = roll(len1 as i32) - 1; // NOTE: -1 for array indexing
    let roll2 = roll(len2 as i32) - 1;
    let part1 = name1[roll1 as usize].clone();
    let part2 = name2[roll2 as usize].clone();
    let name = part1 + " " + &part2;
    return name;
}

/// generate park type functions
pub fn generate_ancient_park_type() -> StadiumTypeAncient {
    let result = roll(3);
    let park_type: StadiumTypeAncient;
    if result == 1 {
        park_type = StadiumTypeAncient::BaseballPalace;
    } else if result == 2 {
        park_type = StadiumTypeAncient::JewelBox;
    } else if result == 3 {
        park_type = StadiumTypeAncient::WoodFramePavilion;
    } else {
        park_type = StadiumTypeAncient::None;
    }

    return park_type;
}

/// generate a modern park type
pub fn generate_modern_park_type() -> StadiumTypeModern {
    let result = roll(5);
    let park_type: StadiumTypeModern;
    if result == 1 {
        park_type = StadiumTypeModern::Retro;
    } else if result == 2 {
        park_type = StadiumTypeModern::JewelBox;
    } else if result == 3 {
        park_type = StadiumTypeModern::BaseballPalace;
    } else if result == 4 {
        park_type = StadiumTypeModern::SpaceAge;
    } else if result == 5 {
        park_type = StadiumTypeModern::ConcreteDonut;
    } else {
        park_type = StadiumTypeModern::None;
    }

    return park_type;
}

/// generate condition function
pub fn generate_ballpark_condition(era: Era) -> Condition {
    let mut result = roll(20);
    let condition: Condition;
    match era {
        Era::Ancient => result -= 1,
        _ => {}
    }
    if result == 0 {
        condition = Condition::FallingApart;
    } else if result >= 1 && result <= 6 {
        condition = Condition::Decrepit;
    } else if result >= 7 && result <= 15 {
        condition = Condition::WellWorn;
    } else {
        condition = Condition::Sparkling;
    }

    return condition;
}

/// generate turf function
pub fn generate_turf() -> Turf {
    let result = roll(20);
    let turf: Turf;
    // TODO: make turf impact steals, etc.
    if result <= 2 {
        turf = Turf::Ragged;
        // -1 to steal and infield defense
    } else if result >= 3 && result <= 10 {
        turf = Turf::Good;
    } else {
        turf = Turf::Artificial;
        // +1 to steal and infield defense
    }

    return turf;
}

/// generate roof function
pub fn generate_roof() -> Roof {
    let result = roll(20);
    let roof: Roof;
    // TODO: make roof impact play
    if result <= 13 {
        roof = Roof::No;
    } else if result >= 14 && result <= 15 {
        roof = Roof::Permanent;
    } else {
        roof = Roof::Retractable;
    }

    return roof;
}

/// generate ballpark quirks functions
// TODO: make quirks impact play
pub fn generate_quirks(quirk_num: i32) -> Vec<Quirks> {
    let mut quirks: Vec<Quirks> = vec![];
    if quirk_num == 0 {
        quirks.push(Quirks::None);
    } else {
        for _i in 0..quirk_num {
            let result = roll(20);
            if result <= 3 {
                quirks.push(Quirks::CozyOutfield);
            } else if result >= 4 && result <= 6 {
                quirks.push(Quirks::ExpansiveOutfield);
            } else if result == 7 {
                quirks.push(Quirks::ShortLeft);
            } else if result == 8 {
                quirks.push(Quirks::ShortRight);
            } else if result == 9 {
                quirks.push(Quirks::OddLeft);
            } else if result == 10 {
                quirks.push(Quirks::OddCenter);
            } else if result == 11 {
                quirks.push(Quirks::OddRight);
            } else if result == 12 {
                quirks.push(Quirks::FastInfield);
            } else if result == 13 {
                quirks.push(Quirks::SlowInfield);
            } else if result == 14 {
                quirks.push(Quirks::HighMound);
            } else if result >= 15 && result <= 17 {
                quirks.push(Quirks::Beautiful);
            } else {
                quirks.push(Quirks::Hideous);
            }
        }
    }

    return quirks;
}

/// generate ballpark functions
pub fn generate_ancient_ballpark(name1: &Vec<String>, name2: &Vec<String>) -> BallparkAncient {
    // generate info
    let park_type = generate_ancient_park_type();
    let capacity: i32;
    let quirk_num: i32;
    match park_type {
        StadiumTypeAncient::WoodFramePavilion => {
            capacity = 5000;
            quirk_num = 2;
        }
        StadiumTypeAncient::JewelBox => {
            capacity = 35000;
            quirk_num = 1;
        }
        StadiumTypeAncient::BaseballPalace => {
            capacity = 50000;
            quirk_num = 0;
        }
        StadiumTypeAncient::None => {
            capacity = 0;
            quirk_num = 0;
        }
    }
    let condition = generate_ballpark_condition(Era::Ancient);
    // TODO: influence fanbase
    // quirk roll - match stadium type for number of rolls

    // build struct
    let ballpark = BallparkAncient {
        name: generate_ballpark_name(name1, name2),
        location: generate_location(),
        park_type,
        capacity,
        condition,
        quirks: generate_quirks(quirk_num),
    };
    return ballpark;
}

/// generate a modern ballpark struct
pub fn generate_modern_ballpark(name1: &Vec<String>, name2: &Vec<String>) -> BallparkModern {
    // generate info
    let park_type = generate_modern_park_type();
    let capacity: i32;
    let turf: Turf;
    let roof: Roof;
    let quirk_num: i32;
    match park_type {
        StadiumTypeModern::None => {
            capacity = 0;
            turf = Turf::Good;
            roof = Roof::None;
            quirk_num = 0;
        }
        StadiumTypeModern::Retro => {
            capacity = 38000;
            turf = Turf::Good;
            roof = generate_roof();
            quirk_num = 1;
        }
        StadiumTypeModern::JewelBox => {
            capacity = 35000;
            turf = Turf::Good;
            roof = Roof::None;
            quirk_num = 1;
        }
        StadiumTypeModern::SpaceAge => {
            capacity = 50000;
            turf = Turf::Good;
            roof = generate_roof();
            quirk_num = 0;
        }
        StadiumTypeModern::ConcreteDonut => {
            capacity = 55000;
            turf = generate_turf();
            roof = generate_roof();
            quirk_num = 0;
            // TODO: generate turf for other stadium types???
        }
        StadiumTypeModern::BaseballPalace => {
            capacity = 50000;
            turf = Turf::Good;
            roof = Roof::None;
            quirk_num = 0;
        }
    }
    let condition = generate_ballpark_condition(Era::Modern);
    // TODO: influence fanbase
    // quirk roll - match stadium type for number of rolls

    // build struct
    let ballpark = BallparkModern {
        name: generate_ballpark_name(name1, name2),
        location: generate_location(),
        park_type: generate_modern_park_type(),
        capacity,
        turf,
        roof,
        condition,
        quirks: generate_quirks(quirk_num),
    };
    return ballpark;
}
