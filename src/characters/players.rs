/*========================================================
MODULE INCLUSIONS
========================================================*/
use std::fs; // needed to read in files

use text_colorizer::*;

//use super::teams::Era;
use crate::core::roll;

/*========================================================
ENUM DEFINITIONS
========================================================*/
#[derive(Debug, Clone)]
pub enum Position {
    Pitcher,
    Catcher,
    Firstbase,
    Secondbase,
    Shortstop,
    Thirdbase,
    Rightfield,
    Centerfield,
    Leftfield,
    Bench,
    None,
}

#[derive(Debug, Clone)]
pub enum Handedness {
    Right,
    Left,
    Switch,
    None,
}

#[derive(Debug, Clone)]
pub enum Traits {
    // hitter traits
    PowerHitter,
    ElitePowerHitter,
    ContactHitter,
    SpeedyRunner,
    GreatDefender,
    ToughPlayer,
    WeakHitter,
    ExtraWeakHitter,
    FreeSwinger,
    SlowRunner,
    PoorDefender,

    // pitcher traits
    StrikeoutArtist,
    GroundballMachine,
    ControlPitcher,
    GreatStamina,
    Wild,

    None,
}

#[derive(Debug, Clone)]
pub enum InjuryLocation {
    Head,
    Shoulder,
    Elbow,
    Forearm,
    Wrist,
    Hand,
    Back,
    Oblique,
    Hip,
    Hamstring,
    Knee,
    Ankle,
    Foot,
    None,
}

#[derive(Debug, Clone)]
pub enum InjurySeverity {
    Catastrophic,
    Major,
    Minor,
    Superficial,
    Uninjured,
}

// this is used for team generation purposes, starting players get better stats, etc.
#[derive(Debug)]
pub enum PlayerClass {
    StartingHitter,
    PinchHitter,
    Pitchers,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
#[derive(Debug, Clone)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub position: Position,
    pub handedness: Handedness,
    pub batter_target: i32,
    pub on_base_target: i32,
    pub pitch_die: i32,
    pub traits: Vec<Traits>,
    pub injury_location: Vec<InjuryLocation>,
    pub injury_severity: Vec<InjurySeverity>,
}

/*========================================================
FUNCTION DEFINITIONS
========================================================*/

