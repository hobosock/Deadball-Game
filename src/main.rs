use deadball::characters::players::*;
use deadball::characters::teams::*;
use deadball::core::*;

use std::fs;

fn main() {
    // need to load in databases for generating names, etc.
    let read_result = fs::read_to_string("src/databases/firstname.csv");

    // quick test, just print out each step of a game and see if it makes sense
    // 1. generate 2 new teams
    let team1 = generate_team(
        Era::Modern,
        8,
        4,
        1,
        5,
        "Red Team",
        &first_names,
        &last_names,
        &logos,
        &mascots,
        &mottos,
        &personalities,
        &backgrounds,
        &name1,
        &name2,
    );
}
