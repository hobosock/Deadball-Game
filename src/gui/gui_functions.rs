/*========================================================
MODULE INCLUSIONS
========================================================*/
use eframe::egui;
use egui::{Align2, Direction, Pos2};

use crate::{
    characters::{
        players::{Player, PlayerClass, Position},
        teams::{ActiveTeam, Era},
    },
    core::game_functions::{find_by_position, RunnersOn},
    DeadballApp,
};

/*========================================================
STRUCT DEFINITIONS
========================================================*/
/// configuration for toast notifications
pub struct ToastData {
    pub alignment: Align2,
    pub offset: Pos2,
    pub direction: Direction,
}

impl Default for ToastData {
    fn default() -> Self {
        Self {
            alignment: Align2::RIGHT_BOTTOM,
            offset: Pos2::new(2.0, 2.0),
            direction: Direction::BottomUp,
        }
    }
}

/// state variables for the Create Team window
pub struct CreateTeamWindow {
    pub is_visible: bool,
    pub era: Era,
    pub name_override: bool,
    pub name: String,
    pub location_override: bool,
    pub location: String,
    pub save_location: String,
}

impl Default for CreateTeamWindow {
    fn default() -> Self {
        Self {
            is_visible: false,
            era: Era::Modern,
            name_override: false,
            name: "".to_string(),
            location_override: false,
            location: "".to_string(),
            save_location: "".to_string(),
        }
    }
}

/// state variables for the Create Player window
pub struct CreatePlayerWindow {
    pub is_visible: bool,
    pub era: Era,
    pub class: PlayerClass,
    pub position: Position,
    pub name_override: bool,
    pub first_name: String,
    pub nickname: String,
    pub last_name: String,
    pub save_location: String,
}

impl Default for CreatePlayerWindow {
    fn default() -> Self {
        Self {
            is_visible: false,
            era: Era::Modern,
            class: PlayerClass::StartingHitter,
            position: Position::Firstbase,
            name_override: false,
            first_name: "".to_string(),
            nickname: "".to_string(),
            last_name: "".to_string(),
            save_location: "".to_string(),
        }
    }
}

pub struct CreateBallparkWindow {
    pub is_visible: bool,
    pub era: Era,
    pub name_override: bool,
    pub name: String,
    pub save_location: String,
}

impl Default for CreateBallparkWindow {
    fn default() -> Self {
        Self {
            is_visible: false,
            era: Era::Modern,
            name_override: false,
            name: "".to_string(),
            save_location: "".to_string(),
        }
    }
}

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
/// produces a String with player first name + last name
/// takes Player struct as input
pub fn get_player_name(player: &Player) -> String {
    // TODO: should this include nicknames somehow?
    format!("{} {}", player.first_name, player.last_name)
}

/// updates the strings shown on the ballfield graphic in player positions
/// input a reference to an ActiveTeam struct and receive a vector of 9 strings
/// order is first, second, shortstop, third, catcher, left, center, right, pitcher
pub fn update_player_labels(team: &ActiveTeam) -> Vec<String> {
    // find positions
    let first_baseman = find_by_position(Position::Firstbase, &team.roster);
    let second_baseman = find_by_position(Position::Secondbase, &team.roster);
    let shortstop = find_by_position(Position::Shortstop, &team.roster);
    let third_baseman = find_by_position(Position::Thirdbase, &team.roster);
    let catcher = find_by_position(Position::Catcher, &team.roster);
    let leftfielder = find_by_position(Position::Leftfield, &team.roster);
    let centerfielder = find_by_position(Position::Centerfield, &team.roster);
    let rightfielder = find_by_position(Position::Rightfield, &team.roster);
    let pitcher = team.pitching[0].clone();

    // generate name strings for labels
    // TODO: do we need some kind of error handling here?
    let first_label = get_player_name(&first_baseman.unwrap());
    let second_label = get_player_name(&second_baseman.unwrap());
    let short_label = get_player_name(&shortstop.unwrap());
    let third_label = get_player_name(&third_baseman.unwrap());
    let catcher_label = get_player_name(&catcher.unwrap());
    let left_label = get_player_name(&leftfielder.unwrap());
    let center_label = get_player_name(&centerfielder.unwrap());
    let right_label = get_player_name(&rightfielder.unwrap());
    let pitcher_label = get_player_name(&pitcher);

    vec![
        first_label,
        second_label,
        short_label,
        third_label,
        catcher_label,
        left_label,
        center_label,
        right_label,
        pitcher_label,
    ]
}

