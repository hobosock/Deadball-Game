/*========================================================
CONFIGURE RUSTC WARNINGS
========================================================*/
//#[allow(non_camel_case_types)]
//#[allow(non_snake_case)]

/*========================================================
MODULE INCLUSIONS
========================================================*/
use std::fs;
use text_colorizer::*;

use crate::characters::{players::*, teams::*};

use super::roll;

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

// 2d10
pub enum Oddity {
    FanInterference,
    AnimalOnField,
    RainDelay,
    FielderAppearsInjured,
    PitcherAppearsInjured,
    Tootblan,
    PickOff,
    CallBlownAtFirst,
    CallBlownAtHomePlate,
    HitByPitch,
    WildPitch,
    PitcherDistracted,
    DroppedThirdStrike,
    PassedBall,
    CurrentBatterAppearsInjured,
    PreviousBatterAppearsInjured,
    PitcherError,
    Balk,
    CatcherInterference,
}

// d20
pub enum HitTable {
    Single,
    SingleDef1B,
    SingleDef2B,
    SingleDef3B,
    SingleDefSS,
    SingleRunnersAdv,
    DoubleDefLF,
    DoubleDefCF,
    DoubleDefRF,
    DoubleRunnerAdv,
    HomeRun,
}

// last digit of swing result
pub enum OutType {
    K,
    G3,
    G4,
    G5,
    G6,
    F7,
    F8,
    F9,
}

// Bunting (d6), needs to be handled with base situation

// d8 to steal second, d8-1 to steal third
pub enum Stealing {
    RunnerOut,
    RunnerSafe,
}

// d12
pub enum Defense {
    Error,
    NoChange,
    DoubleToSingle,
    HitToOut,
}

pub enum Animal {
    Bird,
    Rodent,
    Cat,
    Streaker,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
pub struct GameModern<'a> {
    pub home: &'a Team,
    pub away: &'a Team,
    pub home_active: ActiveTeam,
    pub away_active: ActiveTeam,
    pub ballpark: &'a BallparkModern,
}
pub struct GameState<'a> {
    pub status: GameStatus,
    pub inning: u32,
    pub inning_half: InningTB,
    pub outs: Outs,
    pub runners: RunnersOn,
    pub batting_team1: u32,
    pub batting_team2: u32,
    pub current_pitcher_team1: &'a Player,
    pub current_pitcher_team2: &'a Player,
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
    // initialize structs and then push
    let mut home_active = ActiveTeam {
        roster: vec![],
        bench: vec![],
        pitching: vec![],
        bullpen: vec![],
    };
    // try to load all the players, return error if it fails
    for i in 0..home.roster.len() {
        let read_results = fs::read_to_string(&home.roster[i]);
        match read_results {
            Ok(content) => home_active.roster.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &home.roster[i]
            ),
        }
    }
    for i in 0..home.bench.len() {
        let read_results = fs::read_to_string(&home.bench[i]);
        match read_results {
            Ok(content) => home_active.bench.push(load_player(content)),
            Err(_err) => println!("{}: {}", "failed to load file".red().bold(), &home.bench[i]),
        }
    }
    for i in 0..home.pitcher.len() {
        let read_results = fs::read_to_string(&home.pitcher[i]);
        match read_results {
            Ok(content) => home_active.pitching.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &home.pitcher[i]
            ),
        }
    }
    for i in 0..home.bullpen.len() {
        let read_results = fs::read_to_string(&home.bullpen[i]);
        match read_results {
            Ok(content) => home_active.bullpen.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &home.bullpen[i]
            ),
        }
    }

    let mut away_active = ActiveTeam {
        roster: vec![],
        bench: vec![],
        pitching: vec![],
        bullpen: vec![],
    };
    for i in 0..away.roster.len() {
        let read_results = fs::read_to_string(&away.roster[i]);
        match read_results {
            Ok(content) => away_active.roster.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &away.roster[i]
            ),
        }
    }
    for i in 0..away.bench.len() {
        let read_results = fs::read_to_string(&away.bench[i]);
        match read_results {
            Ok(content) => away_active.bench.push(load_player(content)),
            Err(_err) => println!("{}: {}", "failed to load file".red().bold(), &away.bench[i]),
        }
    }
    for i in 0..away.pitcher.len() {
        let read_results = fs::read_to_string(&away.pitcher[i]);
        match read_results {
            Ok(content) => away_active.pitching.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &away.pitcher[i]
            ),
        }
    }
    for i in 0..away.bullpen.len() {
        let read_results = fs::read_to_string(&away.bullpen[i]);
        match read_results {
            Ok(content) => away_active.bullpen.push(load_player(content)),
            Err(_err) => println!(
                "{}: {}",
                "failed to load file".red().bold(),
                &away.bullpen[i]
            ),
        }
    }

    let game = GameModern {
        home: home,
        away: away,
        ballpark: ballpark,
        home_active: home_active,
        away_active: away_active,
    };
    return Ok(game);
}

