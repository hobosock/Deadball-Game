/* INCLUDES */

/* CONSTANTS */

use std::fs;

use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

pub const TEAM_LOCATION: &str = "src/testfiles/game/teams/";
pub const PLAYER_LOCATION: &str = "src/testfiles/game/players/";
pub const BALLPARK_LOCATION: &str = "src/testfiles/game/ballparks/";

/* ENUMS */
/* STRUCTS */
/// struct for tracking database status
pub struct DeadballDatabases {
    pub loaded: bool, // flag for loading database (reset button sets to false)
    pub status_first_names: bool,
    pub status_last_names: bool,
    pub status_logos: bool,
    pub status_mascot: bool,
    pub status_motto: bool,
    pub status_personalities: bool,
    pub status_backgrounds: bool,
    pub status_park1: bool,
    pub status_park2: bool,
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
    pub logos: Vec<String>,
    pub mascots: Vec<String>,
    pub mottos: Vec<String>,
    pub personalities: Vec<String>,
    pub backgrounds: Vec<String>,
    pub park1: Vec<String>,
    pub park2: Vec<String>,
}

impl Default for DeadballDatabases {
    fn default() -> Self {
        Self {
            loaded: false,
            status_first_names: false,
            status_last_names: false,
            status_logos: false,
            status_mascot: false,
            status_motto: false,
            status_personalities: false,
            status_backgrounds: false,
            status_park1: false,
            status_park2: false,
            first_names: vec!["First".to_string()],
            last_names: vec!["Last".to_string()],
            logos: vec!["Logo".to_string()],
            mascots: vec!["Mascot".to_string()],
            mottos: vec!["Motto".to_string()],
            personalities: vec!["Personality".to_string()],
            backgrounds: vec!["Background".to_string()],
            park1: vec!["Park1".to_string()],
            park2: vec!["Park2".to_string()],
        }
    }
}

/* FUNCTIONS */

/// basic csv read function, useful for reading name databases, etc.
pub fn load_csv(filename: &str, delimiter: &str) -> Result<Vec<String>, std::io::Error> {
    let read_result = fs::read_to_string(filename);
    match read_result {
        Ok(raw_text) => {
            // separate text into vector elements and return
            let mut result: Vec<String> = raw_text.split(delimiter).map(str::to_string).collect();
            // need to check for "empty" entries
            result.retain(|x| !x.is_empty());
            // trim whitespace, can't figure out how to do it during collection
            for i in 0..result.len() {
                result[i] = result[i].trim().to_string();
            }

            Ok(result)
        }
        Err(err) => Err(err),
    }
}

/// function to load databases when program launches
pub fn load_databases(toasts: &mut Toasts) -> DeadballDatabases {
    let mut database = DeadballDatabases {
        loaded: true, // this way databases won't be read again until manual reset
        ..Default::default()
    };
    match load_csv("src/databases/firstname.csv", "\n") {
        Ok(a) => {
            database.first_names = a;
            database.status_first_names = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load first name DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/lastname.csv", "\n") {
        Ok(a) => {
            database.last_names = a;
            database.status_last_names = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load last name DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/logo.csv", "\n") {
        Ok(a) => {
            database.logos = a;
            database.status_logos = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load logo DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/mascot.csv", "\n") {
        Ok(a) => {
            database.mascots = a;
            database.status_mascot = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load mascot DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/motto.csv", "\n") {
        Ok(a) => {
            database.mottos = a;
            database.status_motto = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load motto DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/personality.csv", "\n") {
        Ok(a) => {
            database.personalities = a;
            database.status_personalities = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load personality DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/background.csv", "\n") {
        Ok(a) => {
            database.backgrounds = a;
            database.status_backgrounds = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load background DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/park1.csv", "\n") {
        Ok(a) => {
            database.park1 = a;
            database.status_park1 = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load Park1 DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    match load_csv("src/databases/park2.csv", "\n") {
        Ok(a) => {
            database.park2 = a;
            database.status_park2 = true;
        }
        Err(e) => {
            let e_str = format!("Failed to load Park2 DB:\n{}", e);
            toasts.add(Toast {
                text: e_str.into(),
                kind: ToastKind::Info,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true)
                    .show_icon(true),
            });
        }
    }

    database
}