// loads a *.DBP file and converts to Player struct
pub fn load_player(contents: String) -> Player {
    // initialize player data
    let mut read_first_name = String::new();
    let mut read_last_name = String::new();
    let mut read_nickname = String::new();
    let mut read_position = Position::None;
    let mut read_handedness = Handedness::None;
    let mut read_batter_target: i32 = 0;
    let mut read_on_base_target: i32 = 0;
    let mut read_pitch_die: i32 = 0;
    let mut read_traits = vec![Traits::None];
    let mut read_injury_location = vec![InjuryLocation::None];
    let mut read_injury_severity = vec![InjurySeverity::Uninjured];

    // sort data into player struct
    let stats: Vec<&str> = contents.split("\n").collect();
    for i in 0..stats.len() - 1 {
        // last line is usually just a new line character
        let statline: Vec<&str> = stats[i].split(":").collect();
        if statline[0].trim().eq("First Name") {
            read_first_name = statline[1].trim().to_string();
        } else if statline[0].trim().eq("Last Name") {
            read_last_name = statline[1].trim().to_string();
        } else if statline[0].trim().eq("Nickname") {
            read_nickname = statline[1].trim().to_string();
        } else if statline[0].trim().eq("Position") {
            let pos_str = statline[1];
            if pos_str.trim().eq("C") {
                read_position = Position::Catcher;
            } else if pos_str.trim().eq("1B") {
                read_position = Position::Firstbase;
            } else if pos_str.trim().eq("2B") {
                read_position = Position::Secondbase;
            } else if pos_str.trim().eq("SS") {
                read_position = Position::Shortstop;
            } else if pos_str.trim().eq("3B") {
                read_position = Position::Thirdbase;
            } else if pos_str.trim().eq("LF") {
                read_position = Position::Leftfield;
            } else if pos_str.trim().eq("CF") {
                read_position = Position::Centerfield;
            } else if pos_str.trim().eq("RF") {
                read_position = Position::Rightfield;
            } else if pos_str.trim().eq("Bench") {
                read_position = Position::Bench;
            }
        } else if statline[0].trim().eq("Handedness") {
            let hand_str = statline[1];
            if hand_str.trim().eq("R") {
                read_handedness = Handedness::Right;
            } else if hand_str.trim().eq("L") {
                read_handedness = Handedness::Left;
            } else if hand_str.trim().eq("S") {
                read_handedness = Handedness::Switch;
            }
        } else if statline[0].trim().eq("Batter Target") {
            let bt_result = statline[1].trim().parse();
            match bt_result {
                Ok(bt) => read_batter_target = bt,
                Err(_err) => println!("{}", "Failed to convert batter target number.".red().bold()),
            }
        } else if statline[0].trim().eq("On Base Target") {
            let obt_result = statline[1].trim().parse();
            match obt_result {
                Ok(obt) => read_on_base_target = obt,
                Err(_) => println!(
                    "{}",
                    "Failed to convert on base target number.".red().bold()
                ),
            }
        } else if statline[0].trim().eq("Pitch Die") {
            let pd_result = statline[1].trim().parse();
            match pd_result {
                Ok(pd) => read_pitch_die = pd,
                Err(_) => println!(
                    "{}",
                    "Failed to convert pitch die: leave out the 'd'."
                        .red()
                        .bold()
                ),
            }
        } else if statline[0].trim().eq("Traits") {
            let trait_string: Vec<&str> = statline[1].split(",").collect();
            for i in 0..trait_string.len() {
                if trait_string[i].trim().eq("P+") {
                    read_traits.push(Traits::PowerHitter);
                } else if trait_string[i].trim().eq("P++") {
                    read_traits.push(Traits::ElitePowerHitter);
                } else if trait_string[i].trim().eq("C+") {
                    read_traits.push(Traits::ContactHitter);
                } else if trait_string[i].trim().eq("S+") {
                    read_traits.push(Traits::SpeedyRunner);
                } else if trait_string[i].trim().eq("D+") {
                    read_traits.push(Traits::GreatDefender);
                } else if trait_string[i].trim().eq("T+") {
                    read_traits.push(Traits::ToughPlayer);
                } else if trait_string[i].trim().eq("P-") {
                    read_traits.push(Traits::WeakHitter);
                } else if trait_string[i].trim().eq("P--") {
                    read_traits.push(Traits::ExtraWeakHitter);
                } else if trait_string[i].trim().eq("C-") {
                    read_traits.push(Traits::FreeSwinger);
                } else if trait_string[i].trim().eq("S-") {
                    read_traits.push(Traits::SlowRunner);
                } else if trait_string[i].trim().eq("D-") {
                    read_traits.push(Traits::PoorDefender);
                } else if trait_string[i].trim().eq("K+") {
                    read_traits.push(Traits::StrikeoutArtist);
                } else if trait_string[i].trim().eq("GB+") {
                    read_traits.push(Traits::GroundballMachine);
                } else if trait_string[i].trim().eq("CN+") {
                    read_traits.push(Traits::ControlPitcher);
                } else if trait_string[i].trim().eq("ST+") {
                    read_traits.push(Traits::GreatStamina);
                } else if trait_string[i].trim().eq("CN-") {
                    read_traits.push(Traits::Wild);
                }
            }
        } else if statline[0].trim().eq("Injury Location") {
            let inj_loc_str: Vec<&str> = statline[1].split(",").collect();
            for i in 0..inj_loc_str.len() {
                if inj_loc_str[i].trim().eq("Head") {
                    read_injury_location.push(InjuryLocation::Head);
                } else if inj_loc_str[i].trim().eq("Shoulder") {
                    read_injury_location.push(InjuryLocation::Shoulder);
                } else if inj_loc_str[i].trim().eq("Elbow") {
                    read_injury_location.push(InjuryLocation::Elbow);
                } else if inj_loc_str[i].trim().eq("Forearm") {
                    read_injury_location.push(InjuryLocation::Forearm);
                } else if inj_loc_str[i].trim().eq("Wrist") {
                    read_injury_location.push(InjuryLocation::Wrist);
                } else if inj_loc_str[i].trim().eq("Hand") {
                    read_injury_location.push(InjuryLocation::Hand);
                } else if inj_loc_str[i].trim().eq("Back") {
                    read_injury_location.push(InjuryLocation::Back);
                } else if inj_loc_str[i].trim().eq("Oblique") {
                    read_injury_location.push(InjuryLocation::Oblique);
                } else if inj_loc_str[i].trim().eq("Hip") {
                    read_injury_location.push(InjuryLocation::Hip);
                } else if inj_loc_str[i].trim().eq("Hamstring") {
                    read_injury_location.push(InjuryLocation::Hamstring)
                } else if inj_loc_str[i].trim().eq("Knee") {
                    read_injury_location.push(InjuryLocation::Knee);
                } else if inj_loc_str[i].trim().eq("Ankle") {
                    read_injury_location.push(InjuryLocation::Ankle);
                } else if inj_loc_str[i].trim().eq("Foot") {
                    read_injury_location.push(InjuryLocation::Foot);
                }
            }
        } else if statline[0].trim().eq("Injury Severity") {
            let inj_sev_str: Vec<&str> = statline[1].split(",").collect();
            for i in 0..inj_sev_str.len() {
                if inj_sev_str[i].trim().eq("Catastrophic") {
                    read_injury_severity.push(InjurySeverity::Catastrophic);
                } else if inj_sev_str[i].trim().eq("Major") {
                    read_injury_severity.push(InjurySeverity::Major);
                } else if inj_sev_str[i].trim().eq("Minor") {
                    read_injury_severity.push(InjurySeverity::Minor);
                } else if inj_sev_str[i].trim().eq("Superficial") {
                    read_injury_severity.push(InjurySeverity::Superficial);
                } else if inj_sev_str[i].trim().eq("Uninjured") {
                    read_injury_severity.push(InjurySeverity::Uninjured);
                }
            }
        }
    }

    let player_data = Player {
        first_name: read_first_name,
        last_name: read_last_name,
        nickname: read_nickname,
        position: read_position,
        handedness: read_handedness,
        batter_target: read_batter_target,
        on_base_target: read_on_base_target,
        pitch_die: read_pitch_die,
        traits: read_traits,
        injury_location: read_injury_location,
        injury_severity: read_injury_severity,
    };

    player_data
}

