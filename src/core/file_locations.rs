/* INCLUDES */

/* CONSTANTS */

use std::fs;

pub const TEAM_LOCATION: &str = "src/testfiles/game/teams/";
pub const PLAYER_LOCATION: &str = "src/testfiles/game/players/";
pub const BALLPARK_LOCATION: &str = "src/testfiles/game/ballparks/";

/* ENUMS */
/* STRUCTS */
/* FUNCTIONS */
pub fn load_csv(filename: &str, delimiter: &str) -> Result<Vec<String>, std::io::Error> {
    let read_result = fs::read_to_string(filename);
    match read_result {
        Ok(raw_text) => {
            // separate text into vector elements and return
            let mut result: Vec<String> = raw_text.split(delimiter).map(str::to_string).collect();
            // need to check for "empty" entries
            result.retain(|x| x.len() >= 1);

            return Ok(result);
        }
        Err(err) => return Err(err),
    }
}
