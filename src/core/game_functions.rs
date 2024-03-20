/*========================================================
MODULE INCLUSIONS
========================================================*/
use text_colorizer::*;

use crate::characters::ballparks::BallparkModern;
use crate::characters::{players::*, teams::*};
use crate::core::roll;
use crate::gui::debug::{combined_roll, DebugConfig};

/*========================================================
ENUM DEFINITIONS
========================================================*/
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum InningTB {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outs {
    One,
    Two,
    Three,
    None,
}

// each number is base binary (1 is runner on, 0 is no runner)
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    NotStarted,
    Ongoing,
    Over,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StealType {
    Second,
    Third,
    Home,
    Double,
}

/*
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
*/

/*
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
*/

#[derive(Debug)]
pub enum Animal {
    Bird,
    Rodent,
    Cat,
    Streaker,
}

#[derive(Clone, Debug)]
enum HitAndRun {
    Hit,
    PopUpK,
    Groundball,
}

/*========================================================
STRUCT DEFINITIONS
========================================================*/
#[derive(Clone)]
pub struct GameModern {
    pub home: Team,
    pub away: Team,
    pub home_active: ActiveTeam,
    pub away_active: ActiveTeam,
    pub ballpark: BallparkModern,
    pub oddity: bool, // enables oddity roll (option rule)
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub status: GameStatus,
    pub inning: u32,
    pub inning_half: InningTB,
    pub outs: Outs,
    pub runners: RunnersOn,
    pub runner1: Option<Player>,
    pub runner2: Option<Player>,
    pub runner3: Option<Player>,
    pub home_state: TeamState,
    pub away_state: TeamState,
    pub game_text: String,
}
// NOTE: home team is team 1, away team is team 2

//======== CUSTOM ERRORS =================================
#[derive(Debug, Clone)]
pub struct TeamError {
    pub message: String,
    pub team: String,
}

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
/// takes MSS and batter targets, return AtBatResults enum
/// oddity is boolean indicating if oddity rules are enabled
pub fn at_bat(bat_target: i32, on_base_target: i32, mss_result: i32, oddity: bool) -> AtBatResults {
    let mut at_bat_result = AtBatResults::MegaOut;

    if mss_result == 1 {
        if oddity {
            at_bat_result = AtBatResults::Oddity;
        } else {
            at_bat_result = AtBatResults::CriticalHit;
        }
    } else if (2..=5).contains(&mss_result) {
        at_bat_result = AtBatResults::CriticalHit;
    } else if mss_result >= 6 && mss_result <= bat_target {
        at_bat_result = AtBatResults::Hit;
    } else if mss_result > bat_target && mss_result <= on_base_target {
        at_bat_result = AtBatResults::Walk;
    } else if mss_result > on_base_target && mss_result <= on_base_target + 5 {
        at_bat_result = AtBatResults::PossibleError;
    } else if mss_result >= on_base_target + 6 && mss_result <= 49 {
        at_bat_result = AtBatResults::ProductiveOut1;
    } else if (50..=69).contains(&mss_result) {
        at_bat_result = AtBatResults::ProductiveOut2;
    } else if (70..=98).contains(&mss_result) {
        at_bat_result = AtBatResults::Out;
    } else if mss_result == 99 {
        if oddity {
            at_bat_result = AtBatResults::Oddity;
        } else {
            at_bat_result = AtBatResults::Out;
        }
    } else if mss_result >= 100 {
        at_bat_result = AtBatResults::MegaOut;
    }

    at_bat_result
}

/// creates a GameModern struct
pub fn create_modern_game(
    home: Team,
    away: Team,
    ballpark: BallparkModern,
    oddity: bool,
) -> Result<GameModern, TeamError> {
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
    // TODO: I feel like load_roster function could be used here?
    let mut home_active = ActiveTeam {
        roster: vec![],
        bench: vec![],
        pitching: vec![],
        bullpen: vec![],
        batting_order: vec![],
    };
    (
        home_active.roster,
        home_active.bench,
        home_active.pitching,
        home_active.bullpen,
    ) = load_roster(&home);
    /*
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
    */
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
    (
        away_active.roster,
        away_active.bench,
        away_active.pitching,
        away_active.bullpen,
    ) = load_roster(&away);
    /*
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
    */
    // for now, make batting order roster + pitcher
    away_active.batting_order = away_active.roster.clone();
    away_active
        .batting_order
        .push(away_active.pitching[0].clone());

    let game = GameModern {
        home,
        away,
        ballpark,
        home_active,
        away_active,
        oddity,
    };

    Ok(game)
}

/// call to enter core game logic loop
pub fn modern_game_flow(game: &GameModern, mut state: GameState, debug: DebugConfig) -> GameState {
    // check top of the 9th at a different place
    if state.inning > 9 {
        // check score
        if state.home_state.runs != state.away_state.runs {
            state.status = GameStatus::Over;
            // TODO: I think this message is redundant
            state.game_text += &format!(
                "\nGame!  Final score: {} - {}",
                state.home_state.runs.iter().sum::<u32>(),
                state.away_state.runs.iter().sum::<u32>()
            );
        }
    }
    match state.status {
        GameStatus::NotStarted => {
            state.status = GameStatus::Ongoing;
            println!("Play ball!");
            state.game_text += "\nPlay ball!";
        }
        GameStatus::Ongoing => match state.inning_half {
            InningTB::Top => {
                match state.outs {
                    Outs::Three => {
                        // clean up game state, reset for new inning
                        state.inning_half = InningTB::Bottom;
                        state.outs = Outs::None;
                        state.runners = RunnersOn::Runner000;
                        state.runner1 = None;
                        state.runner2 = None;
                        state.runner3 = None;
                        state.game_text += "\nTop of the inning over.";
                    }
                    _ => {
                        state = modern_inning_flow(game, state, debug);
                    }
                }
            }
            InningTB::Bottom => {
                match state.outs {
                    Outs::Three => {
                        state.inning_half = InningTB::Top;
                        state.runners = RunnersOn::Runner000;
                        state.runner1 = None;
                        state.runner2 = None;
                        state.runner3 = None;
                        state.outs = Outs::None; // reset outs
                        state.inning += 1;
                        state.game_text += "\nBottom of the inning over.";
                    }
                    _ => {
                        state = modern_inning_flow(game, state, debug);
                    }
                }
            }
        },
        GameStatus::Over => {
            // temporary printing of results
            // TODO: print score report?
            // TODO: inning ticks over one final time before game ends, need to fix
            println!("FINAL SCORE");
            println!(
                "HOME: {} - AWAY: {}",
                state.home_state.runs.iter().sum::<u32>(),
                state.away_state.runs.iter().sum::<u32>()
            );
            state.game_text += &format!(
                "\nThat's game!  Final score: {} - {}",
                state.home_state.runs.iter().sum::<u32>(),
                state.away_state.runs.iter().sum::<u32>()
            );
        }
    }

    state
}

/// runs each half inning
pub fn modern_inning_flow(
    game: &GameModern,
    mut state: GameState,
    mut debug: DebugConfig,
) -> GameState {
    match state.inning_half {
        InningTB::Top => {
            // should match Bottom arm, just flip the teams - probably a better way to do this
            match state.outs {
                Outs::Three => state,
                _ => {
                    // get active batter
                    // get at bat Result
                    // update score/runners/Outs
                    let batter = game.away_active.batting_order
                        [state.away_state.current_batter as usize]
                        .clone();
                    let mut pd = state.home_state.current_pitcher.pitch_die;
                    // NOTE: special rules for GB+
                    if state.runners == RunnersOn::Runner111 {
                        pd = change_pitch_die(pd, 1);
                    }
                    // NOTE: handedness check
                    if state.home_state.current_pitcher.handedness == batter.handedness {
                        pd = change_pitch_die(pd, 1);
                        // TODO: make distinction between starting pitcher and reliever
                    }
                    let mut pitch_mod: i32 = 0;
                    if state.home_state.current_pitcher.strikeout() {
                        pitch_mod = -1;
                    }
                    let control_mod = state.home_state.current_pitcher.control();
                    let pitch_result = if pd > 0 {
                        combined_roll(&mut debug, pd)
                    } else {
                        -combined_roll(&mut debug, pd.abs())
                    };
                    state.game_text += &format!("\n\nPitch result: {}", &pitch_result);
                    let mss_result = pitch_result + combined_roll(&mut debug, 100);
                    let mut hit_mod: i32 = 0;
                    if batter.free_swing() {
                        match state.runners {
                            RunnersOn::Runner010 => hit_mod = -3,
                            RunnersOn::Runner001 => hit_mod = -3,
                            RunnersOn::Runner110 => hit_mod = -3,
                            RunnersOn::Runner101 => hit_mod = -3,
                            RunnersOn::Runner011 => hit_mod = -3,
                            RunnersOn::Runner111 => hit_mod = -3,
                            _ => hit_mod = 0,
                        }
                    }
                    state.game_text += &format!("\nMSS: {}", &mss_result);
                    let swing_result = at_bat(
                        batter.batter_target + pitch_mod + hit_mod,
                        batter.on_base_target + control_mod + hit_mod,
                        mss_result,
                        game.oddity,
                    );
                    state.game_text += &format!(" -> {:?}", swing_result);
                    if state.away_state.current_batter == 8 {
                        state.away_state.current_batter = 0;
                    } else {
                        state.away_state.current_batter += 1;
                    }

                    match swing_result {
                        AtBatResults::Oddity => {
                            let oddity_result =
                                combined_roll(&mut debug, 10) + combined_roll(&mut debug, 10);
                            state.game_text += &format!("\n Oddity roll: {}", &oddity_result);
                            state = oddity(&mut debug, &oddity_result, &pitch_result, game, state);
                        }
                        AtBatResults::CriticalHit => {
                            // make hit roll, bump up a level
                            let mut hit_result =
                                combined_roll(&mut debug, 20) + pow_trait_check(game, &state);
                            state.game_text += &format!("\nCrit hit roll: {}", &hit_result);
                            hit_result = crit_hit(&hit_result);
                            state = hit_table(&hit_result, state, game, &mut debug);
                            // TODO: no DEF roll on crit_hit
                        }
                        AtBatResults::Hit => {
                            // hit roll
                            let hit_result =
                                combined_roll(&mut debug, 20) + pow_trait_check(game, &state);
                            state.game_text += &format!("\nHit roll: {}", &hit_result);
                            state = hit_table(&hit_result, state, game, &mut debug);
                        }
                        AtBatResults::Walk => {
                            // basically like a single, just don't update the hit values
                            state.game_text += "\n Walk.";
                            state = runners_advance(state, &1);
                            let batter = game.away_active.batting_order
                                [(state.away_state.current_batter - 2) as usize]
                                .clone();
                            state = add_runner(state, &1, batter);
                        }
                        AtBatResults::PossibleError => {
                            state = possible_error(
                                &mut debug,
                                state,
                                game,
                                position_by_number(get_swing_position(&mss_result)),
                            );
                        }
                        AtBatResults::ProductiveOut1 => {
                            state = productive_out1(state, &mss_result);
                        }
                        AtBatResults::ProductiveOut2 => {
                            let batter = game.away_active.batting_order
                                [(state.away_state.current_batter - 2) as usize]
                                .clone();
                            state = productive_out2(state, &mss_result, batter);
                        }
                        AtBatResults::Out => {
                            state = actual_out(state, &mss_result);
                        }
                        AtBatResults::MegaOut => {
                            state = mega_out(state);
                        }
                    }

                    state
                }
            }
        }
        InningTB::Bottom => {
            match state.outs {
                Outs::Three => state,
                _ => {
                    // get active batter
                    // get at bat Result
                    // update score/runners/Outs
                    let batter = game.home_active.batting_order
                        [state.home_state.current_batter as usize]
                        .clone();
                    let mut pd = state.away_state.current_pitcher.pitch_die;
                    // NOTE: special rules for GB+
                    if state.runners == RunnersOn::Runner111 {
                        pd = change_pitch_die(pd, 1);
                    }
                    // NOTE: handedness check
                    if state.away_state.current_pitcher.handedness == batter.handedness {
                        pd = change_pitch_die(pd, 1);
                        // TODO: make distinction between starting pitcher and reliever
                    }
                    let mut pitch_mod = 0;
                    if state.away_state.current_pitcher.strikeout() {
                        pitch_mod = -1;
                    }
                    let control_mod = state.away_state.current_pitcher.control();
                    let pitch_result = if pd > 0 {
                        combined_roll(&mut debug, pd)
                    } else {
                        -combined_roll(&mut debug, pd.abs())
                    };
                    state.game_text += &format!("\n\nPitch result: {}", &pitch_result);
                    let mss_result = pitch_result + combined_roll(&mut debug, 100);
                    state.game_text += &format!("\nMSS: {}", &mss_result);
                    let mut hit_mod: i32 = 0;
                    if batter.free_swing() {
                        match state.runners {
                            RunnersOn::Runner010 => hit_mod = -3,
                            RunnersOn::Runner001 => hit_mod = -3,
                            RunnersOn::Runner110 => hit_mod = -3,
                            RunnersOn::Runner101 => hit_mod = -3,
                            RunnersOn::Runner011 => hit_mod = -3,
                            RunnersOn::Runner111 => hit_mod = -3,
                            _ => hit_mod = 0,
                        }
                    }
                    let swing_result = at_bat(
                        batter.batter_target + pitch_mod + hit_mod,
                        batter.on_base_target + control_mod + hit_mod,
                        mss_result,
                        game.oddity,
                    );
                    state.game_text += &format!(" -> {:?}", swing_result);
                    if state.home_state.current_batter == 8 {
                        state.home_state.current_batter = 0;
                    } else {
                        state.home_state.current_batter += 1;
                    }

                    match swing_result {
                        AtBatResults::Oddity => {
                            let oddity_result =
                                combined_roll(&mut debug, 10) + combined_roll(&mut debug, 10);
                            state.game_text += &format!("\n Oddity roll: {}", &oddity_result);
                            state = oddity(&mut debug, &oddity_result, &mss_result, game, state);
                        }
                        AtBatResults::CriticalHit => {
                            // make hit roll, bump up a level
                            let mut hit_result =
                                combined_roll(&mut debug, 20) + pow_trait_check(game, &state);
                            state.game_text += &format!("\nCrit hit roll: {}", &hit_result);
                            hit_result = crit_hit(&hit_result);
                            state = hit_table(&hit_result, state, game, &mut debug);
                            // TODO: no DEF roll on crit_hit
                        }
                        AtBatResults::Hit => {
                            // hit roll
                            let hit_result =
                                combined_roll(&mut debug, 20) + pow_trait_check(game, &state);
                            state.game_text += &format!("\nHit roll: {}", &hit_result);
                            state = hit_table(&hit_result, state, game, &mut debug);
                        }
                        AtBatResults::Walk => {
                            // basically like a single, just don't update the hit values
                            state.game_text += "\n Walk.";
                            state = runners_advance(state, &1);
                            let batter = game.home_active.batting_order
                                [(state.home_state.current_batter - 2) as usize]
                                .clone();
                            state = add_runner(state, &1, batter);
                        }
                        AtBatResults::PossibleError => {
                            state = possible_error(
                                &mut debug,
                                state,
                                game,
                                position_by_number(get_swing_position(&mss_result)),
                            );
                        }
                        AtBatResults::ProductiveOut1 => {
                            state = productive_out1(state, &mss_result);
                        }
                        AtBatResults::ProductiveOut2 => {
                            let batter = game.home_active.batting_order
                                [(state.home_state.current_batter - 2) as usize]
                                .clone();
                            state = productive_out2(state, &mss_result, batter);
                        }
                        AtBatResults::Out => {
                            state = actual_out(state, &mss_result);
                        }
                        AtBatResults::MegaOut => {
                            state = mega_out(state);
                        }
                    }

                    state
                }
            }
        }
    }
}

/// rolls on the oddity table and updates game state
pub fn oddity(
    debug: &mut DebugConfig,
    oddity_result: &i32,
    pitch_result: &i32,
    game: &GameModern, // TODO: program oddities
    mut state: GameState,
) -> GameState {
    let batter: Player;
    let batting_order: &mut u32;
    match state.inning_half {
        InningTB::Top => {
            batter = game.away_active.batting_order[(state.away_state.current_batter - 1) as usize]
                .clone();
            batting_order = &mut state.away_state.current_batter;
        }
        InningTB::Bottom => {
            batter = game.home_active.batting_order[(state.home_state.current_batter - 1) as usize]
                .clone();
            batting_order = &mut state.home_state.current_batter;
        }
    }
    if *oddity_result == 2 {
        if pitch_result % 2 == 1 {
            // fan catches sure out, at bat continues
            *batting_order -= 1;
            state.game_text += "\nFan catches a sure out, at bat continues!";
        } else {
            // home run overturned, batter out
            state.outs = increment_out(state.outs, 1);
            state.game_text += "\nHome run overturned, batter is out.";
        }
    } else if *oddity_result == 3 {
        // animal on the field
        println!("{}", "Animal on the field!".bold().yellow());
        let animal = animal(debug);
        state.game_text += &format!("\n{:?} on the field!  [development]", animal);
        *batting_order -= 1;
    } else if *oddity_result == 4 {
        // rain delay
        println!("{}", "Rain delay.".bold().cyan());
        let delay = roll(100) + roll(100);
        state.game_text += &format!("\nRain delay for {} minutes.", delay);
        *batting_order -= 1;
    } else if *oddity_result == 5 {
        // player injured
        state.game_text += "\nPlayer injured!  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 6 {
        // pitcher appears injured
        state.game_text += "\nPitcher inured!  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 7 {
        // TOOTBLAN
        state.game_text += "\nTOOTBLAN [development]";
        *batting_order -= 1;
    } else if *oddity_result == 8 {
        // pick off
        state.game_text += "\nPick off!  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 9 {
        // call blown at first
        state.game_text += "\nCall blown at first [development]";
        *batting_order -= 1;
    } else if *oddity_result == 10 {
        // call blown at home
        state.game_text += "\nCall blown at home [development]";
        *batting_order -= 1;
    } else if *oddity_result == 11 {
        // hit by pitch
        state.game_text += "\nHit by pitch!";
        state = force_advance(state, 1);
        state = add_runner(state, &1, batter);
    } else if *oddity_result == 12 {
        // wild pitch
        state.game_text += "\nWild pitch! [development]";
        *batting_order -= 1;
    } else if *oddity_result == 13 {
        // pitcher distracted
        state.game_text += "\nPitcher distracted.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 14 {
        // dropped third strike
        state.game_text += "\nDropped 3rd strike.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 15 {
        // passed ball
        state.game_text += "\nPassed ball.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 16 {
        // current batter appears injured
        state.game_text += "\nCurrent batter appears injured.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 17 {
        // previous batter appears injured
        state.game_text += "\nPrevious batter appears injured.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 18 {
        // pitcher error
        state.game_text += "\nPitcher error.  [development]";
        *batting_order -= 1;
    } else if *oddity_result == 19 {
        // balk
        state.game_text += "\nBalk!";
        state = force_advance(state, 1);
        state = add_runner(state, &1, batter);
    } else if *oddity_result == 20 {
        // catcher interference
        *batting_order -= 1;
        state.game_text += "\nCatcher interference.";
        state = force_advance(state, 1);
        state = add_runner(state, &1, batter);
    } else {
        state.game_text += "\nYou shouldn't get here (oddity roll > 20 somehow)";
        *batting_order -= 1;
    }

    state
}

/// bumps hit roll up a level on the hit table
pub fn crit_hit(hit_result: &i32) -> i32 {
    // based on 2E Deadball quick reference hit table
    let mut crit_result: i32 = *hit_result;
    if (1..=2).contains(hit_result) || (7..=9).contains(hit_result) {
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

    crit_result
}

/// rolls on the hit table and updates game state accordingly
pub fn hit_table(
    hit_result: &i32,
    mut state: GameState,
    game: &GameModern,
    debug: &mut DebugConfig,
) -> GameState {
    // 1. defense roll (if needed)
    // 2. advance runners
    // 3 move hitter to runner
    // 4. update hit values in game state
    // get batter
    let batter = match state.inning_half {
        InningTB::Top => {
            game.away_active.batting_order[(state.away_state.current_batter - 1) as usize].clone()
        }
        InningTB::Bottom => {
            game.home_active.batting_order[(state.home_state.current_batter - 1) as usize].clone()
        }
    };
    if *hit_result <= 2 {
        if batter.speedy() {
            // NOTE: special rules for S+
            // on 1: batter doubles, runners advance 2, no DEF roll
            // on 2: batter triples, do not roll for defense
            if *hit_result == 1 {
                state = runners_advance(state, &2);
                state = add_runner(state, &2, batter);
                state.game_text += " -> Double (S+)";
            } else {
                state = runners_advance(state, &3);
                state = add_runner(state, &3, batter);
                state.game_text += " -> Triple (S+)";
            }
        } else {
            // NOTE: special rules for C+ (S+ is better if batter has both)
            // on 1-2 batter doubles, runners advance 2, no DEF
            if batter.contact_hit() {
                state = runners_advance(state, &2);
                state = add_runner(state, &2, batter);
                state.game_text += " -> Double (C+)";
            } else {
                state.game_text += " -> Single";
                // single
                state = runners_advance(state, &1);
                state = add_runner(state, &1, batter);
                // simple hit increment when no defense roll involved
                match state.inning_half {
                    InningTB::Top => {
                        state.away_state.hits[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.hits[(state.inning - 1) as usize] += 1;
                    }
                }
            }
        }

        state
    } else if *hit_result == 3 {
        // single DEF 1B
        state.game_text += " -> Single DEF 1B";
        let mut advance = 1;
        let mut base = 1;
        // when a defense roll is involved, add hit first and then you can subtract if there is an
        // out or an error
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[state.inning as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[state.inning as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Firstbase);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 4 {
        state.game_text += " -> Single DEF 2B";
        // single DEF 2B
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Secondbase);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 5 {
        state.game_text += " -> Single DEF 3B";
        // single DEF 3B
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Thirdbase);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 6 {
        state.game_text += " -> Single DEF SS";
        // single DEF SS
        let mut advance = 1;
        let mut base = 1;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Shortstop);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result >= 7 && *hit_result <= 9 {
        state.game_text += " -> Single";
        // single
        state = runners_advance(state, &1);
        state = add_runner(state, &1, batter);
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        return state;
    } else if *hit_result >= 10 && *hit_result <= 14 {
        state.game_text += " -> Single, runners advance 2";
        // single, runners advance 2
        state = runners_advance(state, &2);
        state = add_runner(state, &1, batter);
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        return state;
    } else if *hit_result == 15 {
        state.game_text += " -> Double DEF LF";
        // double DEF LF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Leftfield);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 16 {
        state.game_text += " -> Double, DEF CF";
        // double DEF CF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Centerfield);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 17 {
        state.game_text += " -> Double DEF RF";
        // double DEF RF
        let mut advance = 2;
        let mut base = 2;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        let def_roll = combined_roll(debug, 12)
            + def_trait_check(&state.inning_half, game, Position::Rightfield);
        (state, advance, base) = defense(state, &def_roll, advance, base);
        state = runners_advance(state, &advance);
        state = add_runner(state, &base, batter);
        return state;
    } else if *hit_result == 18 {
        state.game_text += " -> Double, runners advance 3";
        // double, runners advance 3
        state = runners_advance(state, &3);
        state = add_runner(state, &2, batter);
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        return state;
    } else if *hit_result >= 19 {
        state.game_text += " -> HOME RUN!";
        // home run
        let mut runs = runnerson(&state);
        runs += 1;
        state.runners = RunnersOn::Runner000;
        state.runner1 = None;
        state.runner2 = None;
        state.runner3 = None;
        match state.inning_half {
            InningTB::Top => {
                state.away_state.runs[(state.inning - 1) as usize] += runs;
            }
            InningTB::Bottom => {
                state.home_state.runs[(state.inning - 1) as usize] += runs;
            }
        }
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] += 1;
            }
        }
        return state;
    } else {
        return state;
    }
}

// TODO: find position player function - finds player info based on position and inning

/// defense roll function - rolls on the defense table and updates game state
pub fn defense(
    mut state: GameState,
    def_result: &i32,
    mut advance: u32,
    mut base: u32,
) -> (GameState, u32, u32) {
    state.game_text += &format!("\n Defense roll: {}", def_result);
    if *def_result <= 2 {
        state.game_text += " -> Error";
        // error, runners take an extra base
        // modify hit and error values
        // should be okay to subtract here since hit was added before passing into this function
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] -= 1;
                state.home_state.errors[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] -= 1;
                state.away_state.errors[(state.inning - 1) as usize] += 1;
            }
        }
        (state, advance + 1, base + 1)
    } else if *def_result >= 3 && *def_result <= 9 {
        state.game_text += " -> Normal";
        // no change
        (state, advance, base)
    } else if *def_result >= 10 && *def_result <= 11 {
        state.game_text += " -> good defense, reduce hit level by 1";
        // double turns to single, runners advance 2, single turns to out, runners advance 1
        if base == 1 {
            state.outs = increment_out(state.outs, 1);
            base = 0;
            advance = 1;
        } else if base == 2 {
            base = 1;
            advance = 2;
        }
        (state, advance, base)
    } else if *def_result >= 12 {
        state.game_text += " -> Out!  What a play, Runners hold.";
        // hit turned to out, runners hold
        state.outs = increment_out(state.outs, 1);
        match state.inning_half {
            InningTB::Top => {
                state.away_state.hits[(state.inning - 1) as usize] -= 1;
            }
            InningTB::Bottom => {
                state.home_state.hits[(state.inning - 1) as usize] -= 1;
            }
        }
        base = 0;
        advance = 0;
        (state, advance, base)
    } else {
        (state, advance, base)
    }
}
/// advance runners function - handles base runners and scoring after a hit/etc.
/// this function just clones state.runner1/2/3 so make sure you already have players in the right
/// spot
/// for now I think the best way is to handle advancing runners first, then add the batter after
pub fn runners_advance(mut state: GameState, advance_num: &u32) -> GameState {
    if *advance_num == 1 {
        // move 1
        match state.runners {
            RunnersOn::Runner000 => {} // no runners on, don't do anything
            RunnersOn::Runner100 => {
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
            }
            RunnersOn::Runner010 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
            }
            RunnersOn::Runner001 => {
                // runner scores, clear bases and update box score
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner110 => {
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                return state;
            }
            RunnersOn::Runner011 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner101 => {
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state.runner3 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner111 => {
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => {
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
        }
    } else if *advance_num == 2 {
        // move 2
        match state.runners {
            RunnersOn::Runner000 => {} // no runners on, don't do anything
            RunnersOn::Runner100 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner1 = None;
            }
            RunnersOn::Runner010 => {
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => {
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner001 => {
                // runner scores, clear bases and update box score
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner110 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner011 => {
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                state.runner3 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 2;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 2;
                    }
                }
            }
            RunnersOn::Runner101 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => {
                        // away team at bat, update team 2 score
                        state.away_state.runs[(state.inning - 1) as usize] += 1;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 1;
                    }
                }
            }
            RunnersOn::Runner111 => {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner1 = None;
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => {
                        state.away_state.runs[(state.inning - 1) as usize] += 2;
                    }
                    InningTB::Bottom => {
                        state.home_state.runs[(state.inning - 1) as usize] += 2;
                    }
                }
            }
        }
    } else if *advance_num == 3 {
        // all runners score
        let num_runners = runnerson(&state);
        state.runners = RunnersOn::Runner000;
        state.runner1 = None;
        state.runner2 = None;
        state.runner3 = None;
        match state.inning_half {
            InningTB::Top => {
                // away team at bat, update team 2 score
                state.away_state.runs[(state.inning - 1) as usize] += num_runners;
            }
            InningTB::Bottom => {
                state.home_state.runs[(state.inning - 1) as usize] += num_runners;
            }
        }
    }

    state
}

