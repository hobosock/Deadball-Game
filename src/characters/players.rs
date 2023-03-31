/*========================================================
MODULE INCLUSIONS
========================================================*/
use std::fs; // needed to read in files

use text_colorizer::*;

/*========================================================
ENUM DEFINITIONS
========================================================*/
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Handedness {
    Right,
    Left,
    Switch,
    None,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum InjurySeverity {
    Catastrophic,
    Major,
    Minor,
    Superficial,
    Uninjured,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
#[derive(Debug)]
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
