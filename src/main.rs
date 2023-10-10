use deadball::characters::{players::*, teams::*};
use deadball::core::file_locations::*;
use deadball::core::game_functions::{
    create_modern_game, init_new_game_state, modern_game_flow, modern_inning_flow,
    new_game_state_struct, GameModern, GameState, GameStatus, InningTB, Outs, RunnersOn,
};
use gui::gui_functions::{runners_on_bool, update_player_labels};
mod gui;

use std::fs;

/*==============================================================================================
 * IMPORTS
 * ===========================================================================================*/
use eframe::{
    egui,
    epaint::{pos2, Color32},
};
use egui::{Rect, RichText};
use egui_extras::RetainedImage;
use egui_file::FileDialog;
use std::path::PathBuf;

/*==============================================================================================
 * CONSTANTS
 * ===========================================================================================*/
const ABOUT_DEABALL: &str =
    "Deadball: Baseball With Dice is a tabletop game developed by W.M. Akers.  For more information about the game, or to purchase the rules, please visit the Deadball website.";

const ABOUT_APP: &str = "This application was developed as practice with the Rust programming language.  All credit goes to the creator of Deadball, W.M. Akers.  Please purchase and consult the official rulebooks for questions about game mechanics.";

/*==============================================================================================
 * ENUMS
 * ===========================================================================================*/
#[derive(PartialEq, Eq)]
enum Panel {
    Menu,
    Game,
    Roster,
    Debug,
}

/*
impl Default for Panel {
    fn default() -> Self {
        Self::Menu
    }
}
*/

/*==============================================================================================
 * STRUCTS
 * ===========================================================================================*/
struct DeadballApp {
    // score information
    current_inning: String,
    current_outs: String,
    away_hits: String,
    away_errors: String,
    away_runs: String,
    home_hits: String,
    home_errors: String,
    home_runs: String,
    // ballfield interface
    diamond_image: RetainedImage,
    helmet_image: RetainedImage,
    pitcher_label: String,
    catcher_label: String,
    firstbase_label: String,
    secondbase_label: String,
    shortstop_label: String,
    thirdbase_label: String,
    rightfield_label: String,
    centerfield_label: String,
    leftfield_label: String,
    // batting order interface
    away_team_name: String,
    home_team_name: String,
    // menu/controls interface
    bottom_panel: Panel,
    // tracking for other windows
    version_window: bool,
    about_deadball_window: bool,
    about_app_window: bool,
    create_game_window: bool,
    debug_window: bool,
    // create game interface
    create_game_era: Era,
    away_team_file: Option<PathBuf>,
    away_team_file_dialog: Option<FileDialog>,
    home_team_file: Option<PathBuf>,
    home_team_file_dialog: Option<FileDialog>,
    ballpark_file: Option<PathBuf>,
    ballpark_file_dialog: Option<FileDialog>,
    create_game_error: String,
    // game data
    away_team: Option<Team>,
    away_team_active: Option<ActiveTeam>,
    away_batter1: Option<Player>,
    away_batter2: Option<Player>,
    away_batter3: Option<Player>,
    away_batter4: Option<Player>,
    away_batter5: Option<Player>,
    away_batter6: Option<Player>,
    away_batter7: Option<Player>,
    away_batter8: Option<Player>,
    away_batter9: Option<Player>,
    home_team: Option<Team>,
    home_team_active: Option<ActiveTeam>,
    home_batter1: String,
    home_batter2: String,
    home_batter3: String,
    home_batter4: String,
    home_batter5: String,
    home_batter6: String,
    home_batter7: String,
    home_batter8: String,
    home_batter9: String,
    ballpark_modern: Option<BallparkModern>,
    ballpark_ancient: Option<BallparkAncient>,
    game_modern: Option<GameModern>,
    game_state: Option<GameState>,
    // TODO: add ancient game
    // debug settings
    debug_copied: bool, // copy game state to debug state first time window is opened
    debug_state: GameState,
    debug_game_state_text: String,
    debug_inning_text: String,
    debug_inning_half_text: String,
    debug_outs_text: String,
    debug_runners_text: String,
    debug_batting1_text: String,
    debug_batting2_text: String,
    debug_pitched1_text: String,
    debug_pitched2_text: String,
    debug_runs1_text: String,
    debug_runs2_text: String,
    debug_hits1_text: String,
    debug_hits2_text: String,
    debug_errors1_text: String,
    debug_errors2_text: String,
}