// writes a Player struct to a *.DBP file
// TODO - shouldn't this take a reference to a player struct?
pub fn write_player(data: &Player, filename: &str) -> Result<(), std::io::Error> {
    let mut file_text = String::new();
    file_text.push_str("First Name: ");
    file_text.push_str(&data.first_name);
    file_text.push_str("\nLast Name: ");
    file_text.push_str(&data.last_name);
    file_text.push_str("\nNickname: ");
    file_text.push_str(&data.nickname);
    file_text.push_str("\nPosition: ");
    match data.position {
        Position::None => file_text.push_str("None"),
        Position::Bench => file_text.push_str("Bench"),
        Position::Pitcher => file_text.push_str("P"),
        Position::Catcher => file_text.push_str("C"),
        Position::Firstbase => file_text.push_str("1B"),
        Position::Shortstop => file_text.push_str("SS"),
        Position::Thirdbase => file_text.push_str("3B"),
        Position::Leftfield => file_text.push_str("LF"),
        Position::Secondbase => file_text.push_str("2B"),
        Position::Rightfield => file_text.push_str("RF"),
        Position::Centerfield => file_text.push_str("CF"),
    }
    file_text.push_str("\nHandedness: ");
    match data.handedness {
        Handedness::None => file_text.push_str("None"),
        Handedness::Left => file_text.push_str("L"),
        Handedness::Right => file_text.push_str("R"),
        Handedness::Switch => file_text.push_str("S"),
    }
    file_text.push_str("\nBatter Target: ");
    file_text.push_str(&data.batter_target.to_string());
    file_text.push_str("\nOn Base Target: ");
    file_text.push_str(&data.on_base_target.to_string());
    file_text.push_str("\nPitch Die: ");
    file_text.push_str(&data.pitch_die.to_string());
    file_text.push_str("\nTraits:");
    for i in 0..data.traits.len() {
        match data.traits[i] {
            Traits::None => file_text.push_str(" None,"),
            Traits::Wild => file_text.push_str(" CN-,"),
            Traits::WeakHitter => file_text.push_str(" P-,"),
            Traits::SlowRunner => file_text.push_str(" S-,"),
            Traits::PowerHitter => file_text.push_str(" P+,"),
            Traits::ToughPlayer => file_text.push_str(" T+,"),
            Traits::FreeSwinger => file_text.push_str(" C-,"),
            Traits::SpeedyRunner => file_text.push_str(" S+,"),
            Traits::PoorDefender => file_text.push_str(" D-,"),
            Traits::GreatStamina => file_text.push_str(" ST+,"),
            Traits::ContactHitter => file_text.push_str(" C+,"),
            Traits::GreatDefender => file_text.push_str(" D+,"),
            Traits::ControlPitcher => file_text.push_str(" CN+,"),
            Traits::ExtraWeakHitter => file_text.push_str(" P--,"),
            Traits::StrikeoutArtist => file_text.push_str(" K+,"),
            Traits::ElitePowerHitter => file_text.push_str(" P++,"),
            Traits::GroundballMachine => file_text.push_str(" GB+,"),
        }
    }
    file_text.push_str("\nInjury Location:");
    for j in 0..data.injury_location.len() {
        match data.injury_location[j] {
            InjuryLocation::Knee => file_text.push_str(" Knee,"),
            InjuryLocation::Hip => file_text.push_str(" Hip,"),
            InjuryLocation::Head => file_text.push_str(" Head,"),
            InjuryLocation::Hand => file_text.push_str(" Hand,"),
            InjuryLocation::Back => file_text.push_str(" Back,"),
            InjuryLocation::Foot => file_text.push_str(" Foot,"),
            InjuryLocation::None => file_text.push_str(" None,"),
            InjuryLocation::Elbow => file_text.push_str(" Elbow,"),
            InjuryLocation::Wrist => file_text.push_str(" Wrist,"),
            InjuryLocation::Ankle => file_text.push_str(" Ankle,"),
            InjuryLocation::Forearm => file_text.push_str(" Forearm,"),
            InjuryLocation::Oblique => file_text.push_str(" Oblique,"),
            InjuryLocation::Shoulder => file_text.push_str(" Shoulder,"),
            InjuryLocation::Hamstring => file_text.push_str(" Hamstring,"),
        }
    }
    file_text.push_str("\nInjury Severity:");
    for k in 0..data.injury_severity.len() {
        match data.injury_severity[k] {
            InjurySeverity::Major => file_text.push_str(" Major,"),
            InjurySeverity::Minor => file_text.push_str(" Minor,"),
            InjurySeverity::Uninjured => file_text.push_str(" Uninjured,"),
            InjurySeverity::Superficial => file_text.push_str(" Superficial,"),
            InjurySeverity::Catastrophic => file_text.push_str(" Catastrophic,"),
        }
    }

    let write_result = fs::write(filename, &file_text);
    write_result
}

