/*==========================================
MODULE INCLUSIONS
==========================================*/

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
}

pub enum Priority {
    Power,
    Average,
    StartingPitching,
    Bullpen,
    Speed,
    Defense,
}

pub enum Makeup {
    MostlyProspects,
    Balanced,
    MostlyVeterans,
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
}

pub enum StadiumTypeAncient {
    WoodFramePavilion,
    JewelBox,
    BaseballPalace,
}

pub enum Turf {
    Ragged,
    Good,
    Artificial,
}

pub enum Roof {
    NoRoof,
    PermanentRoof,
    RetractableRoof,
}

pub enum Condition {
    FallingApart,
    Decrepit,
    WellWorn,
    Sparkling,
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
    pub location: String,
    pub mascot: String,
    pub priority: Priority,
    pub makeup: Makeup,
    pub years: i32,
    pub championship: i32,
    pub fanbase: i32,
    pub manager_name: String,
    pub manager_position: Position,
    pub manager_league: ManagerLeague,
    pub retired: i32,
    pub personality: String, // could be an ENUM if you want
    pub daring: i32,
    pub motto: String,
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
    //
}
