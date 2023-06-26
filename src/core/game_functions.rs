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

#[derive(Debug)]
pub enum InningTB {
    Top,
    Bottom,
}

#[derive(Debug)]
pub enum Outs {
    One,
    Two,
    Three,
    None,
}

// each number is base binary (1 is runner on, 0 is no runner)
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    if home.roster.len() < 8 {
        println!(
            "{}",
            "Home team does not have a complete roster".red().bold()
        );
        return Err(TeamError {
            message: "Home team does not have a complete roster".to_string(),
            team: home.name.clone(),
        });
    }
    if away.roster.len() < 8 {
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
        batting_order: vec![],
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
    // for now, make batting order roster + pitcher
    home_active.batting_order = home_active.roster.clone();
    home_active
        .batting_order
        .push(home_active.pitching[0].clone());

    let mut away_active = ActiveTeam {
        roster: vec![],
        bench: vec![],
        pitching: vec![],
        bullpen: vec![],
        batting_order: vec![],
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
    // for now, make batting order roster + pitcher
    away_active.batting_order = away_active.roster.clone();
    away_active
        .batting_order
        .push(away_active.pitching[0].clone());

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
        // TODO delete these debug print statements once it is fixed
        println!("{:?}", state);
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
                //state.current_pitcher_team1 = &home_team.pitching[0];
                //state.current_pitcher_team2 = &away_team.pitching[0];
                state.status = GameStatus::Ongoing;
                // TODO delete this
                println!("Play ball!");
            }
            GameStatus::Ongoing => match state.inning_half {
                InningTB::Top => match state.outs {
                    Outs::Three => {
                        state.inning_half = InningTB::Bottom;
                        state.outs = Outs::None;
                    }
                    _ => {
                        state = modern_inning_flow(game, state);
                    }
                },
                InningTB::Bottom => {
                    match state.outs {
                        Outs::Three => {
                            state.inning_half = InningTB::Top;
                            state.outs = Outs::None; // reset outs
                        }
                        _ => {
                            state = modern_inning_flow(game, state);
                        }
                    }
                }
            },
            GameStatus::Over => {
                // temporary printing of results
                println!("FINAL SCORE");
                println!("HOME: {} - AWAY: {}", state.runs_team1, state.runs_team2);
            }
        }
    }
}