// reads in name CSVs and puts them into memory for reference during player generation function
pub fn load_names() -> (Vec<String>, Vec<String>) {
    let firstnames: Vec<String>;
    let lastnames: Vec<String>;
    if let Ok(contents) = fs::read_to_string("src/databases/firstname.csv") {
        // split file up by line - 1 name per line
        firstnames = contents.split('\n').map(String::from).collect();
    } else {
        firstnames = vec!["first".to_string()];
        println!("WARNING: Failed to read firstname.csv");
    }

    if let Ok(contents) = fs::read_to_string("src/databases/lastname.csv") {
        lastnames = contents.split('\n').map(String::from).collect();
    } else {
        lastnames = vec!["last".to_string()];
        println!("WARNING: Failed to read lastname.csv");
    }
    return (firstnames, lastnames);
}

// TODO: player generation should take into account target ERA - will fix later
// TODO: might add aging at a later date
// TODO: could add mechanic for farmhand/prospect/veteran/etc.

// generates a player name
pub fn generate_name(firstnames: &Vec<String>, lastnames: &Vec<String>) -> (String, String) {
    let len_first = firstnames.len();
    let len_last = lastnames.len();
    let roll_first = roll(len_first as i32);
    let roll_last = roll(len_last as i32);
    let first_name = firstnames[roll_first as usize].clone();
    let last_name = lastnames[roll_last as usize].clone();
    return (first_name, last_name);
}

// generates handedness
pub fn generate_handedness(player_type: &PlayerClass) -> Handedness {
    let hand: Handedness;
    match player_type {
        PlayerClass::StartingHitter => {
            let result = roll(10);
            if result <= 6 {
                hand = Handedness::Right;
            } else if result >= 7 && result <= 9 {
                hand = Handedness::Left;
            } else {
                hand = Handedness::Switch;
            }
        }
        PlayerClass::PinchHitter => {
            let result = roll(10);
            if result <= 6 {
                hand = Handedness::Right;
            } else if result >= 7 && result <= 9 {
                hand = Handedness::Left;
            } else {
                hand = Handedness::Switch;
            }
        }
        PlayerClass::Pitchers => {
            let result = roll(10);
            if result <= 6 {
                hand = Handedness::Right;
            } else {
                hand = Handedness::Left;
            }
        }
    }
    return hand;
}

