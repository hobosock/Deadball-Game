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

/*========================================================
STRUCT DEFINITIONS
========================================================*/
pub struct GameModern {
    pub home: Team,
    pub away: Team,
    pub ballpark: BallparkModern,
}
pub struct GameState {
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

pub fn create_modern_game (home: Team, away: Team, ballpark: BallparkModern) -> Result<GameModern, E> {
    // check teams and park for complete information
    if home.roster.len() < 9 {
        println!("{}", "Home team does not have a complete roster".red().bold());
    }
    if away.roster.len() < 9 {
        println!("{}", "Away team does not have a complete roster".red().bold());
    }
    // need to check eras
}

pub fn modern_game_flow(game: GameModern, state: GameState) {
    // check inning
    // check score
}