/// gets number of runners on base
pub fn runnerson(state: &GameState) -> u32 {
    match state.runners {
        RunnersOn::Runner000 => 0,
        RunnersOn::Runner100 => 1,
        RunnersOn::Runner010 => 1,
        RunnersOn::Runner001 => 1,
        RunnersOn::Runner110 => 2,
        RunnersOn::Runner101 => 2,
        RunnersOn::Runner011 => 2,
        RunnersOn::Runner111 => 3,
    }
}

/// function to put a hitter onto the bases
/// clone the current batter from GameModern struct roster to put on base
pub fn add_runner(mut state: GameState, base: &u32, batter: Player) -> GameState {
    // certain conditions shouldn't come up ever, so just skip them
    match state.runners {
        RunnersOn::Runner000 => {
            if *base == 1 {
                state.runners = RunnersOn::Runner100;
                state.runner1 = Some(batter);
            } else if *base == 2 {
                state.runners = RunnersOn::Runner010;
                state.runner2 = Some(batter);
            } else if *base == 3 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = Some(batter);
            }
        }
        RunnersOn::Runner100 => {
            // TODO: is it even possible to get here? after advancing, no one should be on first
            // skip 1 in this case
            if *base == 2 {
                state.runners = RunnersOn::Runner110;
                state.runner2 = Some(batter);
            } else if *base == 3 {
                state.runners = RunnersOn::Runner101;
                state.runner3 = Some(batter);
            }
        }
        RunnersOn::Runner010 => {
            // skip 2 in this case
            if *base == 1 {
                state.runners = RunnersOn::Runner110;
                state.runner1 = Some(batter);
            } else if *base == 3 {
                // TODO: delete these unecessary branches
                state.runners = RunnersOn::Runner011;
                state.runner3 = Some(batter);
            }
        }
        RunnersOn::Runner001 => {
            // skip 3
            if *base == 1 {
                state.runners = RunnersOn::Runner101;
                state.runner1 = Some(batter);
            } else if *base == 2 {
                state.runners = RunnersOn::Runner011;
                state.runner2 = Some(batter);
            }
        }
        RunnersOn::Runner110 => {
            // skip 1 and 2
            if *base == 3 {
                // TODO: delete
                state.runners = RunnersOn::Runner111;
                state.runner3 = Some(batter);
            }
        }
        RunnersOn::Runner101 => {
            // skip 1 and 3
            if *base == 2 {
                // TODO: delete
                state.runners = RunnersOn::Runner111;
                state.runner2 = Some(batter);
            }
        }
        RunnersOn::Runner011 => {
            // skip 2 and 3
            if *base == 1 {
                state.runners = RunnersOn::Runner111;
                state.runner1 = Some(batter);
            }
        }
        RunnersOn::Runner111 => {
            // nothing to do on this one
        }
    }

    state
}