// generate batter target and on base target
pub fn generate_batter_target(player_type: &PlayerClass) -> (i32, i32) {
    let bt: i32;
    match player_type {
        PlayerClass::StartingHitter => {
            bt = 15 + roll(10) + roll(10);
        }
        PlayerClass::PinchHitter => {
            bt = 15 + roll(10);
        }
        PlayerClass::Pitchers => {
            bt = 5 + roll(10);
        }
    }
    let ot = bt + roll(6);
    return (bt, ot);
}

// generate pitch die
pub fn generate_pitch_die(player_type: &PlayerClass) -> i32 {
    let pd: i32;
    match player_type {
        PlayerClass::Pitchers => {
            let result = roll(8);
            if result == 1 {
                pd = 12;
            } else if result == 2 || result == 3 {
                pd = 8;
            } else if result >= 4 && result <= 6 {
                pd = 4;
            } else {
                pd = -4;
            }
        }
        _ => {
            pd = -8;
        }
    }
    return pd;
}
// generate traits
pub fn generate_traits(player_type: &PlayerClass) -> Vec<Traits> {
    let mut traits: Vec<Traits> = vec![];
    // roll for chance of 2 traits
    let chance = roll(100);
    let num_traits: i32;
    if chance <= 2 {
        num_traits = 2;
    } else {
        num_traits = 1;
    }
    // TODO could clean this up so only a single none is written, but it doesn't matter that much
    // so I'm not going to worry about it right now
    for _i in 0..num_traits {
        let result = roll(10) + roll(10);
        match player_type {
            PlayerClass::Pitchers => {
                if result < 5 {
                    traits.push(Traits::None);
                } else if result == 5 {
                    traits.push(Traits::Wild);
                } else if result >= 6 && result <= 14 {
                    traits.push(Traits::None);
                } else if result == 15 {
                    traits.push(Traits::StrikeoutArtist);
                } else if result == 16 {
                    traits.push(Traits::GroundballMachine);
                } else if result == 17 {
                    traits.push(Traits::ControlPitcher);
                } else if result == 18 {
                    traits.push(Traits::GreatStamina);
                } else {
                    traits.push(Traits::None);
                }
            }
            _ => {
                if result == 2 {
                    traits.push(Traits::ExtraWeakHitter);
                } else if result == 3 {
                    traits.push(Traits::WeakHitter);
                } else if result == 4 {
                    traits.push(Traits::SlowRunner);
                } else if result == 5 {
                    traits.push(Traits::FreeSwinger);
                } else if result == 6 {
                    traits.push(Traits::PoorDefender);
                } else if result >= 7 && result <= 14 {
                    traits.push(Traits::None);
                } else if result == 15 {
                    traits.push(Traits::GreatDefender);
                } else if result == 16 {
                    traits.push(Traits::PowerHitter);
                } else if result == 17 {
                    traits.push(Traits::ContactHitter);
                } else if result == 18 {
                    traits.push(Traits::SpeedyRunner);
                    traits.push(Traits::ToughPlayer);
                } else if result == 20 {
                    traits.push(Traits::ElitePowerHitter);
                } else {
                    traits.push(Traits::None);
                }
            }
        }
    }
    return traits;
}

// generates a new player in struct format
pub fn generate_player(
    player_type: PlayerClass,
    //era: &Era,
    position: Position,
    firstnames: &Vec<String>,
    lastnames: &Vec<String>,
) -> Player {
    let (first_name, last_name) = generate_name(firstnames, lastnames);
    let (bt, ot) = generate_batter_target(&player_type);

    let new_player = Player {
        first_name: first_name,
        last_name: last_name,
        nickname: "".to_string(),
        position: position,
        handedness: generate_handedness(&player_type),
        batter_target: bt,
        on_base_target: ot,
        pitch_die: generate_pitch_die(&player_type),
        traits: generate_traits(&player_type),
        injury_location: vec![InjuryLocation::None],
        injury_severity: vec![InjurySeverity::Uninjured],
    };

    return new_player;
}
