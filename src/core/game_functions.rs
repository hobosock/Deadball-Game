/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
//#[allow(non_camel_case_types)]
//#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/
use text_colorizer::*;

use crate::characters::teams::*;

/*========================================================
ENUM DEFINITIONS
========================================================*/
pub enum AtBatResults {
    Oddity,
    CriticalHit,
    Hit,
    Walk,
    PossibleError,
    ProductiveOut1,
    ProductiveOut2,
    Out,
    MegaOut,
}

pub enum InningTB {
    Top,
    Bottom,
}

pub enum Outs {
    One,
    Two,
    Three,
    None,
}

// each number is base binary (1 is runner on, 0 is no runner)
pub enum RunnersOn {
    Runner000,
    Runner100,
    Runner010,
    Runner001,
    Runner110,
    Runner101,
    Runner011,
    Runner111,
}

pub enum GameStatus {
    NotStarted,
    Ongoing,
    Over,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
pub struct GameModern<'a> {
    pub home: &'a Team,
    pub away: &'a Team,
    pub ballpark: &'a BallparkModern,
}
pub struct GameState {
    pub status: GameStatus,
    pub inning: u32,
    pub inning_half: InningTB,
    pub outs: Outs,
    pub runners: RunnersOn,
    pub batting_team1: u32,
    pub batting_team2: u32,
    pub pitched_team1: u32,
    pub pitched_team2: u32,
    pub runs_team1: u32,
    pub runs_team2: u32,
    pub hits_team1: u32,
    pub hits_team2: u32,
    pub errors_team1: u32,
    pub errors_team2: u32,
}

//======== CUSTOM ERRORS =================================
#[derive(Debug, Clone)]
pub struct TeamError {
    pub message: String,
    pub team: String,
}

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
pub fn at_bat(bat_target: i32, on_base_target: i32, pitch_result: i32) -> AtBatResults {
    let mut at_bat_result = AtBatResults::MegaOut;

    if pitch_result == 1 {
        at_bat_result = AtBatResults::Oddity;
    } else if pitch_result >= 2 && pitch_result <= 5 {
        at_bat_result = AtBatResults::CriticalHit;
    } else if pitch_result >= 6 && pitch_result <= bat_target {
        at_bat_result = AtBatResults::Hit;
    } else if pitch_result > bat_target && pitch_result <= on_base_target {
        at_bat_result = AtBatResults::Walk;
    } else if pitch_result > on_base_target && pitch_result <= on_base_target + 5 {
        at_bat_result = AtBatResults::PossibleError;
    } else if pitch_result >= on_base_target + 6 && pitch_result <= 49 {
        at_bat_result = AtBatResults::ProductiveOut1;
    } else if pitch_result >= 50 && pitch_result <= 69 {
        at_bat_result = AtBatResults::ProductiveOut2;
    } else if pitch_result >= 70 && pitch_result <= 98 {
        at_bat_result = AtBatResults::Out;
    } else if pitch_result == 99 {
        at_bat_result = AtBatResults::Oddity;
    } else if pitch_result >= 100 {
        at_bat_result = AtBatResults::MegaOut;
    }

    at_bat_result
}

pub fn create_modern_game<'a>(
    home: &'a Team,
    away: &'a Team,
    ballpark: &'a BallparkModern,
) -> Result<GameModern<'a>, TeamError> {
    // check teams and park for complete information
    if home.roster.len() < 9 {
        println!(
            "{}",
            "Home team does not have a complete roster".red().bold()
        );
        return Err(TeamError {
            message: "Home team does not have a complete roster".to_string(),
            team: home.name.clone(),
        });
    }
    if away.roster.len() < 9 {
        println!(
            "{}",
            "Away team does not have a complete roster".red().bold()
        );
        return Err(TeamError {
            message: "Away team does not have a complete roster".to_string(),
            team: away.name.clone(),
        });
    }
    match home.era {
        Era::Modern => (),
        _ => {
            return Err(TeamError {
                message: "Home team is not for the modern era".to_string(),
                team: home.name.clone(),
            })
        }
    }
    match away.era {
        Era::Modern => (),
        _ => {
            return Err(TeamError {
                message: "Away team is not for the modern era".to_string(),
                team: away.name.clone(),
            })
        }
    }

    let game = GameModern {
        home: home,
        away: away,
        ballpark: ballpark,
    };
    return Ok(game);
}

pub fn modern_game_flow(game: &GameModern, state: &mut GameState) {
    // ONCE PER GAME
    // ONCE PER INNING HALF
    // check inning
    // check score
    // check number of innings pitched
    // ONCE PER AT BAT
    // check number of outs
    // user input???
    // check at bat position
    // simulate at bat
    // update game state

    loop {
        // check top of the 9th at a different place
        if state.inning > 9 {
            // check score
            if state.runs_team1 != state.runs_team2 {
                state.status = GameStatus::Over;
            }
        }
        match state.status {
            GameStatus::NotStarted => {
                // maybe time for the player to make roster adjustments?
                state.status = GameStatus::Ongoing;
            }
            GameStatus::Ongoing => match state.inning_half {
                InningTB::Top => {}
                InningTB::Bottom => {}
            },
            GameStatus::Over => {}
        }
    }
}