/// returns 3 bools indicating if runners are on each base based on game state
pub fn runners_on_bool(runners: RunnersOn) -> (bool, bool, bool) {
    let on_first: bool;
    let on_second: bool;
    let on_third: bool;
    match runners {
        RunnersOn::Runner000 => {
            on_first = false;
            on_second = false;
            on_third = false;
        }
        RunnersOn::Runner100 => {
            on_first = true;
            on_second = false;
            on_third = false;
        }
        RunnersOn::Runner010 => {
            on_first = false;
            on_second = true;
            on_third = false;
        }
        RunnersOn::Runner001 => {
            on_first = false;
            on_second = false;
            on_third = true;
        }
        RunnersOn::Runner110 => {
            on_first = true;
            on_second = true;
            on_third = false;
        }
        RunnersOn::Runner101 => {
            on_first = true;
            on_second = false;
            on_third = true;
        }
        RunnersOn::Runner011 => {
            on_first = false;
            on_second = true;
            on_third = true;
        }
        RunnersOn::Runner111 => {
            on_first = true;
            on_second = true;
            on_third = true;
        }
    }
    (on_first, on_second, on_third)
}

/// builds string for tooltip for batters and baserunners
pub fn batter_tooltip(player: &Player) -> String {
    let tooltip = format!(
        "{} | {} | {} | {:?} | {:?}",
        player.first_name, player.nickname, player.last_name, player.handedness, player.traits
    );
    tooltip
}

/// handles updating numbers stored in DeadballApp struct from user input strings
/// this function in particular deals with debug mode related values
pub fn update_debug_textedits(app: &mut DeadballApp) {
    if let Ok(inning) = app.debug_settings.debug_inning_text.parse::<u32>() {
        app.debug_settings.debug_state.inning = inning;
    }
    if let Ok(batter) = app.debug_settings.debug_batting1_text.parse::<u32>() {
        app.debug_settings.debug_state.home_state.current_batter = batter;
    }
    if let Ok(batter) = app.debug_settings.debug_batting2_text.parse::<u32>() {
        app.debug_settings.debug_state.away_state.current_batter = batter;
    }
    if let Ok(inning) = app.debug_settings.debug_pitched1_text.parse::<u32>() {
        app.debug_settings.debug_state.home_state.innings_pitched = inning;
    }
    if let Ok(inning) = app.debug_settings.debug_pitched2_text.parse::<u32>() {
        app.debug_settings.debug_state.away_state.innings_pitched = inning;
    }
    if let Ok(runs) = app.debug_settings.debug_runs1_text.parse::<u32>() {
        app.debug_settings.debug_state.home_state.runs
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = runs;
    }
    if let Ok(runs) = app.debug_settings.debug_runs2_text.parse::<u32>() {
        app.debug_settings.debug_state.away_state.runs
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = runs;
    }
    if let Ok(hits) = app.debug_settings.debug_hits1_text.parse::<u32>() {
        app.debug_settings.debug_state.home_state.hits
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = hits;
    }
    if let Ok(hits) = app.debug_settings.debug_hits2_text.parse::<u32>() {
        app.debug_settings.debug_state.away_state.hits
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = hits;
    }
    if let Ok(errors) = app.debug_settings.debug_errors1_text.parse::<u32>() {
        app.debug_settings.debug_state.home_state.errors
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = errors;
    }
    if let Ok(errors) = app.debug_settings.debug_errors2_text.parse::<u32>() {
        app.debug_settings.debug_state.away_state.errors
            [(app.game_state.as_ref().unwrap().inning - 1) as usize] = errors;
    }
}
