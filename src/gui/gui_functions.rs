/*========================================================
MODULE INCLUSIONS
========================================================*/

use deadball::{
    characters::{
        players::{Player, Position},
        teams::ActiveTeam,
    },
    core::game_functions::{find_by_position, RunnersOn},
};

/*========================================================
FUNCTION DEFINITIONS
========================================================*/
/// produces a String with player first name + last name
/// takes Player struct as input
pub fn get_player_name(player: &Player) -> String {
    // TODO: should this include nicknames somehow?
    let name_str = format!("{} {}", player.first_name, player.last_name);
    return name_str;
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

    let labels = vec![
        first_label,
        second_label,
        short_label,
        third_label,
        catcher_label,
        left_label,
        center_label,
        right_label,
        pitcher_label,
    ];
    return labels;
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
    return (on_first, on_second, on_third);
}