/// function to get last digit of swing_result - used for determining which fielder makes the out
pub fn get_swing_position(mss_result: &i32) -> i32 {
    *mss_result % 10
}

// TODO: fn default under struct definition instead?
/// convenience function to initialize a game state struct
pub fn init_new_game_state(home_pitcher: Player, away_pitcher: Player) -> GameState {
    let home_state = TeamState {
        current_batter: 0,
        current_pitcher: home_pitcher,
        innings_pitched: 0,
        runs: vec![0],
        hits: vec![0],
        errors: vec![0],
    };
    let away_state = TeamState {
        current_batter: 0,
        current_pitcher: away_pitcher,
        innings_pitched: 0,
        runs: vec![0],
        hits: vec![0],
        errors: vec![0],
    };
    GameState {
        status: GameStatus::NotStarted,
        inning: 1,
        inning_half: InningTB::Top,
        outs: Outs::None,
        runners: RunnersOn::Runner000,
        runner1: None,
        runner2: None,
        runner3: None,
        home_state,
        away_state,
        game_text: "Game created.".to_string(),
    }
}

// TODO: find a player by position in roster\
/// Finds the player in a certain position.  Takes a reference to a roster (active team struct) and
/// returns a copy of the desired player struct
pub fn find_by_position(position: Position, roster: &[Player]) -> Option<Player> {
    for player in roster.iter() {
        if player.position == position {
            return Some(player.clone());
        }
    }
    None
}