pub fn modern_inning_flow<'a>(game: &'a GameModern, mut state: GameState<'a>) -> GameState<'a> {
    loop {
        match state.inning_half {
            InningTB::Top => {
                // should match Bottom arm, just flip the teams - probably a better way to do this
                // than copy paste
                match state.outs {
                    Outs::Three => return state,
                    _ => {
                        // get active batter
                        // get at bat Result
                        // update score/runners/Outs
                        let pd = state.current_pitcher_team1.pitch_die;
                        let mut pitch_result: i32;
                        if pd > 0 {
                            pitch_result = roll(pd);
                        } else {
                            pitch_result = -1 * roll(pd.abs());
                        }
                        pitch_result += roll(100);
                        let swing_result = at_bat(
                            game.home_active.batting_order[state.batting_team2 as usize]
                                .batter_target,
                            game.home_active.batting_order[state.batting_team2 as usize]
                                .on_base_target,
                            pitch_result,
                        );
                        if state.batting_team2 == 8 {
                            state.batting_team2 = 0;
                        } else {
                            state.batting_team2 += 1;
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
                                state = hit_table(&hit_result, state);
                                // TODO no DEF roll on crit_hit
                            }
                            AtBatResults::Hit => {
                                // hit roll
                                let hit_result = roll(20);
                                state = hit_table(&hit_result, state);
                            }
                            AtBatResults::Walk => {
                                // basically like a single, just don't update the hit values
                                state = runners_advance(state, &1);
                                state = add_runner(state, &1);
                            }
                            AtBatResults::PossibleError => {
                                // TODO Not sure I am implementing this correctly, see page 29
                                // get position
                                // TODO get player traits
                                let def_roll = roll(12);
                                if def_roll <= 2 {
                                    // fielder makes an error
                                    // TODO these kind of match statements are redundant, clean it up
                                    match state.inning_half {
                                        InningTB::Top => {
                                            state.errors_team1 += 1;
                                        }
                                        InningTB::Bottom => {
                                            state.errors_team2 += 1;
                                        }
                                    }
                                    state = runners_advance(state, &1);
                                    state = add_runner(state, &1);
                                } else {
                                    // fielder makes the out like normal
                                    match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    }
                                }
                            }
                            AtBatResults::ProductiveOut1 => {
                                // TODO only proceed if less than two outs
                                // if first or outfield, runners on 2nd and 3rd advance
                                // if 2B/SS/3B, runner at first advances and batter is out
                                match state.outs {
                                    Outs::Three => {}
                                    Outs::Two => {
                                        state.outs = Outs::Three;
                                    }
                                    _ => {
                                        let fielder = get_swing_position(&pitch_result);
                                        if fielder == 3 || fielder >= 7 {
                                            // check for runners on second and third
                                            // advance if they exist
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner001 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner011 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner110 => {
                                                    // can't use normal runners advance function because
                                                    // runner at first doesn't move
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner100;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                                RunnersOn::Runner111 => {
                                                    state.runners = RunnersOn::Runner101;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            // check for runner on first
                                            match state.runners {
                                                RunnersOn::Runner100 => {
                                                    state.runners = RunnersOn::Runner010;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner011;
                                                }
                                                _ => {}
                                            }
                                        }
                                        // update out
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::One;
                                            }
                                            Outs::One => {
                                                state.outs = Outs::Two;
                                            }
                                            Outs::Two => {
                                                state.outs = Outs::Three;
                                            }
                                            Outs::Three => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                }
                            }
                            AtBatResults::ProductiveOut2 => {
                                // if first or outfield, runners on 2nd and 3rd advance
                                // if 2B/SS/3B, runner is out and batter makes it to first
                                // the first line is the same as ProductiveOut1
                                match state.outs {
                                    Outs::Three => {}
                                    Outs::Two => {
                                        state.outs = Outs::Three;
                                    }
                                    _ => {
                                        let fielder = get_swing_position(&pitch_result);
                                        if fielder == 3 || fielder >= 7 {
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner001 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner011 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner110 => {
                                                    // can't use normal runners advance function because
                                                    // runner at first doesn't move
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner100;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                                RunnersOn::Runner111 => {
                                                    state.runners = RunnersOn::Runner101;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                            }
                                            //
                                        } else {
                                            // advance batter to first and lead runner is out
                                            // TODO should this be done for force outs only?
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state.runners = RunnersOn::Runner100;
                                                }
                                                RunnersOn::Runner001 => {
                                                    state.runners = RunnersOn::Runner100;
                                                }
                                                RunnersOn::Runner110 => {}
                                                RunnersOn::Runner011 => {
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner110;
                                                }
                                                RunnersOn::Runner111 => {}
                                            }
                                        }
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::One;
                                            }
                                            Outs::One => {
                                                state.outs = Outs::Two;
                                            }
                                            Outs::Two => {
                                                state.outs = Outs::Three;
                                            }
                                            Outs::Three => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                }
                            }
                            AtBatResults::Out => {
                                // no runners advance
                                // anywhere in the infield, runner at first and batter are out
                                let fielder = get_swing_position(&pitch_result);
                                if fielder >= 3 && fielder <= 6 {
                                    match state.outs {
                                        Outs::Three => {}
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        _ => match state.runners {
                                            RunnersOn::Runner100 => {
                                                state.runners = RunnersOn::Runner000;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner110 => {
                                                state.runners = RunnersOn::Runner010;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner101 => {
                                                state.runners = RunnersOn::Runner001;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner111 => {
                                                state.runners = RunnersOn::Runner011;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            _ => match state.outs {
                                                Outs::None => {
                                                    state.outs = Outs::One;
                                                }
                                                Outs::One => {
                                                    state.outs = Outs::Two;
                                                }
                                                Outs::Two => {
                                                    state.outs = Outs::Three;
                                                }
                                                Outs::Three => {
                                                    state.outs = Outs::Three;
                                                }
                                            },
                                        },
                                    }
                                } else {
                                    match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    }
                                }
                            }
                            AtBatResults::MegaOut => {
                                // triple play if no outs and runners on first and second
                                // check for triple play, otherwise same as previous branch
                                match state.runners {
                                    RunnersOn::Runner110 => {
                                        state.outs = Outs::Three;
                                        // TODO only say it's a triple play if no outs
                                    }
                                    RunnersOn::Runner111 => {
                                        state.outs = Outs::Three;
                                    }
                                    RunnersOn::Runner100 => {
                                        state.runners = RunnersOn::Runner000;
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::Two;
                                            }
                                            _ => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                    RunnersOn::Runner101 => {
                                        state.runners = RunnersOn::Runner001;
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::Two;
                                            }
                                            _ => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                    _ => match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    },
                                }
                            }
                        }
                        return state;
                    }
                }
            }
            InningTB::Bottom => {
                match state.outs {
                    Outs::Three => return state,
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
                            game.home_active.batting_order[state.batting_team1 as usize]
                                .batter_target,
                            game.home_active.batting_order[state.batting_team1 as usize]
                                .on_base_target,
                            pitch_result,
                        );
                        if state.batting_team1 == 8 {
                            state.batting_team1 = 0;
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
                                state = hit_table(&hit_result, state);
                                // TODO no DEF roll on crit_hit
                            }
                            AtBatResults::Hit => {
                                // hit roll
                                let hit_result = roll(20);
                                state = hit_table(&hit_result, state);
                            }
                            AtBatResults::Walk => {
                                // basically like a single, just don't update the hit values
                                state = runners_advance(state, &1);
                                state = add_runner(state, &1);
                            }
                            AtBatResults::PossibleError => {
                                // TODO Not sure I am implementing this correctly, see page 29
                                // get position
                                // TODO get player traits
                                let def_roll = roll(12);
                                if def_roll <= 2 {
                                    // fielder makes an error
                                    match state.inning_half {
                                        InningTB::Top => {
                                            state.errors_team1 += 1;
                                        }
                                        InningTB::Bottom => {
                                            state.errors_team2 += 1;
                                        }
                                    }
                                    state = runners_advance(state, &1);
                                    state = add_runner(state, &1);
                                } else {
                                    // fielder makes the out like normal
                                    match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    }
                                }
                            }
                            AtBatResults::ProductiveOut1 => {
                                // TODO only proceed if less than two outs
                                // if first or outfield, runners on 2nd and 3rd advance
                                // if 2B/SS/3B, runner at first advances and batter is out
                                match state.outs {
                                    Outs::Three => {}
                                    Outs::Two => {
                                        state.outs = Outs::Three;
                                    }
                                    _ => {
                                        let fielder = get_swing_position(&pitch_result);
                                        if fielder == 3 || fielder >= 7 {
                                            // check for runners on second and third
                                            // advance if they exist
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner001 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner011 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner110 => {
                                                    // can't use normal runners advance function because
                                                    // runner at first doesn't move
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner100;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                                RunnersOn::Runner111 => {
                                                    state.runners = RunnersOn::Runner101;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            // check for runner on first
                                            match state.runners {
                                                RunnersOn::Runner100 => {
                                                    state.runners = RunnersOn::Runner010;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner011;
                                                }
                                                _ => {}
                                            }
                                        }
                                        // update out
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::One;
                                            }
                                            Outs::One => {
                                                state.outs = Outs::Two;
                                            }
                                            Outs::Two => {
                                                state.outs = Outs::Three;
                                            }
                                            Outs::Three => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                }
                            }
                            AtBatResults::ProductiveOut2 => {
                                // if first or outfield, runners on 2nd and 3rd advance
                                // if 2B/SS/3B, runner is out and batter makes it to first
                                // the first line is the same as ProductiveOut1
                                match state.outs {
                                    Outs::Three => {}
                                    Outs::Two => {
                                        state.outs = Outs::Three;
                                    }
                                    _ => {
                                        let fielder = get_swing_position(&pitch_result);
                                        if fielder == 3 || fielder >= 7 {
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner001 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner011 => {
                                                    state = runners_advance(state, &1);
                                                }
                                                RunnersOn::Runner110 => {
                                                    // can't use normal runners advance function because
                                                    // runner at first doesn't move
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner100;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                                RunnersOn::Runner111 => {
                                                    state.runners = RunnersOn::Runner101;
                                                    match state.inning_half {
                                                        InningTB::Top => {
                                                            state.runs_team2 += 1;
                                                        }
                                                        InningTB::Bottom => {
                                                            state.runs_team1 += 1;
                                                        }
                                                    }
                                                }
                                            }
                                            //
                                        } else {
                                            // advance batter to first and lead runner is out
                                            // TODO should this be done for force outs only?
                                            match state.runners {
                                                RunnersOn::Runner000 => {}
                                                RunnersOn::Runner100 => {}
                                                RunnersOn::Runner010 => {
                                                    state.runners = RunnersOn::Runner100;
                                                }
                                                RunnersOn::Runner001 => {
                                                    state.runners = RunnersOn::Runner100;
                                                }
                                                RunnersOn::Runner110 => {}
                                                RunnersOn::Runner011 => {
                                                    state.runners = RunnersOn::Runner101;
                                                }
                                                RunnersOn::Runner101 => {
                                                    state.runners = RunnersOn::Runner110;
                                                }
                                                RunnersOn::Runner111 => {}
                                            }
                                        }
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::One;
                                            }
                                            Outs::One => {
                                                state.outs = Outs::Two;
                                            }
                                            Outs::Two => {
                                                state.outs = Outs::Three;
                                            }
                                            Outs::Three => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                }
                            }
                            AtBatResults::Out => {
                                // no runners advance
                                // anywhere in the infield, runner at first and batter are out
                                let fielder = get_swing_position(&pitch_result);
                                if fielder >= 3 && fielder <= 6 {
                                    match state.outs {
                                        Outs::Three => {}
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        _ => match state.runners {
                                            RunnersOn::Runner100 => {
                                                state.runners = RunnersOn::Runner000;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner110 => {
                                                state.runners = RunnersOn::Runner010;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner101 => {
                                                state.runners = RunnersOn::Runner001;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            RunnersOn::Runner111 => {
                                                state.runners = RunnersOn::Runner011;
                                                match state.outs {
                                                    Outs::None => {
                                                        state.outs = Outs::Two;
                                                    }
                                                    _ => {
                                                        state.outs = Outs::Three;
                                                    }
                                                }
                                            }
                                            _ => match state.outs {
                                                Outs::None => {
                                                    state.outs = Outs::One;
                                                }
                                                Outs::One => {
                                                    state.outs = Outs::Two;
                                                }
                                                Outs::Two => {
                                                    state.outs = Outs::Three;
                                                }
                                                Outs::Three => {
                                                    state.outs = Outs::Three;
                                                }
                                            },
                                        },
                                    }
                                } else {
                                    match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    }
                                }
                            }
                            AtBatResults::MegaOut => {
                                // triple play if no outs and runners on first and second
                                // check for triple play, otherwise same as previous branch
                                match state.runners {
                                    RunnersOn::Runner110 => {
                                        state.outs = Outs::Three;
                                        // TODO only say it's a triple play if no outs
                                    }
                                    RunnersOn::Runner111 => {
                                        state.outs = Outs::Three;
                                    }
                                    RunnersOn::Runner100 => {
                                        state.runners = RunnersOn::Runner000;
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::Two;
                                            }
                                            _ => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                    RunnersOn::Runner101 => {
                                        state.runners = RunnersOn::Runner001;
                                        match state.outs {
                                            Outs::None => {
                                                state.outs = Outs::Two;
                                            }
                                            _ => {
                                                state.outs = Outs::Three;
                                            }
                                        }
                                    }
                                    _ => match state.outs {
                                        Outs::None => {
                                            state.outs = Outs::One;
                                        }
                                        Outs::One => {
                                            state.outs = Outs::Two;
                                        }
                                        Outs::Two => {
                                            state.outs = Outs::Three;
                                        }
                                        Outs::Three => {
                                            state.outs = Outs::Three;
                                        }
                                    },
                                }
                            }
                        }
                        return state;
                    }
                }
            }
        }
    }
}

// rolls on the oddity table and updates game state
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
                    //state.batting_team1 -= 1;
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

// bumps hit roll up a level on the hit table
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

// rolls on the hit table and updates game state accordingly
pub fn hit_table<'b>(hit_result: &i32, mut state: GameState<'b>) -> GameState<'b> {
    // 1. defense roll (if needed)
    // 2. advance runners
    // 3 move hitter to runner
    // 4. update hit values in game state
    if *hit_result <= 2 {
        // single
        state = runners_advance(state, &1);
        state = add_runner(state, &1);
        // simple hit increment when no defense roll involved
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        return state;
    } else if *hit_result == 3 {
        // single DEF 1B
        let mut advance = 1;
        let mut base = 1;
        // when a defense roll is involved, add hit first and then you can subtract if there is an
        // out or an error
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 4 {
        // single DEF 2B
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 5 {
        // single DEF 3B
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 6 {
        // single DEF SS
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result >= 7 && *hit_result <= 9 {
        // single
        state = runners_advance(state, &1);
        state = add_runner(state, &1);
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        return state;
    } else if *hit_result >= 10 && *hit_result <= 14 {
        // single, runners advance 2
        state = runners_advance(state, &2);
        state = add_runner(state, &1);
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        return state;
    } else if *hit_result == 15 {
        // double DEF LF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 16 {
        // double DEF CF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 17 {
        // double DEF RF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        let def_roll = roll(12); // defense rolls are d12
                                 // TODO: eventually will put trait check here
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base);
        return state;
    } else if *hit_result == 18 {
        // double, runners advance 3
        state = runners_advance(state, &3);
        state = add_runner(state, &2);
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        return state;
    } else if *hit_result >= 19 {
        // home run
        let mut runs = runnerson(&state);
        runs += 1;
        state.runners = RunnersOn::Runner000;
        match state.inning_half {
            InningTB::Top => {
                state.runs_team2 += runs;
            }
            InningTB::Bottom => {
                state.runs_team1 += runs;
            }
        }
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 += 1;
            }
        }
        return state;
    } else {
        return state;
    }
}

