/*========================================================
MODULE INCLUSIONS
========================================================*/
use std::fs; // needed to read in files

/*========================================================
ENUM DEFINITIONS
========================================================*/
#[derive(Debug)]
pub enum Position {
    pitcher,
    catcher,
    firstbase,
    secondbase,
    shortstop,
    thirdbase,
    rightfield,
    centerfield,
    leftfield,
}

#[derive(Debug)]
pub enum Handedness {
    right,
    left,
    switch,
}

#[derive(Debug)]
pub enum Traits {
    // hitter traits
    powerHitter,
    elitePowerHitter,
    contactHitter,
    speedyRunner,
    greatDefender,
    toughPlayer,
    weakHitter,
    extraWeakHitter,
    freeSwinger,
    slowRunner,
    poorDefender,

    // pitcher traits
    strikeoutArtist,
    groundballMachine,
    controlPitcher,
    greatStamina,
    wild,
}

#[derive(Debug)]
pub enum InjuryLocation {
    head,
    shoulder,
    elbow,
    forearm,
    wrist,
    hand,
    back,
    oblique,
    hip,
    hamstring,
    knee,
    ankle,
    foot,
    none,
}

#[derive(Debug)]
pub enum InjurySeverity {
    catastrophic,
    major,
    minor,
    superficial,
    uninjured,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
#[derive(Debug)]
pub struct Player {
    pub firstName: String,
    pub lastName: String,
    pub nickname: String,
    pub position: Position,
    pub handedness: Handedness,
    pub batterTarget: i32,
    pub onBaseTarget: i32,
    pub pitchDie: i32,
    pub traits: Vec<Traits>,
    pub injuryLocation: Vec<InjuryLocation>,
    pub injurySeverity: Vec<InjurySeverity>,
}

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn loadPlayer(playerFilePath: String) -> Player {
    // load file
    let contents = fs::read_to_string(playerFilePath)
        .expect("Failed to read player file.");

    // sort data into player struct
    let lines: Vec<&str> = contents.split("\n").collect();
    //let readFirstName: String = (lines[0].split(":").collect::Vec<&Str>())[1].trim().to_string();
    let tempFirstName: Vec<&str> = lines[0].split(":").collect();
    let readFirstName = tempFirstName[1].trim().to_string();

    // temporary
    let readLastName = "Loveall".to_string();
    let readNickname = "none".to_string();
    let readPosition = Position::shortstop;
    let readHandedness = Handedness::right;
    let readBatterTarget = 32;
    let readOnBaseTarget = 40;
    let readPitchDie = -8;
    let readTraits = vec![Traits::contactHitter];
    let readInjuryLocation = vec![InjuryLocation::none];
    let readInjurySeverity = vec![InjurySeverity::uninjured];

    let playerData = Player {
        firstName: readFirstName,
        lastName: readLastName,
        nickname: readNickname,
        position: readPosition,
        handedness: readHandedness,
        batterTarget: readBatterTarget,
        onBaseTarget: readOnBaseTarget,
        pitchDie: readPitchDie,
        traits: readTraits,
        injuryLocation: readInjuryLocation,
        injurySeverity: readInjurySeverity,
    };

    playerData
}