/// convert MSS digit to position
pub fn position_by_number(mut last_digit: i32) -> Position {
    let position: Position;
    if last_digit < 1 {
        last_digit = 1;
    }
    if last_digit > 9 {
        last_digit = 9;
    }
    if last_digit == 1 {
        position = Position::Pitcher;
    } else if last_digit == 2 {
        position = Position::Catcher;
    } else if last_digit == 3 {
        position = Position::Firstbase;
    } else if last_digit == 4 {
        position = Position::Secondbase;
    } else if last_digit == 5 {
        position = Position::Shortstop;
    } else if last_digit == 6 {
        position = Position::Thirdbase;
    } else if last_digit == 7 {
        position = Position::Leftfield;
    } else if last_digit == 8 {
        position = Position::Centerfield;
    } else {
        position = Position::Rightfield;
    }

    position
}

/// convenience function to return a default GameState struct
pub fn new_game_state_struct() -> GameState {
    let home_state = TeamState {
        current_batter: 1,
        current_pitcher: generate_player(
            PlayerClass::Pitchers,
            Position::Pitcher,
            &["Seth".to_string()],
            &["Loveall".to_string()],
        ),
        innings_pitched: 0,
        runs: vec![0],
        hits: vec![0],
        errors: vec![0],
    };
    let away_state = TeamState {
        current_batter: 1,
        current_pitcher: generate_player(
            PlayerClass::Pitchers,
            Position::Pitcher,
            &["Seth".to_string()],
            &["Loveall".to_string()],
        ),
        innings_pitched: 0,
        runs: vec![0],
        hits: vec![0],
        errors: vec![0],
    };
    GameState {
        status: GameStatus::NotStarted,
        inning: 1,
        inning_half: InningTB::Top,
        outs: Outs::None,
        runners: RunnersOn::Runner000,
        runner1: None,
        runner2: None,
        runner3: None,
        home_state,
        away_state,
        game_text: "Game created.".to_string(),
    }
}