// TODO find position player function - finds player info based on position and inning

// defense roll function - rolls on the defense table and updates game state
pub fn defense<'b>(
    mut state: GameState<'b>,
    def_result: &i32,
    mut advance: u32,
    mut base: u32,
) -> (GameState<'b>, u32, u32) {
    if *def_result <= 2 {
        // error, runners take an extra base
        // modify hit and error values
        // should be okay to subtract here since hit was added before passing into this function
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 -= 1;
                state.errors_team1 += 1;
            }
            InningTB::Bottom => {
                state.hits_team1 -= 1;
                state.errors_team2 += 1;
            }
        }
        return (state, advance + 1, base + 1);
    } else if *def_result >= 3 && *def_result <= 9 {
        // no change
        return (state, advance, base);
    } else if *def_result >= 10 && *def_result <= 11 {
        // double turns to single, runners advance 2, single turns to out, runners advance 1
        if base == 1 {
            match state.outs {
                Outs::None => {
                    state.outs = Outs::One;
                }
                Outs::One => {
                    state.outs = Outs::Two;
                }
                Outs::Two => {
                    state.outs = Outs::Three;
                }
                Outs::Three => {
                    state.outs = Outs::Three;
                }
            }
            base = 0;
            advance = 1;
        } else if base == 2 {
            base = 1;
            advance = 2;
        }
        return (state, advance, base);
    } else if *def_result >= 12 {
        // hit turned to out, runners hold
        match state.outs {
            Outs::None => {
                state.outs = Outs::One;
            }
            Outs::One => {
                state.outs = Outs::Two;
            }
            Outs::Two => {
                state.outs = Outs::Three;
            }
            Outs::Three => {
                state.outs = Outs::Three;
            }
        }
        match state.inning_half {
            InningTB::Top => {
                state.hits_team2 -= 1;
            }
            InningTB::Bottom => {
                state.hits_team1 -= 1;
            }
        }
        base = 0;
        advance = 0;
        return (state, advance, base);
    } else {
        return (state, advance, base);
    }
}
// advance runners function - handles base runners and scoring after a hit/etc.
// for now I think the best way is to handle advancing runners first, then add the batter after
pub fn runners_advance<'b>(mut state: GameState<'b>, advance_num: &u32) -> GameState<'b> {
    if *advance_num == 1 {
        // move 1
        match state.runners {
            RunnersOn::Runner000 => {
                return state;
            } // no runners on, don't do anything
            RunnersOn::Runner100 => {
                state.runners = RunnersOn::Runner010;
                return state;
            }
            RunnersOn::Runner010 => {
                state.runners = RunnersOn::Runner001;
                return state;
            }
            RunnersOn::Runner001 => {
                // runner scores, clear bases and update box score
                state.runners = RunnersOn::Runner000;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner110 => {
                state.runners = RunnersOn::Runner011;
                return state;
            }
            RunnersOn::Runner011 => {
                state.runners = RunnersOn::Runner001;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner101 => {
                state.runners = RunnersOn::Runner010;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner111 => {
                state.runners = RunnersOn::Runner011;
                match state.inning_half {
                    InningTB::Top => {
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
        }
    } else if *advance_num == 2 {
        // move 2
        match state.runners {
            RunnersOn::Runner000 => {
                return state;
            } // no runners on, don't do anything
            RunnersOn::Runner100 => {
                state.runners = RunnersOn::Runner001;
                return state;
            }
            RunnersOn::Runner010 => {
                state.runners = RunnersOn::Runner000;
                match state.inning_half {
                    InningTB::Top => {
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner001 => {
                // runner scores, clear bases and update box score
                state.runners = RunnersOn::Runner000;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner110 => {
                state.runners = RunnersOn::Runner001;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner011 => {
                state.runners = RunnersOn::Runner000;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 2;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 2;
                    }
                }
                return state;
            }
            RunnersOn::Runner101 => {
                state.runners = RunnersOn::Runner001;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.runs_team2 += 1;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 1;
                    }
                }
                return state;
            }
            RunnersOn::Runner111 => {
                state.runners = RunnersOn::Runner001;
                match state.inning_half {
                    InningTB::Top => {
                        state.runs_team2 += 2;
                    }
                    InningTB::Bottom => {
                        state.runs_team1 += 2;
                    }
                }
                return state;
            }
        }
    } else if *advance_num == 3 {
        // all runners score
        let num_runners = runnerson(&state);
        state.runners = RunnersOn::Runner000;
        match state.inning_half {
            InningTB::Top => {
                // away team at bat, update team 2 score
                state.runs_team2 += num_runners;
            }
            InningTB::Bottom => {
                state.runs_team1 += num_runners;
            }
        }
        return state;
    } else {
        return state;
    }
}

// gets number of runners on base
pub fn runnerson(state: &GameState) -> u32 {
    match state.runners {
        RunnersOn::Runner000 => {
            return 0;
        }
        RunnersOn::Runner100 => {
            return 1;
        }
        RunnersOn::Runner010 => {
            return 1;
        }
        RunnersOn::Runner001 => {
            return 1;
        }
        RunnersOn::Runner110 => {
            return 2;
        }
        RunnersOn::Runner101 => {
            return 2;
        }
        RunnersOn::Runner011 => {
            return 2;
        }
        RunnersOn::Runner111 => {
            return 3;
        }
    }
}

// function to put a hitter onto the bases
// certain conditions shouldn't come up ever, so just skip them
pub fn add_runner<'b>(mut state: GameState<'b>, base: &u32) -> GameState<'b> {
    match state.runners {
        RunnersOn::Runner000 => {
            if *base == 1 {
                state.runners = RunnersOn::Runner100;
            } else if *base == 2 {
                state.runners = RunnersOn::Runner010;
            } else if *base == 3 {
                state.runners = RunnersOn::Runner001;
            }
            return state;
        }
        RunnersOn::Runner100 => {
            // skip 1 in this case
            if *base == 2 {
                state.runners = RunnersOn::Runner110;
            } else if *base == 3 {
                state.runners = RunnersOn::Runner101;
            }
            return state;
        }
        RunnersOn::Runner010 => {
            // skip 2 in this case
            if *base == 1 {
                state.runners = RunnersOn::Runner110;
            } else if *base == 3 {
                state.runners = RunnersOn::Runner011;
            }
            return state;
        }
        RunnersOn::Runner001 => {
            // skip 3
            if *base == 1 {
                state.runners = RunnersOn::Runner101;
            } else if *base == 2 {
                state.runners = RunnersOn::Runner011;
            }
            return state;
        }
        RunnersOn::Runner110 => {
            // skip 1 and 2
            if *base == 3 {
                state.runners = RunnersOn::Runner111;
            }
            return state;
        }
        RunnersOn::Runner101 => {
            // skip 1 and 3
            if *base == 2 {
                state.runners = RunnersOn::Runner111;
            }
            return state;
        }
        RunnersOn::Runner011 => {
            // skip 2 and 3
            if *base == 1 {
                state.runners = RunnersOn::Runner111;
            }
            return state;
        }
        RunnersOn::Runner111 => {
            // nothing to do on this one
            return state;
        }
    }
}

// function to get last digit of swing_result - used for determining which fielder makes the out
pub fn get_swing_position(pitch_result: &i32) -> i32 {
    let last_digit = *pitch_result % 10;
    return last_digit;
}

// convenience function to initialize a game state struct
pub fn init_new_game_state<'a>(
    home_pitcher: &'a Player,
    away_pitcher: &'a Player,
) -> GameState<'a> {
    let game_state = GameState {
        status: GameStatus::NotStarted,
        inning: 1,
        inning_half: InningTB::Top,
        outs: Outs::None,
        runners: RunnersOn::Runner000,
        batting_team1: 0,
        batting_team2: 0,
        current_pitcher_team1: home_pitcher,
        current_pitcher_team2: away_pitcher,
        pitched_team1: 0,
        pitched_team2: 0,
        runs_team1: 0,
        runs_team2: 0,
        hits_team1: 0,
        hits_team2: 0,
        errors_team1: 0,
        errors_team2: 0,
    };

    return game_state;
}