impl<'a> Default for DeadballApp {
    fn default() -> Self {
        Self {
            current_inning: "1^".to_string(),
            current_outs: "0".to_string(),
            away_hits: "0".to_string(),
            away_errors: "0".to_string(),
            away_runs: "0".to_string(),
            home_hits: "0".to_string(),
            home_errors: "0".to_string(),
            home_runs: "0".to_string(),
            diamond_image: RetainedImage::from_image_bytes(
                "baseball_diamond.png",
                include_bytes!("images/baseball_diamond.png"),
            )
            .unwrap(),
            helmet_image: RetainedImage::from_image_bytes(
                "helmet.png",
                include_bytes!("images/helmet.png"),
            )
            .unwrap(),
            pitcher_label: "P: Seth Loveall".to_string(),
            catcher_label: "C: Seth Loveall".to_string(),
            firstbase_label: "1B: Seth Loveall".to_string(),
            secondbase_label: "2B: Seth Loveall".to_string(),
            shortstop_label: "SS: Seth Loveall".to_string(),
            thirdbase_label: "3B: Seth Loveall".to_string(),
            rightfield_label: "RF: Seth Loveall".to_string(),
            centerfield_label: "CF: Seth Loveall".to_string(),
            leftfield_label: "LF: Seth Loveall".to_string(),
            away_team_name: "Away Team".to_owned(),
            home_team_name: "Home Team".to_owned(),
            bottom_panel: Panel::Menu,
            version_window: false,
            about_deadball_window: false,
            about_app_window: false,
            create_game_window: false,
            debug_window: false,
            create_game_era: Era::None,
            away_team_file: None,
            away_team_file_dialog: None,
            home_team_file: None,
            home_team_file_dialog: None,
            ballpark_file: None,
            ballpark_file_dialog: None,
            create_game_error: "".to_owned(),
            away_team: None,
            away_team_active: None,
            away_batter1: None,
            away_batter2: None,
            away_batter3: None,
            away_batter4: None,
            away_batter5: None,
            away_batter6: None,
            away_batter7: None,
            away_batter8: None,
            away_batter9: None,
            home_team: None,
            home_team_active: None,
            home_batter1: "None".to_string(),
            home_batter2: "None".to_string(),
            home_batter3: "None".to_string(),
            home_batter4: "None".to_string(),
            home_batter5: "None".to_string(),
            home_batter6: "None".to_string(),
            home_batter7: "None".to_string(),
            home_batter8: "None".to_string(),
            home_batter9: "None".to_string(),
            ballpark_modern: None,
            ballpark_ancient: None,
            game_modern: None,
            game_state: None,
            debug_copied: false,
            debug_state: new_game_state_struct(),
            debug_game_state_text: "Not Started".to_string(),
            debug_inning_text: "1".to_string(),
            debug_inning_half_text: "^".to_string(),
            debug_outs_text: "None".to_string(),
            debug_runners_text: "000".to_string(),
            debug_batting1_text: "1".to_string(),
            debug_batting2_text: "1".to_string(),
            debug_pitched1_text: "0".to_string(),
            debug_pitched2_text: "0".to_string(),
            debug_runs1_text: "0".to_string(),
            debug_runs2_text: "0".to_string(),
            debug_hits1_text: "0".to_string(),
            debug_hits2_text: "0".to_string(),
            debug_errors1_text: "0".to_string(),
            debug_errors2_text: "0".to_string(),
        }
    }
}