/// handle PossibleError swing result
fn possible_error(
    debug: &mut DebugConfig,
    mut state: GameState,
    game: &GameModern,
    position: Position,
) -> GameState {
    // TODO: Not sure I am implementing this correctly, see page 29
    // get position
    // TODO: get player traits
    let batter: Player = match state.inning_half {
        InningTB::Top => {
            game.away_active.batting_order[(state.away_state.current_batter - 1) as usize].clone()
        }
        InningTB::Bottom => {
            game.home_active.batting_order[(state.home_state.current_batter - 1) as usize].clone()
        }
    };
    state.game_text += "\n Possible error -> ";
    let def_roll = combined_roll(debug, 12) + def_trait_check(&state.inning_half, game, position);
    state.game_text += &format!("defense roll: {}", &def_roll);
    if def_roll <= 2 {
        state.game_text += "-> Error!";
        // fielder makes an error
        // TODO: these kind of match statements are redundant, clean it up
        match state.inning_half {
            InningTB::Top => {
                state.home_state.errors[(state.inning - 1) as usize] += 1;
            }
            InningTB::Bottom => {
                state.away_state.errors[(state.inning - 1) as usize] += 1;
            }
        }
        state = runners_advance(state, &1);
        state = add_runner(state, &1, batter);
    } else {
        state.game_text += "-> No error.  Out!";
        // fielder makes the out like normal
        state.outs = increment_out(state.outs, 1);
    }

    state
}

/// handles ProductiveOut1 swing result
fn productive_out1(mut state: GameState, mss_result: &i32) -> GameState {
    // if first or outfield, runners on 2nd and 3rd advance
    // if 2B/SS/3B, runner at first advances and batter is out
    match state.outs {
        Outs::Three => {}
        Outs::Two => {
            state.outs = Outs::Three;
        }
        _ => {
            state.game_text += "\nPossible productive out (type 1).";
            let fielder = get_swing_position(mss_result);
            if fielder == 3 || fielder >= 7 {
                // check for runners on second and third
                // advance if they exist
                state.game_text += "\nRunners on second and third advance.";
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
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                    }
                    RunnersOn::Runner101 => {
                        state.runners = RunnersOn::Runner100;
                        state.runner3 = None;
                        match state.inning_half {
                            InningTB::Top => {
                                state.away_state.runs[(state.inning - 1) as usize] += 1;
                            }
                            InningTB::Bottom => {
                                state.home_state.runs[(state.inning - 1) as usize] += 1;
                            }
                        }
                    }
                    RunnersOn::Runner111 => {
                        state.runners = RunnersOn::Runner101;
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                        match state.inning_half {
                            InningTB::Top => {
                                state.away_state.runs[(state.inning - 1) as usize] += 1;
                            }
                            InningTB::Bottom => {
                                state.home_state.runs[(state.inning - 1) as usize] += 1;
                            }
                        }
                    }
                }
            } else {
                let pitcher: &Player = match state.inning_half {
                    InningTB::Top => &state.home_state.current_pitcher,
                    InningTB::Bottom => &state.away_state.current_pitcher,
                };
                // check for runner on first
                match state.runners {
                    RunnersOn::Runner100 => {
                        // NOTE: special rules for GB+ pitchers
                        if pitcher.groundball() && fielder == 2 {
                            state.game_text += "\nGB+ automatic double play.";
                            state.runners = RunnersOn::Runner000;
                            state.runner1 = None;
                            state.outs = increment_out(state.outs, 1); // one extra
                        } else {
                            state.game_text += "\nRunner at first advances, batter is out.";
                            state.runners = RunnersOn::Runner010;
                            state.runner2 = state.runner1.clone();
                            state.runner1 = None;
                            state.outs = increment_out(state.outs, 1);
                        }
                    }
                    RunnersOn::Runner101 => {
                        // NOTE: special rules for GB+ pitchers
                        if pitcher.groundball() && fielder == 2 {
                            state.game_text += "\nGB+ automatic double play.";
                            state.runners = RunnersOn::Runner001;
                            state.runner1 = None;
                        } else {
                            state.game_text += "\nRunner at first advances, batter is out.";
                            state.runners = RunnersOn::Runner011;
                            state.runner2 = state.runner1.clone();
                            state.runner1 = None;
                        }
                    }
                    _ => {}
                }
            }
            // update out
            state.outs = increment_out(state.outs, 1);
        }
    }

    state
}