pub fn modern_game_flow<'a>(game: &'a GameModern, mut state: GameState<'a>) {
    let home_team_info = game.home; // home = team 1
    let away_team_info = game.away; // away = team 2
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
    let home_team = &game.home_active;
    let away_team = &game.away_active;

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
                // just set first pitcher as active pitcher for now
                state.current_pitcher_team1 = &home_team.pitching[0];
                state.current_pitcher_team2 = &away_team.pitching[0];
                state.status = GameStatus::Ongoing;
            }
            GameStatus::Ongoing => match state.inning_half {
                InningTB::Top => {
                    match state.outs {
                        Outs::Three => {
                            state.inning_half = InningTB::Bottom;
                            state.outs = Outs::None;
                        }
                        _ => {
                            // inning function
                        }
                    }
                }
                InningTB::Bottom => {}
            },
            GameStatus::Over => {}
        }
    }
}

pub fn modern_inning_flow<'a>(game: &'a GameModern, mut state: GameState<'a>) {
    loop {
        match state.inning_half {
            InningTB::Top => {}
            InningTB::Bottom => {
                match state.outs {
                    Outs::Three => return,
                    _ => {
                        // get active batter
                        // get at bat Result
                        // update score/runners/Outs
                        let pd = state.current_pitcher_team2.pitch_die;
                        let mut pitch_result: i32;
                        if pd > 0 {
                            pitch_result = roll(pd);
                        } else {
                            pitch_result = -1 * roll(pd.abs());
                        }
                        pitch_result += roll(100);
                        let swing_result = at_bat(
                            game.home_active.roster[state.batting_team1 as usize].batter_target,
                            game.home_active.roster[state.batting_team1 as usize].on_base_target,
                            pitch_result,
                        );
                        if state.batting_team1 == 9 {
                            state.batting_team1 = 1;
                        } else {
                            state.batting_team1 += 1;
                        }

                        match swing_result {
                            AtBatResults::Oddity => {
                                let oddity_result = roll(10) + roll(10);
                                state = oddity(&oddity_result, &pitch_result, game, state);
                            }
                            AtBatResults::CriticalHit => {
                                // make hit roll, bump up a level
                                let mut hit_result = roll(20);
                                hit_result = crit_hit(&hit_result);
                            }
                            AtBatResults::Hit => {}
                            AtBatResults::Walk => {}
                            AtBatResults::PossibleError => {}
                            AtBatResults::ProductiveOut1 => {}
                            AtBatResults::ProductiveOut2 => {}
                            AtBatResults::Out => {}
                            AtBatResults::MegaOut => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn oddity<'b>(
    oddity_result: &i32,
    pitch_result: &i32,
    game: &'b GameModern,
    mut state: GameState<'b>,
) -> GameState<'b> {
    match state.inning_half {
        InningTB::Top => return state,
        InningTB::Bottom => {
            if *oddity_result == 2 {
                if pitch_result % 2 == 1 {
                    // fan catches sure out, at bat continues
                    state.batting_team1 -= 1;
                } else {
                    // home run overturned, batter out
                    match state.outs {
                        Outs::None => state.outs = Outs::One,
                        Outs::One => state.outs = Outs::Two,
                        Outs::Two => state.outs = Outs::Three,
                        Outs::Three => state.outs = Outs::Three,
                    }
                }
                return state;
            } else if *oddity_result == 3 {
                // animal on the field
                // animal function here
                println!("{}", "Animal on the field!".bold().yellow());
                return state;
            } else if *oddity_result == 4 {
                // rain delay
                println!("{}", "Rain delay.".bold().cyan());
                // rain delay function
                return state;
            } else if *oddity_result == 5 {
                // player injured
                // player injured function
                return state;
            } else if *oddity_result == 6 {
                // pitcher appears injured
                // player injured function
                return state;
            } else if *oddity_result == 7 {
                // TOOTBLAN
                return state;
            } else if *oddity_result == 8 {
                // pick off
                return state;
            } else if *oddity_result == 9 {
                // call blown at first
                return state;
            } else if *oddity_result == 10 {
                // call blown at home
                return state;
            } else if *oddity_result == 11 {
                // hit by pitch
                return state;
            } else if *oddity_result == 12 {
                // wild pitch
                return state;
            } else if *oddity_result == 13 {
                // pitcher distracted
                return state;
            } else if *oddity_result == 14 {
                // dropped third strike
                return state;
            } else if *oddity_result == 15 {
                // passed ball
                return state;
            } else if *oddity_result == 16 {
                // current batter appears injured
                return state;
            } else if *oddity_result == 17 {
                // previous batter appears injured
                return state;
            } else if *oddity_result == 18 {
                // pitcher error
                return state;
            } else if *oddity_result == 19 {
                // balk
                return state;
            } else if *oddity_result == 20 {
                // catcher interference
                return state;
            } else {
                return state;
            }
        }
    }
}

pub fn crit_hit<'a>(hit_result: &i32) -> i32 {
    // based on 2E Deadball quick reference hit table
    let mut crit_result: i32 = *hit_result;
    if *hit_result >= 1 && *hit_result <= 2 {
        crit_result = 18;
    } else if *hit_result >= 7 && *hit_result <= 9 {
        crit_result = 18;
    } else if *hit_result >= 5 && *hit_result <= 6 {
        crit_result = 15;
    } else if *hit_result == 3 {
        crit_result = 17;
    } else if *hit_result == 4 {
        crit_result = 16;
    } else if *hit_result >= 15 && *hit_result <= 18 {
        crit_result = 19;
    }

    return crit_result;
}