impl<'a> eframe::App for DeadballApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // app state updates
        match self.debug_inning_text.parse::<u32>() {
            Ok(inning) => self.debug_state.inning = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_batting1_text.parse::<u32>() {
            Ok(inning) => self.debug_state.batting_team1 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_batting2_text.parse::<u32>() {
            Ok(inning) => self.debug_state.batting_team2 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_pitched1_text.parse::<u32>() {
            Ok(inning) => self.debug_state.pitched_team1 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_pitched2_text.parse::<u32>() {
            Ok(inning) => self.debug_state.pitched_team2 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_runs1_text.parse::<u32>() {
            Ok(inning) => self.debug_state.runs_team1 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_runs2_text.parse::<u32>() {
            Ok(inning) => self.debug_state.runs_team2 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_hits1_text.parse::<u32>() {
            Ok(inning) => self.debug_state.hits_team1 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_hits2_text.parse::<u32>() {
            Ok(inning) => self.debug_state.hits_team2 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_errors1_text.parse::<u32>() {
            Ok(inning) => self.debug_state.errors_team1 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        match self.debug_errors2_text.parse::<u32>() {
            Ok(inning) => self.debug_state.errors_team2 = inning,
            Err(_) => {} // don't do anything if user is typing, weird characters, etc.
        }
        // check if other windows are open
        egui::Window::new("Version")
            .open(&mut self.version_window)
            .show(ctx, |ui| {
                ui.label("Version 0.1");
            });
        egui::Window::new("About Deadball Game")
            .open(&mut self.about_deadball_window)
            .show(ctx, |ui| {
                ui.label(ABOUT_DEABALL);
                ui.hyperlink("http://wmakers.net/deadball");
            });
        egui::Window::new("About this app")
            .open(&mut self.about_app_window)
            .show(ctx, |ui| {
                ui.label(ABOUT_APP);
            });
        egui::Window::new("Debug Mode")
            .open(&mut self.debug_window)
            .show(ctx, |ui| {
                // set debug state to current game state (if it exists)
                if self.game_state.is_some() && self.debug_copied == false {
                    self.debug_state = self.game_state.clone().unwrap();
                    self.debug_copied = true;
                    self.debug_inning_text = self.debug_state.inning.clone().to_string();
                }
                ui.horizontal(|ui| {
                    ui.label("Game Status:");
                    egui::ComboBox::from_label("Select status.")
                        .selected_text(&self.debug_game_state_text)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.debug_state.status,
                                GameStatus::NotStarted,
                                "Not Started",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.status,
                                GameStatus::Ongoing,
                                "Ongoing",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.status,
                                GameStatus::Over,
                                "Over",
                            );
                        })
                });
                ui.horizontal(|ui| {
                    ui.label("Inning:");
                    ui.text_edit_singleline(&mut self.debug_inning_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Inning Half:");
                    egui::ComboBox::from_label("Select inning half.")
                        .selected_text(self.debug_inning_half_text.clone())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.debug_state.inning_half,
                                InningTB::Top,
                                "^",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.inning_half,
                                InningTB::Bottom,
                                "v",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Outs:");
                    egui::ComboBox::from_label("Select outs.")
                        .selected_text(self.debug_outs_text.clone())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.debug_state.outs, Outs::None, "None");
                            ui.selectable_value(&mut self.debug_state.outs, Outs::One, "One");
                            ui.selectable_value(&mut self.debug_state.outs, Outs::Two, "Two");
                            ui.selectable_value(&mut self.debug_state.outs, Outs::Three, "Three");
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Runners On:");
                    egui::ComboBox::from_label("Select base runners.")
                        .selected_text(self.debug_runners_text.clone())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner000,
                                "000",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner001,
                                "001",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner010,
                                "010",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner100,
                                "100",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner011,
                                "011",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner110,
                                "110",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner101,
                                "101",
                            );
                            ui.selectable_value(
                                &mut self.debug_state.runners,
                                RunnersOn::Runner111,
                                "111",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Batting Team 1:");
                    ui.text_edit_singleline(&mut self.debug_batting1_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Batting Team 2:");
                    ui.text_edit_singleline(&mut self.debug_batting2_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Pitched Team 1:");
                    ui.text_edit_singleline(&mut self.debug_pitched1_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Pitched Team 2:");
                    ui.text_edit_singleline(&mut self.debug_pitched2_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Runs Team 1:");
                    ui.text_edit_singleline(&mut self.debug_runs1_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Runs Team 2:");
                    ui.text_edit_singleline(&mut self.debug_runs2_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Hits Team 1:");
                    ui.text_edit_singleline(&mut self.debug_hits1_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Hits Team 2:");
                    ui.text_edit_singleline(&mut self.debug_hits2_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Errors Team 1:");
                    ui.text_edit_singleline(&mut self.debug_errors1_text);
                });
                ui.horizontal(|ui| {
                    ui.label("Errors Team 2:");
                    ui.text_edit_singleline(&mut self.debug_errors2_text);
                });
                // update debug game state combo box text
                match &self.debug_state.status {
                    GameStatus::NotStarted => {
                        self.debug_game_state_text = "Not Started".to_string()
                    }
                    GameStatus::Ongoing => self.debug_game_state_text = "Ongoing".to_string(),
                    GameStatus::Over => self.debug_game_state_text = "Over".to_string(),
                }
                // update inning half combo box text
                match &self.debug_state.inning_half {
                    InningTB::Top => self.debug_inning_half_text = "^".to_string(),
                    InningTB::Bottom => self.debug_inning_half_text = "v".to_string(),
                }
                // update outs combo box text
                match &self.debug_state.outs {
                    Outs::None => self.debug_outs_text = "None".to_string(),
                    Outs::One => self.debug_outs_text = "One".to_string(),
                    Outs::Two => self.debug_outs_text = "Two".to_string(),
                    Outs::Three => self.debug_outs_text = "Three".to_string(),
                }
                // update runners on text
                match &self.debug_state.runners {
                    RunnersOn::Runner000 => self.debug_runners_text = "000".to_string(),
                    RunnersOn::Runner001 => self.debug_runners_text = "001".to_string(),
                    RunnersOn::Runner010 => self.debug_runners_text = "010".to_string(),
                    RunnersOn::Runner100 => self.debug_runners_text = "100".to_string(),
                    RunnersOn::Runner011 => self.debug_runners_text = "011".to_string(),
                    RunnersOn::Runner110 => self.debug_runners_text = "110".to_string(),
                    RunnersOn::Runner101 => self.debug_runners_text = "101".to_string(),
                    RunnersOn::Runner111 => self.debug_runners_text = "111".to_string(),
                }
                // button to write changes to game state
                ui.separator();
                if ui.button("Write Changes").clicked() {
                    self.game_state = Some(self.debug_state.clone());
                }
            });
        egui::Window::new("Create new game")
            .open(&mut self.create_game_window)
            .show(ctx, |ui| {
                // selectable value for game era
                ui.horizontal(|ui| {
                    ui.label("Era:");
                    ui.selectable_value(&mut self.create_game_era, Era::None, "None");
                    ui.selectable_value(&mut self.create_game_era, Era::Modern, "Modern");
                    ui.selectable_value(&mut self.create_game_era, Era::Ancient, "Ancient");
                });
                // file dialog for away team
                ui.horizontal(|ui| {
                    ui.label("Away Team:");
                    if let Some(away_file) = &mut self.away_team_file {
                        ui.label(format!("{:?}", away_file));
                    } else {
                        ui.label("None");
                    }
                    if ui.button("Open").clicked() {
                        let mut dialog = FileDialog::open_file(self.away_team_file.clone());
                        dialog.open();
                        self.away_team_file_dialog = Some(dialog);
                    }
                    if let Some(dialog) = &mut self.away_team_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.away_team_file = Some(file);
                            }
                        }
                    }
                });
                // file dialog for home team
                ui.horizontal(|ui| {
                    ui.label("Home Team:");
                    if let Some(home_file) = &mut self.home_team_file {
                        ui.label(format!("{:?}", home_file));
                    } else {
                        ui.label("None");
                    }
                    if ui.button("Open").clicked() {
                        let mut dialog = FileDialog::open_file(self.home_team_file.clone());
                        dialog.open();
                        self.home_team_file_dialog = Some(dialog);
                    }
                    if let Some(dialog) = &mut self.home_team_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.home_team_file = Some(file);
                            }
                        }
                    }
                });
                // file dialog for ball park
                ui.horizontal(|ui| {
                    ui.label("Ballpark: ");
                    if let Some(ballpark_file) = &mut self.ballpark_file {
                        ui.label(format!("{:?}", ballpark_file));
                    } else {
                        ui.label("None");
                    }
                    if ui.button("Open").clicked() {
                        let mut dialog = FileDialog::open_file(self.ballpark_file.clone());
                        dialog.open();
                        self.ballpark_file_dialog = Some(dialog);
                    }
                    if let Some(dialog) = &mut self.ballpark_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.ballpark_file = Some(file);
                            }
                        }
                    }
                });
                ui.separator();
                // button to create game and return to main screen
                self.create_game_error = "".to_owned();
                if ui.button("Create").clicked() {
                    // check and make sure options are set properly
                    if self.away_team_file.is_some()
                        && self.home_team_file.is_some()
                        && self.ballpark_file.is_some()
                    {
                        // try to load teams and ballpark files
                        match fs::read_to_string(&self.away_team_file.as_ref().unwrap().as_path()) {
                            Ok(contents) => {
                                self.away_team = Some(load_team(contents));
                            }
                            Err(err) => {
                                self.create_game_error = self.create_game_error.clone()
                                    + "Failed to read Away team file."
                                    + &format!("{:?}", err);
                            }
                        }
                        match fs::read_to_string(&self.home_team_file.as_ref().unwrap().as_path()) {
                            Ok(contents) => {
                                self.home_team = Some(load_team(contents));
                            }
                            Err(err) => {
                                self.create_game_error = self.create_game_error.clone()
                                    + "Failed to read Home team file."
                                    + &format!("{:?}", err);
                            }
                        }
                        match self.create_game_era {
                            Era::Modern => {
                                match fs::read_to_string(
                                    &self.ballpark_file.as_ref().unwrap().as_path(),
                                ) {
                                    Ok(contents) => {
                                        self.ballpark_modern = Some(load_park_modern(contents));
                                    }
                                    Err(err) => {
                                        self.create_game_error = self.create_game_error.clone()
                                            + "Failed to read Ballpark file."
                                            + &format!("{:?}", err);
                                    }
                                }
                            }
                            Era::Ancient => {
                                match fs::read_to_string(
                                    &self.ballpark_file.as_ref().unwrap().as_path(),
                                ) {
                                    Ok(contents) => {
                                        self.ballpark_ancient = Some(load_park_ancient(contents));
                                    }
                                    Err(err) => {
                                        self.create_game_error = self.create_game_error.clone()
                                            + "Failed to read Ballpark file."
                                            + &format!("{:?}", err);
                                    }
                                }
                            }
                            Era::None => {
                                self.create_game_error =
                                    self.create_game_error.clone() + "Please select an Era.";
                            }
                        }
                    } else {
                        // update error message and display error window
                        if self.away_team_file.is_none() {
                            self.create_game_error = self.create_game_error.clone()
                                + "Must select a *.dbt file for away team.\n";
                        }
                        if self.home_team_file.is_none() {
                            self.create_game_error = self.create_game_error.clone()
                                + "Must select a *.dbt file for home team.\n";
                        }
                        if self.ballpark_file.is_none() {
                            self.create_game_error = self.create_game_error.clone()
                                + "Must select a *.dbb file for ballpark.\n";
                        }
                    }
                    match self.create_game_era {
                        Era::Modern => {
                            if self.away_team.is_some()
                                && self.home_team.is_some()
                                && self.ballpark_modern.is_some()
                            {
                                match create_modern_game(
                                    self.home_team.clone().unwrap(),
                                    self.away_team.clone().unwrap(),
                                    self.ballpark_modern.clone().unwrap(),
                                ) {
                                    Ok(game) => {
                                        self.game_modern = Some(game);
                                        self.home_team_active = Some(
                                            self.game_modern.clone().unwrap().home_active.clone(),
                                        );
                                        self.away_team_active = Some(
                                            self.game_modern.clone().unwrap().away_active.clone(),
                                        );
                                        //TODO: make the window close after successfully generating a game
                                    }
                                    Err(err) => {
                                        self.create_game_error =
                                            self.create_game_error.clone() + &format!("{:?}", err)
                                    }
                                }
                            }
                        }
                        Era::Ancient => {
                            if self.away_team.is_some()
                                && self.home_team.is_some()
                                && self.ballpark_ancient.is_some()
                            {}
                        }
                        Era::None => {
                            self.create_game_error =
                                self.create_game_error.clone() + "Please select an Era.";
                        }
                    }
                }
                // if everything loaded okay, generate game
                ui.add(eframe::egui::Label::new(
                    RichText::new(&self.create_game_error).color(Color32::RED),
                ));
            });

        // main window
        egui::TopBottomPanel::bottom("Control Panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.bottom_panel, Panel::Game, "Game");
                ui.selectable_value(&mut self.bottom_panel, Panel::Menu, "Menu");
                ui.selectable_value(&mut self.bottom_panel, Panel::Roster, "Roster");
                ui.selectable_value(&mut self.bottom_panel, Panel::Debug, "Debug");
            });
            ui.separator();
            match self.bottom_panel {
                Panel::Menu => {
                    ui.horizontal(|ui| {
                        ui.menu_button("Game", |ui| {
                            if ui.button("Create Game").clicked() {
                                self.create_game_window = true;
                                ui.close_menu();
                            }
                            if ui.button("Start Game").clicked() {
                                // check if there are active teams loaded
                                if self.home_team_active.is_some()
                                    && self.away_team_active.is_some()
                                {
                                    self.game_state = Some(init_new_game_state(
                                        self.home_team_active.clone().unwrap().pitching[0].clone(),
                                        self.away_team_active.clone().unwrap().pitching[0].clone(),
                                    ));
                                } else {
                                    println!("Load teams first.");
                                }
                            }
                            if ui.button("Load Game").clicked() {
                                // TODO: add load game feature (need to add save game feature first)
                                println!("Load game feature has not yet been added.");
                                ui.close_menu();
                            }
                            if ui.button("Save Game").clicked() {
                                // TODO: add save game feature
                                println!("Save game feature has not been added yet.");
                            }
                        });
                        ui.menu_button("About", |ui| {
                            if ui.button("Version").clicked() {
                                self.version_window = true;
                                ui.close_menu();
                            }
                            if ui.button("Help").clicked() {
                                // TODO: add help window
                                println!("No help menu available at this time.");
                                ui.close_menu();
                            }
                            if ui.button("About Deadball").clicked() {
                                self.about_deadball_window = true;
                                ui.close_menu();
                            }
                            if ui.button("About This App").clicked() {
                                self.about_app_window = true;
                                ui.close_menu();
                            }
                        })
                    });
                }
                Panel::Game => {
                    if ui.button("Next At Bat").clicked() {
                        // TODO: this could be cleaner
                        if self.game_state.is_some() && self.game_modern.is_some() {
                            // TODO: update with ancient game when ready
                            match self.game_state.clone().unwrap().status {
                                GameStatus::NotStarted => {
                                    self.game_state.as_mut().unwrap().status = GameStatus::Ongoing
                                }
                                GameStatus::Ongoing => {
                                    self.game_state = Some(modern_game_flow(
                                        &self.game_modern.clone().unwrap(),
                                        self.game_state.clone().unwrap(),
                                    ));
                                    println!("{:?}", self.game_state);
                                }
                                GameStatus::Over => {}
                            }
                        }
                        if self.game_state.is_some() && self.game_modern.is_some() {
                            // TODO: update with ancient game when ready
                            self.game_state = Some(modern_game_flow(
                                &self.game_modern.clone().unwrap(),
                                self.game_state.clone().unwrap(),
                            ));
                            println!("{:?}", self.game_state);
                        } else {
                            println!("Need to initialize a game first.");
                        }
                    }
                }
                Panel::Roster => {
                    ui.horizontal(|ui| {
                        if ui.button("Batting Order").clicked() {
                            println!("Batting Order button placeholder.");
                        }
                        if ui.button("Bullpen").clicked() {
                            println!("Bullpen button placeholder.");
                        }
                        if ui.button("View Team").clicked() {
                            println!("View Team button placeholder.");
                        }
                    });
                }
                Panel::Debug => {
                    ui.horizontal(|ui| {
                        if ui.button("Game").clicked() {
                            self.debug_window = true;
                            self.debug_copied = false;
                        }
                    });
                }
            }
        });
        egui::SidePanel::left("Away Team").show(ctx, |ui| {
            ui.heading(&self.away_team_name);
            let away_name1: String;
            let away_name2: String;
            let away_name3: String;
            let away_name4: String;
            let away_name5: String;
            let away_name6: String;
            let away_name7: String;
            let away_name8: String;
            let away_name9: String;
            if self.away_team.is_some() {
                let away_team = self.away_team.as_ref().unwrap();
                let away_team_active = self.game_modern.clone().unwrap().away_active;
                self.away_team_name = away_team.name.to_string();
                self.away_batter1 = Some(away_team_active.roster[0].clone());
                self.away_batter2 = Some(away_team_active.roster[1].clone());
                self.away_batter3 = Some(away_team_active.roster[2].clone());
                self.away_batter4 = Some(away_team_active.roster[3].clone());
                self.away_batter5 = Some(away_team_active.roster[4].clone());
                self.away_batter6 = Some(away_team_active.roster[5].clone());
                self.away_batter7 = Some(away_team_active.roster[6].clone());
                self.away_batter8 = Some(away_team_active.roster[7].clone());
                self.away_batter9 = Some(away_team_active.pitching[0].clone());
                away_name1 = format!(
                    "{} {}",
                    self.away_batter1.clone().unwrap().first_name,
                    self.away_batter1.clone().unwrap().last_name
                );
                away_name2 = format!(
                    "{} {}",
                    self.away_batter2.clone().unwrap().first_name,
                    self.away_batter2.clone().unwrap().last_name
                );
                away_name3 = format!(
                    "{} {}",
                    self.away_batter3.clone().unwrap().first_name,
                    self.away_batter3.clone().unwrap().last_name
                );
                away_name4 = format!(
                    "{} {}",
                    self.away_batter4.clone().unwrap().first_name,
                    self.away_batter4.clone().unwrap().last_name
                );
                away_name5 = format!(
                    "{} {}",
                    self.away_batter5.clone().unwrap().first_name,
                    self.away_batter5.clone().unwrap().last_name
                );
                away_name6 = format!(
                    "{} {}",
                    self.away_batter6.clone().unwrap().first_name,
                    self.away_batter6.clone().unwrap().last_name
                );
                away_name7 = format!(
                    "{} {}",
                    self.away_batter7.clone().unwrap().first_name,
                    self.away_batter7.clone().unwrap().last_name
                );
                away_name8 = format!(
                    "{} {}",
                    self.away_batter8.clone().unwrap().first_name,
                    self.away_batter8.clone().unwrap().last_name
                );
                away_name9 = format!(
                    "{} {}",
                    self.away_batter9.clone().unwrap().first_name,
                    self.away_batter9.clone().unwrap().last_name
                );
            } else {
                away_name1 = "None".to_string();
                away_name2 = "None".to_string();
                away_name3 = "None".to_string();
                away_name4 = "None".to_string();
                away_name5 = "None".to_string();
                away_name6 = "None".to_string();
                away_name7 = "None".to_string();
                away_name8 = "None".to_string();
                away_name9 = "None".to_string();
            }
            ui.horizontal(|ui| {
                ui.label("1. ");
                ui.label(away_name1);
                // TODO: figure out a way to put baseball icon to indicate current batter
            });
            ui.horizontal(|ui| {
                ui.label("2. ");
                ui.label(away_name2);
            });
            ui.horizontal(|ui| {
                ui.label("3. ");
                ui.label(away_name3);
            });
            ui.horizontal(|ui| {
                ui.label("4. ");
                ui.label(away_name4);
            });
            ui.horizontal(|ui| {
                ui.label("5. ");
                ui.label(away_name5);
            });
            ui.horizontal(|ui| {
                ui.label("6. ");
                ui.label(away_name6);
            });
            ui.horizontal(|ui| {
                ui.label("7. ");
                ui.label(away_name7);
            });
            ui.horizontal(|ui| {
                ui.label("8. ");
                ui.label(away_name8);
            });
            ui.horizontal(|ui| {
                ui.label("9. ");
                ui.label(away_name9);
            });
        });
        egui::SidePanel::right("Home Team").show(ctx, |ui| {
            ui.heading(&self.home_team_name);
            if self.home_team.is_some() {
                let home_team = self.home_team.as_ref().unwrap();
                self.home_team_name = home_team.name.to_string();
                self.home_batter1 = format!(
                    "{} {}",
                    self.game_modern.clone().unwrap().home_active.roster[0].first_name,
                    self.game_modern.clone().unwrap().home_active.roster[0].last_name
                );
                let batter2 = &self.game_modern.clone().unwrap().home_active.roster[1];
                self.home_batter2 = format!("{} {}", &batter2.first_name, &batter2.last_name);
                let batter3 = &self.game_modern.clone().unwrap().home_active.roster[2];
                self.home_batter3 = format!("{} {}", &batter3.first_name, &batter3.last_name);
                let batter4 = &self.game_modern.clone().unwrap().home_active.roster[3];
                self.home_batter4 = format!("{} {}", &batter4.first_name, &batter4.last_name);
                let batter5 = &self.game_modern.clone().unwrap().home_active.roster[4];
                self.home_batter5 = format!("{} {}", &batter5.first_name, &batter5.last_name);
                let batter6 = &self.game_modern.clone().unwrap().home_active.roster[5];
                self.home_batter6 = format!("{} {}", &batter6.first_name, &batter6.last_name);
                let batter7 = &self.game_modern.clone().unwrap().home_active.roster[6];
                self.home_batter7 = format!("{} {}", &batter7.first_name, &batter7.last_name);
                let batter8 = &self.game_modern.clone().unwrap().home_active.roster[7];
                self.home_batter8 = format!("{} {}", &batter8.first_name, &batter8.last_name);
                let batter9 = &self.game_modern.clone().unwrap().home_active.pitching[0];
                self.home_batter9 = format!("{} {}", &batter9.first_name, &batter9.last_name);
            }
            ui.horizontal(|ui| {
                ui.label("1. ");
                ui.label(&self.home_batter1);
            });
            ui.horizontal(|ui| {
                ui.label("2. ");
                ui.label(&self.home_batter2);
            });
            ui.horizontal(|ui| {
                ui.label("3. ");
                ui.label(&self.home_batter3);
            });
            ui.horizontal(|ui| {
                ui.label("4. ");
                ui.label(&self.home_batter4);
            });
            ui.horizontal(|ui| {
                ui.label("5. ");
                ui.label(&self.home_batter5);
            });
            ui.horizontal(|ui| {
                ui.label("6. ");
                ui.label(&self.home_batter6);
            });
            ui.horizontal(|ui| {
                ui.label("7. ");
                ui.label(&self.home_batter7);
            });
            ui.horizontal(|ui| {
                ui.label("8. ");
                ui.label(&self.home_batter8);
            });
            ui.horizontal(|ui| {
                ui.label("9. ");
                ui.label(&self.home_batter9);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // update GUI scoreboard values (if game is in progress)
            let mut on_first = false;
            let mut on_second = false;
            let mut on_third = false;
            if self.game_state.is_some() {
                let inning_number = self.game_state.as_ref().unwrap().inning.to_string();
                let inning_top_bottom: &str;
                match self.game_state.as_ref().unwrap().inning_half {
                    InningTB::Top => inning_top_bottom = "^",
                    InningTB::Bottom => inning_top_bottom = "v",
                }
                self.current_inning = inning_number + inning_top_bottom;
                self.away_hits = self.game_state.as_ref().unwrap().hits_team2.to_string();
                self.away_errors = self.game_state.as_ref().unwrap().errors_team2.to_string();
                self.away_runs = self.game_state.as_ref().unwrap().runs_team2.to_string();
                let out_string: String;
                match self.game_state.as_ref().unwrap().outs {
                    Outs::None => out_string = "0".to_string(),
                    Outs::One => out_string = "1".to_string(),
                    Outs::Two => out_string = "2".to_string(),
                    Outs::Three => out_string = "3".to_string(),
                }
                self.current_outs = out_string;
                self.home_hits = self.game_state.as_ref().unwrap().hits_team1.to_string();
                self.home_errors = self.game_state.as_ref().unwrap().errors_team1.to_string();
                self.home_runs = self.game_state.as_ref().unwrap().runs_team1.to_string();
                (on_first, on_second, on_third) =
                    runners_on_bool(self.game_state.clone().unwrap().runners);
            }
            // score line
            ui.horizontal(|ui| {
                ui.label("Inning:");
                ui.label(&self.current_inning);
                ui.label("AWAY");
                ui.label("hits:");
                ui.label(&self.away_hits);
                ui.label("errors:");
                ui.label(&self.away_errors);
                ui.label("runs:");
                ui.label(&self.away_runs);
            });
            ui.horizontal(|ui| {
                ui.label("Outs:");
                ui.label(&self.current_outs);
                ui.label("HOME");
                ui.label("hits:");
                ui.label(&self.home_hits);
                ui.label("errors:");
                ui.label(&self.home_errors);
                ui.label("runs:");
                ui.label(&self.home_runs);
            });
            // draw baseball field and label players
            ui.add(egui::Image::new(
                self.diamond_image.texture_id(ctx),
                self.diamond_image.size_vec2() * 0.3,
            ));
            // draw helmets to indicate runners on base
            if on_first {
                ui.put(
                    Rect {
                        min: pos2(490.0, 260.0),
                        max: pos2(590.0, 360.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                );
            }
            if on_second {
                ui.put(
                    Rect {
                        min: pos2(340.0, 120.0),
                        max: pos2(440.0, 220.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                );
            }
            if on_third {
                ui.put(
                    Rect {
                        min: pos2(205.0, 270.0),
                        max: pos2(305.0, 370.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                );
            }
            // update player labels
            if self.home_team_active.is_some()
                && self.away_team_active.is_some()
                && self.game_state.is_some()
            {
                let labels: Vec<String>;
                match self.game_state.as_ref().unwrap().inning_half {
                    InningTB::Top => {
                        labels = update_player_labels(&self.home_team_active.as_ref().unwrap());
                    }
                    InningTB::Bottom => {
                        labels = update_player_labels(&self.away_team_active.as_ref().unwrap());
                    }
                }
                self.firstbase_label = labels[0].clone();
                self.secondbase_label = labels[1].clone();
                self.shortstop_label = labels[2].clone();
                self.thirdbase_label = labels[3].clone();
                self.catcher_label = labels[4].clone();
                self.leftfield_label = labels[5].clone();
                self.centerfield_label = labels[6].clone();
                self.rightfield_label = labels[7].clone();
                self.pitcher_label = labels[8].clone();
            }
            // put player names
            ui.put(
                Rect {
                    min: pos2(460.0, 260.0),
                    max: pos2(560.0, 280.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.firstbase_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(400.0, 180.0),
                    max: pos2(500.0, 200.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.secondbase_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(340.0, 305.0),
                    max: pos2(440.0, 325.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.pitcher_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(340.0, 475.0),
                    max: pos2(440.0, 495.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.catcher_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(200.0, 270.0),
                    max: pos2(300.0, 290.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.thirdbase_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(240.0, 200.0),
                    max: pos2(340.0, 220.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.shortstop_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(510.0, 100.0),
                    max: pos2(610.0, 120.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.rightfield_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(330.0, 100.0),
                    max: pos2(430.0, 120.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.centerfield_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(160.0, 100.0),
                    max: pos2(260.0, 120.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.leftfield_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Deadball",
        options,
        Box::new(|_cc| Box::<DeadballApp>::default()),
    )
    /* SEGMENT OF CODE TO GENERATE TEAMS, DON'T NEED IT EVERY TIME
    // need to load in databases for generating names, etc.
    let mut first_names: Vec<String> = vec![];
    let read_result = load_csv("src/databases/firstname.csv", "\n");
    match read_result {
        Ok(mut a) => first_names.append(&mut a),
        Err(_) => {}
    }

    let mut last_names: Vec<String> = vec![];
    let read_result = load_csv("src/databases/lastname.csv", "\n");
    match read_result {
        Ok(mut a) => last_names.append(&mut a),
        Err(_) => {}
    }

    let mut logos: Vec<String> = vec![];
    let read_result = load_csv("src/databases/logo.csv", "\n");
    match read_result {
        Ok(mut a) => logos.append(&mut a),
        Err(_) => {}
    }

    let mut mascots: Vec<String> = vec![];
    let read_result = load_csv("src/databases/mascot.csv", "\n");
    match read_result {
        Ok(mut a) => mascots.append(&mut a),
        Err(_) => {}
    }

    let mut mottos: Vec<String> = vec![];
    let read_result = load_csv("src/databases/motto.csv", "\n");
    match read_result {
        Ok(mut a) => mottos.append(&mut a),
        Err(_) => {}
    }

    let mut personalities: Vec<String> = vec![];
    let read_result = load_csv("src/databases/personality.csv", "\n");
    match read_result {
        Ok(mut a) => personalities.append(&mut a),
        Err(_) => {}
    }

    let mut backgrounds: Vec<String> = vec![];
    let read_result = load_csv("src/databases/background.csv", "\n");
    match read_result {
        Ok(mut a) => backgrounds.append(&mut a),
        Err(_) => {}
    }

    let mut name1: Vec<String> = vec![];
    let read_result = load_csv("src/databases/park1.csv", "\n");
    match read_result {
        Ok(mut a) => name1.append(&mut a),
        Err(_) => {}
    }

    let mut name2: Vec<String> = vec![];
    let read_result = load_csv("src/databases/park2.csv", "\n");
    match read_result {
        Ok(mut a) => name2.append(&mut a),
        Err(_) => {}
    }

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
    let _ = write_team(team1, "src/testfiles/game/teams/red_team.dbt");

    let team2 = generate_team(
        Era::Modern,
        8,
        4,
        1,
        5,
        "Blue Team",
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
    let _ = write_team(team2, "src/testfiles/game/teams/blue_team.dbt");
    */

    /*
    let contents1 = fs::read_to_string("src/testfiles/game/teams/red_team.dbt").unwrap();
    let contents2 = fs::read_to_string("src/testfiles/game/teams/blue_team.dbt").unwrap();
    let team1 = load_team(contents1);
    let team2 = load_team(contents2);
    let (mut roster1, mut bench1, mut pitcher1, mut bullpen1) = load_roster(&team1);
    let (mut roster2, mut bench2, mut pitcher2, mut bullpen2) = load_roster(&team2);
    let contents3 = fs::read_to_string(&team1.ballpark).unwrap();
    let ballpark = load_park_modern(contents3);

    let game = create_modern_game(&team1, &team2, &ballpark).unwrap();
    let mut game_state = init_new_game_state(&pitcher1[0], &pitcher2[0]);
    modern_game_flow(&game, game_state);
    */
}