/// handles ProductiveOut2 swing_results
fn productive_out2(mut state: GameState, mss_result: &i32, batter: Player) -> GameState {
    // if first or outfield, runners on 2nd and 3rd advance
    // if 2B/SS/3B, runner is out and batter makes it to first
    // the first line is the same as ProductiveOut1
    let pitcher: &Player = match state.inning_half {
        InningTB::Top => &state.home_state.current_pitcher,
        InningTB::Bottom => &state.away_state.current_pitcher,
    };
    match state.outs {
        Outs::Three => {}
        Outs::Two => {
            state.outs = Outs::Three;
        }
        _ => {
            state.game_text += "\nPossible producive out 2.";
            let fielder = get_swing_position(mss_result);
            if fielder == 3 || fielder >= 7 {
                state.game_text += "\nBall hit to 1B or OF, runners at 2nd and 3rd advance.";
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
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                    }
                    RunnersOn::Runner101 => {
                        state.runners = RunnersOn::Runner100;
                        state.runner3 = None;
                        match state.inning_half {
                            InningTB::Top => {
                                state.away_state.runs[(state.inning - 1) as usize] += 1;
                            }
                            InningTB::Bottom => {
                                state.home_state.runs[(state.inning - 1) as usize] += 1;
                            }
                        }
                    }
                    RunnersOn::Runner111 => {
                        state.runners = RunnersOn::Runner101;
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                        match state.inning_half {
                            InningTB::Top => {
                                state.away_state.runs[(state.inning - 1) as usize] += 1;
                            }
                            InningTB::Bottom => {
                                state.home_state.runs[(state.inning - 1) as usize] += 1;
                            }
                        }
                    }
                }
                //
            } else {
                // advance batter to first and lead runner is out
                // TODO: should this be done for force outs only
                state.game_text += "\nFielder's choice.";
                match state.runners {
                    RunnersOn::Runner000 => {}
                    RunnersOn::Runner100 => {
                        // NOTE: special rules for GB+ pitchers
                        if pitcher.groundball() && fielder == 2 {
                            state.game_text += "\nGB+ automatic double play.";
                            state.runners = RunnersOn::Runner000;
                            state.runner1 = None;
                            state.outs = increment_out(state.outs, 1); // 1 extra
                        } else {
                            state.runner1 = Some(batter);
                        }
                    }
                    RunnersOn::Runner010 => {
                        state.runners = RunnersOn::Runner100;
                        state.runner2 = None;
                        state.runner1 = Some(batter);
                    }
                    RunnersOn::Runner001 => {
                        state.runners = RunnersOn::Runner100;
                        state.runner3 = None;
                        state.runner1 = Some(batter);
                    }
                    RunnersOn::Runner110 => {}
                    RunnersOn::Runner011 => {
                        state.runners = RunnersOn::Runner101;
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                        state.runner1 = Some(batter);
                    }
                    RunnersOn::Runner101 => {
                        // NOTE: special rules for GB+ pitchers
                        if pitcher.groundball() && fielder == 2 {
                            state.game_text += "\nGB+ automatic double play.";
                            state.runners = RunnersOn::Runner001;
                            state.runner1 = None;
                            state.outs = increment_out(state.outs, 1); // 1 extra
                        }
                        state.runners = RunnersOn::Runner110;
                        state.runner3 = None;
                        state.runner1 = Some(batter);
                    }
                    RunnersOn::Runner111 => {}
                }
            }
            state.outs = increment_out(state.outs, 1);
        }
    }

    state
}

/// process non-productive out swing results
fn actual_out(mut state: GameState, mss_result: &i32) -> GameState {
    state.game_text += "\nOut!";
    // runners at second and third cannot advance on a flyball
    // TODO: check if runner at first can advance
    // anywhere in the infield, runner at first and batter are out
    let fielder = get_swing_position(mss_result);
    if (3..=6).contains(&fielder) {
        match state.outs {
            Outs::Three => {}
            Outs::Two => {
                state.outs = Outs::Three;
            }
            _ => match state.runners {
                RunnersOn::Runner100 => {
                    state.game_text += "\nDouble Play!  Runner at first and batter are out.";
                    state.runners = RunnersOn::Runner000;
                    state.runner1 = None;
                    state.outs = increment_out(state.outs, 2);
                }
                RunnersOn::Runner110 => {
                    state.game_text += "\nDouble Play!  Runner at first and batter are out.";
                    state.runners = RunnersOn::Runner010;
                    state.runner1 = None;
                    state.outs = increment_out(state.outs, 2);
                }
                RunnersOn::Runner101 => {
                    state.game_text += "\nDouble Play!  Runner at first and batter are out.";
                    state.runners = RunnersOn::Runner001;
                    state.runner1 = None;
                    state.outs = increment_out(state.outs, 2);
                }
                RunnersOn::Runner111 => {
                    state.game_text += "\nDouble Play!  Runner at first and batter are out.";
                    state.runners = RunnersOn::Runner011;
                    state.runner1 = None;
                    state.outs = increment_out(state.outs, 2);
                }
                _ => state.outs = increment_out(state.outs, 1),
            },
        }
    } else {
        state.outs = increment_out(state.outs, 1);
    }

    state
}

/// processes mega out swing results
fn mega_out(mut state: GameState) -> GameState {
    // triple play if no outs and runners on first and second
    // check for triple play, otherwise same as previous branch
    state.game_text += "\nOut!";
    match state.runners {
        RunnersOn::Runner110 => {
            state.game_text += "\nTriple play!";
            state.outs = Outs::Three;
            // TODO: only say it's a triple play if no outs
        }
        RunnersOn::Runner111 => {
            state.game_text += "\nTriple play!";
            state.outs = Outs::Three;
        }
        RunnersOn::Runner100 => {
            state.game_text += "\nDouble Play!  Runner at first and batter are out.";
            state.runners = RunnersOn::Runner000;
            state.runner1 = None;
            state.outs = increment_out(state.outs, 2);
        }
        RunnersOn::Runner101 => {
            state.game_text += "\nDouble Play!  Runner at first and batter are out.";
            state.runners = RunnersOn::Runner001;
            state.runner1 = None;
            state.outs = increment_out(state.outs, 2);
        }
        _ => state.outs = increment_out(state.outs, 2),
    }

    state
}

// TODO: check catcher's defense trait
/// takes a game state and processes steals of the indicated type
/// includes rules for S+/S-
/// (!) assumes you have checked for valid steal scenarios before calling it
pub fn process_steals(
    steal_type: StealType,
    mut state: GameState,
    mut debug: DebugConfig,
    catcher: &Player,
) -> GameState {
    let catcher_mod = catcher.defense();
    match steal_type {
        StealType::Second => {
            let mut steal_mod = catcher_mod;
            let stealer = state.runner1.clone().unwrap(); // TODO: error proof?
            if stealer.speedy() {
                steal_mod = 1;
            }
            if stealer.slow() {
                steal_mod = -2;
            }
            let steal_result = combined_roll(&mut debug, 8) + steal_mod;

            if steal_result > 3 {
                // successful steal
                match state.runners {
                    RunnersOn::Runner100 => {
                        state.runners = RunnersOn::Runner010;
                        state.runner2 = state.runner1.clone();
                        state.runner1 = None;
                    }
                    RunnersOn::Runner101 => {
                        state.runners = RunnersOn::Runner011;
                        state.runner2 = state.runner1.clone();
                        state.runner1 = None;
                    }
                    _ => {} // only valid configurations
                }
                state.game_text +=
                    &format!("\n{} {} stole 2B!", stealer.first_name, stealer.last_name);
            } else {
                // runner is out
                match state.runners {
                    RunnersOn::Runner100 => {
                        state.runners = RunnersOn::Runner000;
                        state.runner1 = None;
                    }
                    RunnersOn::Runner101 => {
                        state.runners = RunnersOn::Runner001;
                        state.runner1 = None;
                    }
                    _ => {}
                }
                state.outs = increment_out(state.outs, 1);
                state.game_text += &format!(
                    "\n{} {} thrown out stealing 2B!",
                    stealer.first_name, stealer.last_name
                );
            }
        }
        StealType::Third => {
            let mut steal_mod = catcher_mod;
            let stealer = state.runner2.clone().unwrap(); // TODO: error proof?
            if stealer.speedy() {
                steal_mod = 1;
            }
            if stealer.slow() {
                steal_mod = -2;
            }
            let steal_result = combined_roll(&mut debug, 8) - 1 + steal_mod;

            if steal_result > 3 {
                match state.runners {
                    RunnersOn::Runner010 => {
                        state.runners = RunnersOn::Runner001;
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                    }
                    RunnersOn::Runner110 => {
                        state.runners = RunnersOn::Runner101;
                        state.runner3 = state.runner2.clone();
                        state.runner2 = None;
                    }
                    _ => {}
                }
                state.game_text +=
                    &format!("\n{} {} stole 3B!", stealer.first_name, stealer.last_name);
            } else {
                match state.runners {
                    RunnersOn::Runner010 => {
                        state.runners = RunnersOn::Runner000;
                        state.runner2 = None;
                    }
                    RunnersOn::Runner110 => {
                        state.runners = RunnersOn::Runner100;
                        state.runner2 = None;
                    }
                    _ => {}
                }
                state.outs = increment_out(state.outs, 1);
                state.game_text += &format!(
                    "\n{} {} thrown out stealing 3B!",
                    stealer.first_name, stealer.last_name
                );
            }
        }
        StealType::Home => {
            // NOTE: your runner should have S+ to end up here!
            let stealer = state.runner3.clone().unwrap();
            let steal_result = combined_roll(&mut debug, 8) + 1 + catcher_mod;

            // runner leaves 3rd no matter outcome of steal attempt
            match state.runners {
                RunnersOn::Runner001 => {
                    state.runners = RunnersOn::Runner000;
                    state.runner3 = None;
                }
                RunnersOn::Runner101 => {
                    state.runners = RunnersOn::Runner100;
                    state.runner3 = None;
                }
                RunnersOn::Runner011 => {
                    state.runners = RunnersOn::Runner010;
                    state.runner3 = None;
                }
                RunnersOn::Runner111 => {
                    state.runners = RunnersOn::Runner110;
                    state.runner3 = None;
                }
                _ => {}
            }
            if steal_result >= 8 {
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
                state.game_text +=
                    &format!("\n{} {} stole home!", stealer.first_name, stealer.last_name);
            } else {
                state.outs = increment_out(state.outs, 1);
                state.game_text += &format!(
                    "\n{} {} thrown out stealing home.",
                    stealer.first_name, stealer.last_name
                );
            }
        }
        StealType::Double => {
            let mut steal_mod = catcher_mod;
            // look at traits of lead runner
            let stealer = state.runner2.clone().unwrap(); // TODO: error proof?
            let stealer2 = state.runner1.clone().unwrap();
            if stealer.speedy() {
                steal_mod = 1;
            }
            if stealer.slow() {
                steal_mod = -1; // see 2nd ed. pg. 31 - is it a typo?
            }
            let steal_result = combined_roll(&mut debug, 8) + steal_mod;

            if steal_result <= 3 {
                // lead runner is out - only valid condition is Runner110
                state.runners = RunnersOn::Runner010;
                state.runner3 = None;
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                state.outs = increment_out(state.outs, 1);
                state.game_text += &format!(
                    "\n{} {} thrown out at third",
                    stealer.first_name, stealer.last_name
                );
                state.game_text += &format!(
                    "\n{} {} steals 2B safely.",
                    stealer2.first_name, stealer2.last_name
                );
            } else if steal_result > 3 && steal_result <= 5 {
                // trailing runner is out
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
                state.runner1 = None;
                state.outs = increment_out(state.outs, 1);
                state.game_text += &format!(
                    "\n{} {} steals 3B safely.",
                    stealer.first_name, stealer.last_name
                );
                state.game_text += &format!(
                    "\n{} {} thrown out at 2B.",
                    stealer2.first_name, stealer2.last_name
                );
            } else {
                // both runners reach safely
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                state.game_text +=
                    &format!("\n{} {} stole 3B!", stealer.first_name, stealer.last_name);
                state.game_text +=
                    &format!("\n{} {} stole 2B!", stealer2.first_name, stealer2.last_name);
            }
        }
    }
    state
}

/// process bunting
pub fn bunt(
    mut state: GameState,
    game: &GameModern,
    mut debug: DebugConfig,
    batter: Player,
) -> GameState {
    // check traits, get bunt roll result
    let mut bunt_mod: i32 = 0;
    if batter.contact_hit() {
        bunt_mod = 1;
    }
    if batter.free_swing() {
        bunt_mod = -1;
    }
    let bunt_result = combined_roll(&mut debug, 6) + bunt_mod;
    state.game_text += &format!("\nBunting!  Bunt roll: {}", &bunt_result);

    // process result
    if bunt_result <= 2 {
        // lead runner out, batter safe
        state.game_text += "\nLead runner out, batter safe.";
        match state.runners {
            RunnersOn::Runner000 => state.game_text += "\nNo runners, no bunt.",
            RunnersOn::Runner100 => {
                state.outs = increment_out(state.outs, 1);
                state.runner1 = None;
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner010 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner001 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner110 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner101 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state.runner3 = None;
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner011 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
                state = add_runner(state, &1, batter);
            }
            RunnersOn::Runner111 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
            }
        }
    } else if bunt_result == 3 {
        // 1st & 2nd -> lead runner advances, batter out
        // 3rd -> lead runner out, batter safe
        match state.runners {
            RunnersOn::Runner000 => state.game_text += "\nNo runners, no bunt.", // TODO: allow bunt against shift
            RunnersOn::Runner100 => {
                state.outs = increment_out(state.outs, 1);
                state = runners_advance(state, &1);
                state.game_text += "\nLead runner advances, batter out.";
            }
            RunnersOn::Runner010 => {
                state.outs = increment_out(state.outs, 1);
                state = runners_advance(state, &1);
                state.game_text += "\nLead runner advances, batter out.";
            }
            RunnersOn::Runner001 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                state = add_runner(state, &1, batter);
                state.game_text += "\nLead runner out, batter safe.";
            }
            RunnersOn::Runner110 => {
                state.outs = increment_out(state.outs, 1);
                state = runners_advance(state, &1);
                state.game_text += "\nLead runner advances, batter out.";
            }
            RunnersOn::Runner101 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state.runner3 = None;
                state = add_runner(state, &1, batter);
                state.game_text += "\nLead runner out, batter safe.";
            }
            RunnersOn::Runner011 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
                state = add_runner(state, &1, batter);
                state.game_text += "\nLead runner out, batter safe.";
            }
            RunnersOn::Runner111 => {
                state.outs = increment_out(state.outs, 1);
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                state.game_text += "\nLead runner out, batter safe.";
            }
        }
    } else if bunt_result == 4 || bunt_result == 5 {
        // lead runner advances, batter out
        state.outs = increment_out(state.outs, 1);
        state = runners_advance(state, &1);
        state.game_text += "\nLead runner advances, batter out.";
    } else {
        // >= 6
        // S+ -> Single, DEF 3B
        // lead runner advances, batter out
        if batter.speedy() {
            state = hit_table(&5, state, game, &mut debug);
            state.game_text += "\nLead runner advances, bunter races for first!";
        } else {
            state.outs = increment_out(state.outs, 1);
            state = runners_advance(state, &1);
            state.game_text += "\nLead runner advances, batter out.";
        }
    }

    state
}

/// increment outs
pub fn increment_out(current: Outs, mut increment: u32) -> Outs {
    let mut outs = Outs::None;
    if increment > 3 {
        increment = 3;
    }
    if increment == 0 {
        increment = 1;
    }
    match current {
        Outs::None => {
            if increment == 1 {
                outs = Outs::One;
            }
            if increment == 2 {
                outs = Outs::Two;
            }
            if increment == 3 {
                outs = Outs::Three;
            }
        }
        Outs::One => {
            if increment == 1 {
                outs = Outs::Two;
            }
            if increment >= 2 {
                outs = Outs::Three;
            }
        }
        Outs::Two => {
            outs = Outs::Three;
        }
        Outs::Three => outs = Outs::Three,
    }

    outs
}

/// hit and run - should be RUnner100 otherwise can't do it
pub fn hit_and_run(
    mut state: GameState,
    game: &GameModern,
    debug: &mut DebugConfig,
    batter: Player,
) -> GameState {
    state.game_text += "\n\nThe hit and run is on!";
    // first roll a steal like normal
    let stealer = state.runner1.clone().unwrap();
    let mut steal_mod = 0;
    if stealer.speedy() {
        steal_mod = 1;
    }
    if stealer.slow() {
        steal_mod = -1;
    }
    let steal_result = combined_roll(debug, 8) + steal_mod;
    state.game_text += &format!("\nSteal result: {} -> ", steal_result);
    let steal_success: bool;
    if steal_result >= 4 {
        steal_success = true;
        state.game_text += "Success!";
    } else {
        steal_success = false;
        state.game_text += "Fail!";
    }

    // now handle hit chance
    let mut pd: i32;
    let mut pitch_mod: i32 = 0;
    let control_mod: i32;
    match state.inning_half {
        InningTB::Top => {
            pd = state.home_state.current_pitcher.pitch_die;
            if state.home_state.current_pitcher.strikeout() {
                pitch_mod = -1;
            }
            control_mod = state.home_state.current_pitcher.control();
        }
        InningTB::Bottom => {
            pd = state.away_state.current_pitcher.pitch_die;
            if state.away_state.current_pitcher.strikeout() {
                pitch_mod = -1;
            }
            control_mod = state.away_state.current_pitcher.control();
        }
    }
    // NOTE: special rules for GB+
    if state.runners == RunnersOn::Runner111 {
        pd = change_pitch_die(pd, 1);
    }
    // NOTE: handedness check
    if state.away_state.current_pitcher.handedness == batter.handedness {
        pd = change_pitch_die(pd, 1);
        // TODO: make distinction between starting pitcher and reliever
    }
    let pitch_result: i32 = if pd > 0 {
        combined_roll(debug, pd)
    } else {
        -combined_roll(debug, pd.abs())
    };
    state.game_text += &format!("\nPitch result: {}", &pitch_result);
    let mss_result = pitch_result + combined_roll(debug, 100);
    state.game_text += &format!("\nMSS: {}", &mss_result);
    let mut hit_bonus = 5;
    if batter.contact_hit() {
        hit_bonus = 10;
    }
    if batter.free_swing() {
        hit_bonus = 0;
    }
    let swing_result = at_bat(
        batter.batter_target + hit_bonus + pitch_mod,
        batter.on_base_target + control_mod + hit_bonus,
        mss_result,
        game.oddity,
    );
    state.game_text += &format!(" -> {:?}", swing_result);
    match state.inning_half {
        InningTB::Top => {
            if state.away_state.current_batter == 8 {
                state.away_state.current_batter = 0;
            } else {
                state.away_state.current_batter += 1;
            }
        }
        InningTB::Bottom => {
            if state.home_state.current_batter == 8 {
                state.home_state.current_batter = 0;
            } else {
                state.home_state.current_batter += 1;
            }
        }
    }
    let hnr: HitAndRun;
    let out_type = get_swing_position(&mss_result);
    match swing_result {
        AtBatResults::Hit => hnr = HitAndRun::Hit,
        AtBatResults::Out => {
            if out_type <= 3 || out_type >= 7 {
                hnr = HitAndRun::PopUpK;
            } else {
                hnr = HitAndRun::Groundball;
            }
        }
        AtBatResults::Walk => hnr = HitAndRun::Hit,
        AtBatResults::Oddity => hnr = HitAndRun::Hit,
        AtBatResults::MegaOut => {
            if out_type <= 3 || out_type >= 7 {
                hnr = HitAndRun::PopUpK;
            } else {
                hnr = HitAndRun::Groundball;
            }
        }
        AtBatResults::CriticalHit => hnr = HitAndRun::Hit,
        AtBatResults::PossibleError => {
            let defender: Option<Player> = match state.inning_half {
                InningTB::Top => {
                    let defender_position = position_by_number(out_type);
                    find_by_position(defender_position, &game.home_active.batting_order)
                }
                InningTB::Bottom => {
                    let defender_position = position_by_number(out_type);
                    find_by_position(defender_position, &game.away_active.batting_order)
                }
            };
            let mut defense_bonus = 0;
            if defender.is_some() {
                defense_bonus += defender.unwrap().defense();
            }
            let def_roll = combined_roll(debug, 12) + defense_bonus;
            if def_roll <= 2 {
                hnr = HitAndRun::Hit;
            } else if out_type <= 3 || out_type >= 7 {
                hnr = HitAndRun::PopUpK;
            } else {
                hnr = HitAndRun::Groundball;
            }
        }
        AtBatResults::ProductiveOut1 => {
            if out_type <= 3 || out_type >= 7 {
                hnr = HitAndRun::PopUpK;
            } else {
                hnr = HitAndRun::Groundball;
            }
        }
        AtBatResults::ProductiveOut2 => {
            if out_type <= 3 || out_type >= 7 {
                hnr = HitAndRun::PopUpK;
            } else {
                hnr = HitAndRun::Groundball;
            }
        }
    }
    state.game_text += &format!("\nHit result: {:?}", hnr);

    // clean up bases
    match hnr {
        HitAndRun::Hit => {
            if steal_success {
                // runners at 1st and 3rd
                state.game_text += "\nRunners on 1st and 3rd!";
                state.runners = RunnersOn::Runner101;
                state.runner3 = state.runner1.clone();
                state.runner1 = Some(batter);
            } else {
                // runners at 1st and 2nd
                state.game_text += "\nRunners on 1st and 2nd!";
                state.runners = RunnersOn::Runner110;
                state.runner2 = state.runner1.clone();
                state.runner1 = Some(batter);
            }
        }
        HitAndRun::PopUpK => {
            if steal_success {
                // batter out, runner stays at 1st
                state.game_text += "\nBatter out, runner stays at 1st.";
                state.outs = increment_out(state.outs, 1);
            } else {
                // double play
                state.game_text += "\nDouble play!";
                state.outs = increment_out(state.outs, 2);
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                state.runner1 = None;
            }
        }
        HitAndRun::Groundball => {
            if steal_success {
                // batter out, runner reaches 2nd
                state.game_text += "\nBatter out, runner reaches 2nd.";
                state.outs = increment_out(state.outs, 1);
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                state.runners = RunnersOn::Runner010;
            } else {
                // double play
                state.game_text += "\nDouble play!";
                state.outs = increment_out(state.outs, 2);
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                state.runner1 = None;
            }
        }
    }

    state
}

/// function to generate random animal on the field
pub fn animal(debug: &mut DebugConfig) -> Animal {
    let animal_result = combined_roll(debug, 4);
    let animal: Animal;
    if animal_result == 1 {
        animal = Animal::Bird;
    } else if animal_result == 2 {
        animal = Animal::Rodent;
    } else if animal_result == 3 {
        animal = Animal::Cat;
    } else {
        animal = Animal::Streaker;
    }

    animal
}

/// advances only runners that are "forced", used for things like walks/balks/HBB
pub fn force_advance(mut state: GameState, advance: u32) -> GameState {
    match state.runners {
        RunnersOn::Runner000 => {}
        RunnersOn::Runner100 => {
            if advance == 1 {
                state.runners = RunnersOn::Runner010;
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
            } else if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner1 = None;
            } else if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            }
        }
        RunnersOn::Runner010 => {
            if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
            } else if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            }
        }
        RunnersOn::Runner001 => {
            if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            }
        }
        RunnersOn::Runner110 => {
            if advance == 1 {
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
            } else if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner2 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            } else if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner2 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 2,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 2,
                }
            }
        }
        RunnersOn::Runner101 => {
            if advance == 1 {
                state.runners = RunnersOn::Runner011;
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
            } else if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            } else if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 2,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 2,
                }
            }
        }
        RunnersOn::Runner011 => {
            if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner2.clone();
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            } else if advance >= 3 {
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                state.runner2 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 2,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 2,
                }
            }
        }
        RunnersOn::Runner111 => {
            if advance == 1 {
                state.runners = RunnersOn::Runner011;
                state.runner3 = state.runner2.clone();
                state.runner2 = state.runner1.clone();
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 1,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 1,
                }
            } else if advance == 2 {
                state.runners = RunnersOn::Runner001;
                state.runner3 = state.runner1.clone();
                state.runner2 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 2,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 2,
                }
            } else if advance == 3 {
                state.runners = RunnersOn::Runner000;
                state.runner3 = None;
                state.runner2 = None;
                state.runner1 = None;
                match state.inning_half {
                    InningTB::Top => state.away_state.runs[(state.inning - 1) as usize] += 3,
                    InningTB::Bottom => state.home_state.runs[(state.inning - 1) as usize] += 3,
                }
            }
        }
    }

    state
}
