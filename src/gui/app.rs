/*==============================================================================================
 * IMPORTS
 * ===========================================================================================*/
// LOCAL IMPORTS
use crate::characters::{players::*, teams::*};
//use deadball::core::file_locations::*;
use crate::core::game_functions::{
    bunt, create_modern_game, hit_and_run, init_new_game_state, modern_game_flow,
    new_game_state_struct, process_steals, GameModern, GameState, GameStatus, InningTB, Outs,
    RunnersOn, StealType,
};
use crate::{
    gui::debug::DebugConfig,
    gui::gui_functions::{runners_on_bool, update_player_labels},
};

use std::{fs, usize};

// EXTERNAL IMPORTS
use eframe::{
    egui::{self, Context},
    epaint::{pos2, Color32},
};
use egui::{Rect, RichText};
use egui_extras::RetainedImage;
use egui_file::FileDialog;
use std::path::PathBuf;
/*==============================================================================================
 * CONSTANTS
 * ===========================================================================================*/
pub const ABOUT_DEABALL: &str =
    "Deadball: Baseball With Dice is a tabletop game developed by W.M. Akers.  For more information about the game, or to purchase the rules, please visit the Deadball website.";

pub const ABOUT_APP: &str = "This application was developed as practice with the Rust programming language.  All credit goes to the creator of Deadball, W.M. Akers.  Please purchase and consult the official rulebooks for questions about game mechanics.";

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

/*==============================================================================================
 * STRUCTS
 * ===========================================================================================*/
pub struct DeadballApp {
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
    debug_roll_window: bool,
    console_window: bool,
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
    debug_roll_state: DebugConfig,
    debug_roll_text: String,
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
            debug_roll_window: false,
            console_window: false,
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
            debug_roll_state: DebugConfig::default(),
            debug_roll_text: "0".to_string(),
        }
    }
}

impl<'a> eframe::App for DeadballApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // app state updates
        update_debug_textedits(self);
        // draw other windows (if needed)
        draw_version_window(ctx, self);
        draw_about_deadball_window(ctx, self);
        draw_about_app_window(ctx, self);
        draw_debug_window(ctx, self);
        draw_create_new_game(ctx, self);
        draw_debug_roll_window(ctx, self);
        draw_console_window(ctx, self);

        // main window
        draw_bottom_panel(ctx, self);
        draw_left_panel(ctx, self);
        draw_right_panel(ctx, self);
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
                // no error, game state should already exist
                let runner1 = self.game_state.as_ref().unwrap().runner1.clone().unwrap();
                let runner1_text = format!(
                    "{} {} {} | {:?}",
                    runner1.first_name, runner1.nickname, runner1.last_name, runner1.traits
                );
                ui.put(
                    Rect {
                        min: pos2(490.0, 260.0),
                        max: pos2(590.0, 360.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                )
                .on_hover_text(runner1_text);
            }
            if on_second {
                let runner2 = self.game_state.as_ref().unwrap().runner2.clone().unwrap();
                let runner2_text = format!(
                    "{} {} {} | {:?}",
                    runner2.first_name, runner2.nickname, runner2.last_name, runner2.traits
                );
                ui.put(
                    Rect {
                        min: pos2(340.0, 120.0),
                        max: pos2(440.0, 220.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                )
                .on_hover_text(runner2_text);
            }
            if on_third {
                let runner3 = self.game_state.as_ref().unwrap().runner3.clone().unwrap();
                let runner3_text = format!(
                    "{} {} {} | {:?}",
                    runner3.first_name, runner3.nickname, runner3.last_name, runner3.traits
                );
                ui.put(
                    Rect {
                        min: pos2(205.0, 270.0),
                        max: pos2(305.0, 370.0),
                    },
                    eframe::egui::Image::new(
                        self.helmet_image.texture_id(ctx),
                        self.helmet_image.size_vec2() * 0.1,
                    ),
                )
                .on_hover_text(runner3_text);
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

/// handles updating numbers stored in DeadballApp struct from user input strings
/// this function in particular deals with debug mode related values
fn update_debug_textedits(app: &mut DeadballApp) {
    match app.debug_inning_text.parse::<u32>() {
        Ok(inning) => app.debug_state.inning = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_batting1_text.parse::<u32>() {
        Ok(inning) => app.debug_state.batting_team1 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_batting2_text.parse::<u32>() {
        Ok(inning) => app.debug_state.batting_team2 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_pitched1_text.parse::<u32>() {
        Ok(inning) => app.debug_state.pitched_team1 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_pitched2_text.parse::<u32>() {
        Ok(inning) => app.debug_state.pitched_team2 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_runs1_text.parse::<u32>() {
        Ok(inning) => app.debug_state.runs_team1 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_runs2_text.parse::<u32>() {
        Ok(inning) => app.debug_state.runs_team2 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_hits1_text.parse::<u32>() {
        Ok(inning) => app.debug_state.hits_team1 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_hits2_text.parse::<u32>() {
        Ok(inning) => app.debug_state.hits_team2 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_errors1_text.parse::<u32>() {
        Ok(inning) => app.debug_state.errors_team1 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
    match app.debug_errors2_text.parse::<u32>() {
        Ok(inning) => app.debug_state.errors_team2 = inning,
        Err(_) => {} // don't do anything if user is typing, weird characters, etc.
    }
}

/// populates ui for the version window
fn draw_version_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Version")
        .open(&mut app.version_window)
        .show(ctx, |ui| {
            ui.label("Version 0.3.0");
        });
}

// populates ui for the "About Deadball Game" window
fn draw_about_deadball_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("About Deadball Game")
        .open(&mut app.about_deadball_window)
        .show(ctx, |ui| {
            ui.label(ABOUT_DEABALL);
            ui.hyperlink("http://wmakers.net/deadball");
        });
}

/// populates ui for the "About this app" window
fn draw_about_app_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("About this app")
        .open(&mut app.about_app_window)
        .show(ctx, |ui| {
            ui.label(ABOUT_APP);
        });
}

/// draw the debug window
fn draw_debug_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Debug Mode")
        .open(&mut app.debug_window)
        .show(ctx, |ui| {
            // set debug state to current game state (if it exists)
            if app.game_state.is_some() && app.debug_copied == false {
                app.debug_state = app.game_state.clone().unwrap();
                app.debug_copied = true;
                app.debug_inning_text = app.debug_state.inning.clone().to_string();
            }
            ui.horizontal(|ui| {
                ui.label("Game Status:");
                egui::ComboBox::from_label("Select status.")
                    .selected_text(&app.debug_game_state_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_state.status,
                            GameStatus::NotStarted,
                            "Not Started",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.status,
                            GameStatus::Ongoing,
                            "Ongoing",
                        );
                        ui.selectable_value(&mut app.debug_state.status, GameStatus::Over, "Over");
                    })
            });
            ui.horizontal(|ui| {
                ui.label("Inning:");
                ui.text_edit_singleline(&mut app.debug_inning_text);
            });
            ui.horizontal(|ui| {
                ui.label("Inning Half:");
                egui::ComboBox::from_label("Select inning half.")
                    .selected_text(app.debug_inning_half_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.debug_state.inning_half, InningTB::Top, "^");
                        ui.selectable_value(
                            &mut app.debug_state.inning_half,
                            InningTB::Bottom,
                            "v",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Outs:");
                egui::ComboBox::from_label("Select outs.")
                    .selected_text(app.debug_outs_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.debug_state.outs, Outs::None, "None");
                        ui.selectable_value(&mut app.debug_state.outs, Outs::One, "One");
                        ui.selectable_value(&mut app.debug_state.outs, Outs::Two, "Two");
                        ui.selectable_value(&mut app.debug_state.outs, Outs::Three, "Three");
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Runners On:");
                egui::ComboBox::from_label("Select base runners.")
                    .selected_text(app.debug_runners_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner000,
                            "000",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner001,
                            "001",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner010,
                            "010",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner100,
                            "100",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner011,
                            "011",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner110,
                            "110",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner101,
                            "101",
                        );
                        ui.selectable_value(
                            &mut app.debug_state.runners,
                            RunnersOn::Runner111,
                            "111",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Batting Team 1:");
                ui.text_edit_singleline(&mut app.debug_batting1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Batting Team 2:");
                ui.text_edit_singleline(&mut app.debug_batting2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Pitched Team 1:");
                ui.text_edit_singleline(&mut app.debug_pitched1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Pitched Team 2:");
                ui.text_edit_singleline(&mut app.debug_pitched2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Runs Team 1:");
                ui.text_edit_singleline(&mut app.debug_runs1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Runs Team 2:");
                ui.text_edit_singleline(&mut app.debug_runs2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Hits Team 1:");
                ui.text_edit_singleline(&mut app.debug_hits1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Hits Team 2:");
                ui.text_edit_singleline(&mut app.debug_hits2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Errors Team 1:");
                ui.text_edit_singleline(&mut app.debug_errors1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Errors Team 2:");
                ui.text_edit_singleline(&mut app.debug_errors2_text);
            });
            // update debug game state combo box text
            match &app.debug_state.status {
                GameStatus::NotStarted => app.debug_game_state_text = "Not Started".to_string(),
                GameStatus::Ongoing => app.debug_game_state_text = "Ongoing".to_string(),
                GameStatus::Over => app.debug_game_state_text = "Over".to_string(),
            }
            // update inning half combo box text
            match &app.debug_state.inning_half {
                InningTB::Top => app.debug_inning_half_text = "^".to_string(),
                InningTB::Bottom => app.debug_inning_half_text = "v".to_string(),
            }
            // update outs combo box text
            match &app.debug_state.outs {
                Outs::None => app.debug_outs_text = "None".to_string(),
                Outs::One => app.debug_outs_text = "One".to_string(),
                Outs::Two => app.debug_outs_text = "Two".to_string(),
                Outs::Three => app.debug_outs_text = "Three".to_string(),
            }
            // update runners on text
            match &app.debug_state.runners {
                RunnersOn::Runner000 => app.debug_runners_text = "000".to_string(),
                RunnersOn::Runner001 => app.debug_runners_text = "001".to_string(),
                RunnersOn::Runner010 => app.debug_runners_text = "010".to_string(),
                RunnersOn::Runner100 => app.debug_runners_text = "100".to_string(),
                RunnersOn::Runner011 => app.debug_runners_text = "011".to_string(),
                RunnersOn::Runner110 => app.debug_runners_text = "110".to_string(),
                RunnersOn::Runner101 => app.debug_runners_text = "101".to_string(),
                RunnersOn::Runner111 => app.debug_runners_text = "111".to_string(),
            }
            // button to write changes to game state
            ui.separator();
            if ui.button("Write Changes").clicked() {
                // put players in the runner fields to avoid crashes
                let current_batter: i32;
                match app.debug_state.inning_half {
                    InningTB::Top => {
                        current_batter = app.debug_state.batting_team2 as i32;
                        match app.debug_state.runners {
                            RunnersOn::Runner000 => {}
                            RunnersOn::Runner100 => {
                                if current_batter == 1 {
                                    app.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [(current_batter - 1) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner010 => {
                                if current_batter == 1 {
                                    app.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner001 => {
                                if current_batter == 1 {
                                    app.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner110 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner101 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner011 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner111 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                let mut batter3 = current_batter - 3;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                if batter3 < 0 {
                                    batter3 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter3 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                        }
                    }
                    InningTB::Bottom => {
                        current_batter = app.debug_state.batting_team2 as i32;
                        match app.debug_state.runners {
                            RunnersOn::Runner000 => {}
                            RunnersOn::Runner100 => {
                                if current_batter == 1 {
                                    app.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner010 => {
                                if current_batter == 1 {
                                    app.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner001 => {
                                if current_batter == 1 {
                                    app.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner110 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner101 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner011 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                            RunnersOn::Runner111 => {
                                let mut batter1 = current_batter - 1;
                                let mut batter2 = current_batter - 2;
                                let mut batter3 = current_batter - 3;
                                if batter1 < 0 {
                                    batter1 += 9;
                                }
                                if batter2 < 0 {
                                    batter2 += 9;
                                }
                                if batter3 < 0 {
                                    batter3 += 9;
                                }
                                app.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter3 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                        }
                    }
                }
                app.game_state = Some(app.debug_state.clone());
            }
        });
}

/// draws the debug roll window
fn draw_debug_roll_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Roll Debug Mode")
        .open(&mut app.debug_roll_window)
        .show(ctx, |ui| {
            ui.checkbox(&mut app.debug_roll_state.mode, "Enable roll override.")
                .on_hover_text("Check to replace rolls with predetermined values.");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut app.debug_roll_text);
                if ui
                    .button("Add")
                    .on_hover_text("Add value to roll list.")
                    .clicked()
                {
                    if app.debug_roll_state.rolls.len() == 1 && app.debug_roll_state.rolls[0] == 0 {
                        if let Ok(val) = app.debug_roll_text.parse::<i32>() {
                            app.debug_roll_state.rolls[0] = val;
                        }
                    } else {
                        if let Ok(val) = app.debug_roll_text.parse::<i32>() {
                            app.debug_roll_state.rolls.push(val);
                        }
                    }
                }
                if ui
                    .button("Clear")
                    .on_hover_text("Clear roll list.")
                    .clicked()
                {
                    app.debug_roll_state.rolls = vec![0];
                }
            });
            ui.horizontal(|ui| {
                ui.label("Rolls:");
                for roll in app.debug_roll_state.rolls.iter() {
                    ui.label(roll.to_string());
                }
            });
        });
}

/// draws the console for displaying game text
fn draw_console_window(ctx: &Context, app: &mut DeadballApp) {
    let mut console_text: String;
    if app.game_state.is_some() {
        console_text = app.game_state.clone().unwrap().game_text;
    } else {
        console_text = "No game is currently active.".to_string();
    }
    egui::Window::new("Console")
        .open(&mut app.console_window)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut console_text);
                });
        });
}

/// renders the new game window
fn draw_create_new_game(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Create new game")
        .open(&mut app.create_game_window)
        .show(ctx, |ui| {
            // selectable value for game era
            ui.horizontal(|ui| {
                ui.label("Era:");
                ui.selectable_value(&mut app.create_game_era, Era::None, "None");
                ui.selectable_value(&mut app.create_game_era, Era::Modern, "Modern");
                ui.selectable_value(&mut app.create_game_era, Era::Ancient, "Ancient");
            });
            // file dialog for away team
            ui.horizontal(|ui| {
                ui.label("Away Team:");
                if let Some(away_file) = &mut app.away_team_file {
                    ui.label(format!("{:?}", away_file));
                } else {
                    ui.label("None");
                }
                if ui.button("Open").clicked() {
                    let mut dialog = FileDialog::open_file(app.away_team_file.clone());
                    dialog.open();
                    app.away_team_file_dialog = Some(dialog);
                }
                if let Some(dialog) = &mut app.away_team_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            app.away_team_file = Some(file);
                        }
                    }
                }
            });
            // file dialog for home team
            ui.horizontal(|ui| {
                ui.label("Home Team:");
                if let Some(home_file) = &mut app.home_team_file {
                    ui.label(format!("{:?}", home_file));
                } else {
                    ui.label("None");
                }
                if ui.button("Open").clicked() {
                    let mut dialog = FileDialog::open_file(app.home_team_file.clone());
                    dialog.open();
                    app.home_team_file_dialog = Some(dialog);
                }
                if let Some(dialog) = &mut app.home_team_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            app.home_team_file = Some(file);
                        }
                    }
                }
            });
            // file dialog for ball park
            ui.horizontal(|ui| {
                ui.label("Ballpark: ");
                if let Some(ballpark_file) = &mut app.ballpark_file {
                    ui.label(format!("{:?}", ballpark_file));
                } else {
                    ui.label("None");
                }
                if ui.button("Open").clicked() {
                    let mut dialog = FileDialog::open_file(app.ballpark_file.clone());
                    dialog.open();
                    app.ballpark_file_dialog = Some(dialog);
                }
                if let Some(dialog) = &mut app.ballpark_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            app.ballpark_file = Some(file);
                        }
                    }
                }
            });
            ui.separator();
            // button to create game and return to main screen
            app.create_game_error = "".to_owned();
            if ui.button("Create").clicked() {
                // check and make sure options are set properly
                if app.away_team_file.is_some()
                    && app.home_team_file.is_some()
                    && app.ballpark_file.is_some()
                {
                    // try to load teams and ballpark files
                    match fs::read_to_string(&app.away_team_file.as_ref().unwrap().as_path()) {
                        Ok(contents) => {
                            app.away_team = Some(load_team(contents));
                        }
                        Err(err) => {
                            app.create_game_error = app.create_game_error.clone()
                                + "Failed to read Away team file."
                                + &format!("{:?}", err);
                        }
                    }
                    match fs::read_to_string(&app.home_team_file.as_ref().unwrap().as_path()) {
                        Ok(contents) => {
                            app.home_team = Some(load_team(contents));
                        }
                        Err(err) => {
                            app.create_game_error = app.create_game_error.clone()
                                + "Failed to read Home team file."
                                + &format!("{:?}", err);
                        }
                    }
                    match app.create_game_era {
                        Era::Modern => {
                            match fs::read_to_string(&app.ballpark_file.as_ref().unwrap().as_path())
                            {
                                Ok(contents) => {
                                    app.ballpark_modern = Some(load_park_modern(contents));
                                }
                                Err(err) => {
                                    app.create_game_error = app.create_game_error.clone()
                                        + "Failed to read Ballpark file."
                                        + &format!("{:?}", err);
                                }
                            }
                        }
                        Era::Ancient => {
                            match fs::read_to_string(&app.ballpark_file.as_ref().unwrap().as_path())
                            {
                                Ok(contents) => {
                                    app.ballpark_ancient = Some(load_park_ancient(contents));
                                }
                                Err(err) => {
                                    app.create_game_error = app.create_game_error.clone()
                                        + "Failed to read Ballpark file."
                                        + &format!("{:?}", err);
                                }
                            }
                        }
                        Era::None => {
                            app.create_game_error =
                                app.create_game_error.clone() + "Please select an Era.";
                        }
                    }
                } else {
                    // update error message and display error window
                    if app.away_team_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbt file for away team.\n";
                    }
                    if app.home_team_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbt file for home team.\n";
                    }
                    if app.ballpark_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbb file for ballpark.\n";
                    }
                }
                match app.create_game_era {
                    Era::Modern => {
                        if app.away_team.is_some()
                            && app.home_team.is_some()
                            && app.ballpark_modern.is_some()
                        {
                            match create_modern_game(
                                app.home_team.clone().unwrap(),
                                app.away_team.clone().unwrap(),
                                app.ballpark_modern.clone().unwrap(),
                            ) {
                                Ok(game) => {
                                    app.game_modern = Some(game);
                                    app.home_team_active =
                                        Some(app.game_modern.clone().unwrap().home_active.clone());
                                    app.away_team_active =
                                        Some(app.game_modern.clone().unwrap().away_active.clone());
                                    // TODO: make the window close after successfully generating a game
                                }
                                Err(err) => {
                                    app.create_game_error =
                                        app.create_game_error.clone() + &format!("{:?}", err)
                                }
                            }
                        }
                    }
                    Era::Ancient => {
                        if app.away_team.is_some()
                            && app.home_team.is_some()
                            && app.ballpark_ancient.is_some()
                        {}
                    }
                    Era::None => {
                        app.create_game_error =
                            app.create_game_error.clone() + "Please select an Era.";
                    }
                }
            }
            // if everything loaded okay, generate game
            ui.add(eframe::egui::Label::new(
                RichText::new(&app.create_game_error).color(Color32::RED),
            ));
        });
}

/// renders the bottom panel
fn draw_bottom_panel(ctx: &Context, app: &mut DeadballApp) {
    egui::TopBottomPanel::bottom("Control Panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut app.bottom_panel, Panel::Game, "Game");
            ui.selectable_value(&mut app.bottom_panel, Panel::Menu, "Menu");
            ui.selectable_value(&mut app.bottom_panel, Panel::Roster, "Roster");
            ui.selectable_value(&mut app.bottom_panel, Panel::Debug, "Debug");
        });
        ui.separator();
        match app.bottom_panel {
            Panel::Menu => {
                ui.horizontal(|ui| {
                    ui.menu_button("Game", |ui| {
                        if ui.button("Create Game").clicked() {
                            app.create_game_window = true;
                            ui.close_menu();
                        }
                        if ui.button("Start Game").clicked() {
                            // check if there are active teams loaded
                            if app.home_team_active.is_some() && app.away_team_active.is_some() {
                                app.game_state = Some(init_new_game_state(
                                    app.home_team_active.clone().unwrap().pitching[0].clone(),
                                    app.away_team_active.clone().unwrap().pitching[0].clone(),
                                ));
                                println!(
                                    "Away: {} | Home: {}",
                                    app.game_state.as_ref().unwrap().batting_team2,
                                    app.game_state.as_ref().unwrap().batting_team1
                                ); // TODO: delete this
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
                            app.version_window = true;
                            ui.close_menu();
                        }
                        if ui.button("Help").clicked() {
                            // TODO: add help window
                            println!("No help menu available at this time.");
                            ui.close_menu();
                        }
                        if ui.button("About Deadball").clicked() {
                            app.about_deadball_window = true;
                            ui.close_menu();
                        }
                        if ui.button("About This App").clicked() {
                            app.about_app_window = true;
                            ui.close_menu();
                        }
                    })
                });
            }
            Panel::Game => {
                ui.horizontal(|ui| {
                    if ui.button("Next At Bat").clicked() {
                        // TODO: this could be cleaner
                        if app.game_state.is_some() && app.game_modern.is_some() {
                            // TODO: update with ancient game when ready
                            match app.game_state.clone().unwrap().status {
                                GameStatus::NotStarted => {
                                    app.game_state.as_mut().unwrap().status = GameStatus::Ongoing
                                }
                                GameStatus::Ongoing => {
                                    app.game_state = Some(modern_game_flow(
                                        &app.game_modern.clone().unwrap(),
                                        app.game_state.clone().unwrap(),
                                        app.debug_roll_state.clone(),
                                    ));
                                    println!("{:?}", app.game_state);
                                }
                                GameStatus::Over => {}
                            }
                            println!(
                                "Away: {} | Home: {}",
                                app.game_state.as_ref().unwrap().batting_team2,
                                app.game_state.as_ref().unwrap().batting_team1
                            ); // TODO: delete this
                        }
                    }
                    ui.menu_button("Steal", |ui| {
                        // evaluate if steal conditions are met, show relevant options
                        if app.game_state.is_some() {
                            let mut steal2 = false;
                            let mut steal3 = false;
                            let mut steal4 = false;
                            let mut double_steal = false;
                            // runner on 1st can steal 2nd
                            match app.game_state.as_ref().unwrap().runners {
                                RunnersOn::Runner000 => {}
                                RunnersOn::Runner100 => steal2 = true,
                                RunnersOn::Runner010 => steal3 = true,
                                RunnersOn::Runner001 => {
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() {
                                        if runner3.unwrap().speedy() {
                                            steal4 = true;
                                        }
                                    }
                                }
                                RunnersOn::Runner110 => {
                                    steal3 = true;
                                    double_steal = true;
                                }
                                RunnersOn::Runner101 => {
                                    steal2 = true;
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() {
                                        if runner3.unwrap().speedy() {
                                            steal4 = true;
                                        }
                                    }
                                }
                                RunnersOn::Runner011 => {
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() {
                                        if runner3.unwrap().speedy() {
                                            steal4 = true;
                                        }
                                    }
                                }
                                RunnersOn::Runner111 => {
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() {
                                        if runner3.unwrap().speedy() {
                                            steal4 = true;
                                        }
                                    }
                                }
                            }
                            if steal2 {
                                if ui.button("Steal 2nd").clicked() {
                                    app.game_state = Some(process_steals(
                                        StealType::Second,
                                        app.game_state.clone().unwrap(),
                                        app.debug_roll_state.clone(),
                                    ));
                                }
                            }
                            if steal3 {
                                if ui.button("Steal 3rd").clicked() {
                                    app.game_state = Some(process_steals(
                                        StealType::Third,
                                        app.game_state.clone().unwrap(),
                                        app.debug_roll_state.clone(),
                                    ));
                                }
                            }
                            if steal4 {
                                if ui.button("Steal Home").clicked() {
                                    app.game_state = Some(process_steals(
                                        StealType::Home,
                                        app.game_state.clone().unwrap(),
                                        app.debug_roll_state.clone(),
                                    ));
                                }
                            }
                            if double_steal {
                                if ui.button("Double Steal").clicked() {
                                    app.game_state = Some(process_steals(
                                        StealType::Double,
                                        app.game_state.clone().unwrap(),
                                        app.debug_roll_state.clone(),
                                    ));
                                }
                            }
                        } else {
                            ui.label("No active game.");
                        }
                    });
                    if ui.button("Bunt").clicked() {
                        if app.game_state.is_some() && app.game_modern.is_some() {
                            // TODO: check and make sure base runners make sense
                            let batter: Player;
                            match app.game_state.as_ref().unwrap().inning_half {
                                InningTB::Top => {
                                    let bat_num = app.game_state.as_ref().unwrap().batting_team2;
                                    batter =
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [bat_num as usize]
                                            .clone();
                                }
                                InningTB::Bottom => {
                                    let bat_num = app.game_state.as_ref().unwrap().batting_team1;
                                    batter =
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [bat_num as usize]
                                            .clone();
                                }
                            }
                            app.game_state = Some(bunt(
                                app.game_state.clone().unwrap(),
                                app.game_modern.as_ref().unwrap(),
                                app.debug_roll_state.clone(),
                                batter,
                            ));
                        }
                    }
                    if ui.button("Hit & Run").clicked() {
                        if app.game_state.is_some() && app.game_modern.is_some() {
                            let batter: Player;
                            match app.game_state.as_ref().unwrap().inning_half {
                                InningTB::Top => {
                                    let bat_num = app.game_state.as_ref().unwrap().batting_team2;
                                    batter =
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [bat_num as usize]
                                            .clone();
                                }
                                InningTB::Bottom => {
                                    let bat_num = app.game_state.as_ref().unwrap().batting_team1;
                                    batter =
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [bat_num as usize]
                                            .clone();
                                }
                            }
                            app.game_state = Some(hit_and_run(
                                app.game_state.clone().unwrap(),
                                app.game_modern.as_ref().unwrap(),
                                &mut app.debug_roll_state,
                                batter,
                            ));
                        }
                    }
                });
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
                        app.debug_window = true;
                        app.debug_copied = false;
                    }
                    if ui.button("Roll").clicked() {
                        app.debug_roll_window = true;
                    }
                    if ui.button("Console").clicked() {
                        app.console_window = true;
                    }
                });
            }
        }
    });
}

/// render left panel
fn draw_left_panel(ctx: &Context, app: &mut DeadballApp) {
    egui::SidePanel::left("Away Team").show(ctx, |ui| {
        ui.heading(&app.away_team_name);
        let away_name1: String;
        let away_name2: String;
        let away_name3: String;
        let away_name4: String;
        let away_name5: String;
        let away_name6: String;
        let away_name7: String;
        let away_name8: String;
        let away_name9: String;
        let mut away_info1 = "".to_string();
        let mut away_info2 = "".to_string();
        let mut away_info3 = "".to_string();
        let mut away_info4 = "".to_string();
        let mut away_info5 = "".to_string();
        let mut away_info6 = "".to_string();
        let mut away_info7 = "".to_string();
        let mut away_info8 = "".to_string();
        let mut away_info9 = "".to_string();
        if app.away_team.is_some() {
            let away_team = app.away_team.as_ref().unwrap();
            let away_team_active = app.game_modern.clone().unwrap().away_active;
            app.away_team_name = away_team.name.to_string();
            app.away_batter1 = Some(away_team_active.roster[0].clone());
            app.away_batter2 = Some(away_team_active.roster[1].clone());
            app.away_batter3 = Some(away_team_active.roster[2].clone());
            app.away_batter4 = Some(away_team_active.roster[3].clone());
            app.away_batter5 = Some(away_team_active.roster[4].clone());
            app.away_batter6 = Some(away_team_active.roster[5].clone());
            app.away_batter7 = Some(away_team_active.roster[6].clone());
            app.away_batter8 = Some(away_team_active.roster[7].clone());
            app.away_batter9 = Some(away_team_active.pitching[0].clone());
            let batter1 = app.away_batter1.clone().unwrap();
            let batter2 = app.away_batter2.clone().unwrap();
            let batter3 = app.away_batter3.clone().unwrap();
            let batter4 = app.away_batter4.clone().unwrap();
            let batter5 = app.away_batter5.clone().unwrap();
            let batter6 = app.away_batter6.clone().unwrap();
            let batter7 = app.away_batter7.clone().unwrap();
            let batter8 = app.away_batter8.clone().unwrap();
            let batter9 = app.away_batter9.clone().unwrap();
            // TODO: clean this up (use local var)
            away_name1 = format!("{} {}", batter1.first_name, batter1.last_name);
            away_name2 = format!("{} {}", batter2.first_name, batter2.last_name);
            away_name3 = format!("{} {}", batter3.first_name, batter3.last_name);
            away_name4 = format!("{} {}", batter4.first_name, batter4.last_name);
            away_name5 = format!("{} {}", batter5.first_name, batter5.last_name);
            away_name6 = format!("{} {}", batter6.first_name, batter6.last_name);
            away_name7 = format!("{} {}", batter7.first_name, batter7.last_name);
            away_name8 = format!("{} {}", batter8.first_name, batter8.last_name);
            away_name9 = format!("{} {}", batter9.first_name, batter9.last_name);
            away_info1 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter1.position,
                batter1.handedness,
                batter1.batter_target,
                batter1.on_base_target,
                batter1.traits,
            );
            away_info2 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter2.position,
                batter2.handedness,
                batter2.batter_target,
                batter2.on_base_target,
                batter2.traits,
            );
            away_info3 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter3.position,
                batter3.handedness,
                batter3.batter_target,
                batter3.on_base_target,
                batter3.traits,
            );
            away_info4 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter4.position,
                batter4.handedness,
                batter4.batter_target,
                batter4.on_base_target,
                batter4.traits,
            );
            away_info5 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter5.position,
                batter5.handedness,
                batter5.batter_target,
                batter5.on_base_target,
                batter5.traits,
            );
            away_info6 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter6.position,
                batter6.handedness,
                batter6.batter_target,
                batter6.on_base_target,
                batter6.traits,
            );
            away_info7 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter7.position,
                batter7.handedness,
                batter7.batter_target,
                batter7.on_base_target,
                batter7.traits,
            );
            away_info8 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter8.position,
                batter8.handedness,
                batter8.batter_target,
                batter8.on_base_target,
                batter8.traits,
            );
            away_info9 = format!(
                "{:?} | {:?} | {} | {} | {:?} ",
                batter9.position,
                batter9.handedness,
                batter9.batter_target,
                batter9.on_base_target,
                batter9.traits,
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
        let mut away_at_bat = 1;
        if app.game_state.is_some() {
            away_at_bat = app.game_state.clone().unwrap().batting_team2 + 1; // array indexing :/
        }
        ui.horizontal(|ui| {
            if away_at_bat == 1 {
                ui.label(RichText::new("1. ").strong());
                ui.label(RichText::new(away_name1).strong())
                    .on_hover_text(away_info1);
            // TODO: figure out a way to put baseball icon to indicate current batter
            } else {
                ui.label("1. ");
                ui.label(away_name1).on_hover_text(away_info1);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 2 {
                ui.label(RichText::new("2. ").strong());
                ui.label(RichText::new(away_name2).strong())
                    .on_hover_text(away_info2);
            } else {
                ui.label("2. ");
                ui.label(away_name2).on_hover_text(away_info2);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 3 {
                ui.label(RichText::new("3. ").strong());
                ui.label(RichText::new(away_name3).strong())
                    .on_hover_text(away_info3);
            } else {
                ui.label("3. ");
                ui.label(away_name3).on_hover_text(away_info3);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 4 {
                ui.label(RichText::new("4. ").strong());
                ui.label(RichText::new(away_name4).strong())
                    .on_hover_text(away_info4);
            } else {
                ui.label("4. ");
                ui.label(away_name4).on_hover_text(away_info4);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 5 {
                ui.label(RichText::new("5. ").strong());
                ui.label(RichText::new(away_name5).strong())
                    .on_hover_text(away_info5);
            } else {
                ui.label("5. ");
                ui.label(away_name5).on_hover_text(away_info5);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 6 {
                ui.label(RichText::new("6. ").strong());
                ui.label(RichText::new(away_name6).strong())
                    .on_hover_text(away_info6);
            } else {
                ui.label("6. ");
                ui.label(away_name6).on_hover_text(away_info6);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 7 {
                ui.label(RichText::new("7. ").strong());
                ui.label(RichText::new(away_name7).strong())
                    .on_hover_text(away_info7);
            } else {
                ui.label("7. ");
                ui.label(away_name7).on_hover_text(away_info7);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 8 {
                ui.label(RichText::new("8. ").strong());
                ui.label(RichText::new(away_name8).strong())
                    .on_hover_text(away_info8);
            } else {
                ui.label("8. ");
                ui.label(away_name8).on_hover_text(away_info8);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 9 {
                ui.label(RichText::new("9. ").strong());
                ui.label(RichText::new(away_name9).strong())
                    .on_hover_text(away_info9);
            } else {
                ui.label("9. ");
                ui.label(away_name9).on_hover_text(away_info9);
            }
        });
    });
}

/// renders the right panel of the main interface
fn draw_right_panel(ctx: &Context, app: &mut DeadballApp) {
    egui::SidePanel::right("Home Team").show(ctx, |ui| {
        ui.heading(&app.home_team_name);
        let mut home_info1 = "".to_string();
        let mut home_info2 = "".to_string();
        let mut home_info3 = "".to_string();
        let mut home_info4 = "".to_string();
        let mut home_info5 = "".to_string();
        let mut home_info6 = "".to_string();
        let mut home_info7 = "".to_string();
        let mut home_info8 = "".to_string();
        let mut home_info9 = "".to_string();
        if app.home_team.is_some() {
            let home_team = app.home_team.as_ref().unwrap();
            app.home_team_name = home_team.name.to_string();
            // TODO: use batting_order instead?
            let batter1 = &app.game_modern.clone().unwrap().home_active.roster[0];
            app.home_batter1 = format!(
                "{} {}",
                app.game_modern.clone().unwrap().home_active.roster[0].first_name,
                app.game_modern.clone().unwrap().home_active.roster[0].last_name
            );
            let batter2 = &app.game_modern.clone().unwrap().home_active.roster[1];
            app.home_batter2 = format!("{} {}", &batter2.first_name, &batter2.last_name);
            let batter3 = &app.game_modern.clone().unwrap().home_active.roster[2];
            app.home_batter3 = format!("{} {}", &batter3.first_name, &batter3.last_name);
            let batter4 = &app.game_modern.clone().unwrap().home_active.roster[3];
            app.home_batter4 = format!("{} {}", &batter4.first_name, &batter4.last_name);
            let batter5 = &app.game_modern.clone().unwrap().home_active.roster[4];
            app.home_batter5 = format!("{} {}", &batter5.first_name, &batter5.last_name);
            let batter6 = &app.game_modern.clone().unwrap().home_active.roster[5];
            app.home_batter6 = format!("{} {}", &batter6.first_name, &batter6.last_name);
            let batter7 = &app.game_modern.clone().unwrap().home_active.roster[6];
            app.home_batter7 = format!("{} {}", &batter7.first_name, &batter7.last_name);
            let batter8 = &app.game_modern.clone().unwrap().home_active.roster[7];
            app.home_batter8 = format!("{} {}", &batter8.first_name, &batter8.last_name);
            let batter9 = &app.game_modern.clone().unwrap().home_active.pitching[0];
            app.home_batter9 = format!("{} {}", &batter9.first_name, &batter9.last_name);
            home_info1 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter1.position,
                batter1.handedness,
                batter1.batter_target,
                batter1.on_base_target,
                batter1.traits
            );
            home_info2 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter2.position,
                batter2.handedness,
                batter2.batter_target,
                batter2.on_base_target,
                batter2.traits
            );
            home_info3 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter3.position,
                batter3.handedness,
                batter3.batter_target,
                batter3.on_base_target,
                batter3.traits
            );
            home_info4 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter4.position,
                batter4.handedness,
                batter4.batter_target,
                batter4.on_base_target,
                batter4.traits
            );
            home_info5 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter5.position,
                batter5.handedness,
                batter5.batter_target,
                batter5.on_base_target,
                batter5.traits
            );
            home_info6 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter6.position,
                batter6.handedness,
                batter6.batter_target,
                batter6.on_base_target,
                batter6.traits
            );
            home_info7 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter7.position,
                batter7.handedness,
                batter7.batter_target,
                batter7.on_base_target,
                batter7.traits
            );
            home_info8 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter8.position,
                batter8.handedness,
                batter8.batter_target,
                batter8.on_base_target,
                batter8.traits
            );
            home_info9 = format!(
                "{:?} | {:?} | {} | {} | {:?}",
                batter9.position,
                batter9.handedness,
                batter9.batter_target,
                batter9.on_base_target,
                batter9.traits
            );
        }
        let mut home_at_bat = 1;
        if app.game_state.is_some() {
            home_at_bat = app.game_state.clone().unwrap().batting_team1 + 1; // array indexing :/
        }
        ui.horizontal(|ui| {
            if home_at_bat == 1 {
                ui.label(RichText::new("1. ").strong());
                ui.label(RichText::new(app.home_batter1.clone()).strong())
                    .on_hover_text(home_info1);
            } else {
                ui.label("1. ");
                ui.label(&app.home_batter1).on_hover_text(home_info1);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 2 {
                ui.label(RichText::new("2. ").strong());
                ui.label(RichText::new(app.home_batter2.clone()).strong())
                    .on_hover_text(home_info2);
            } else {
                ui.label("2. ");
                ui.label(&app.home_batter2).on_hover_text(home_info2);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 3 {
                ui.label(RichText::new("3. ").strong());
                ui.label(RichText::new(app.home_batter3.clone()).strong())
                    .on_hover_text(home_info3);
            } else {
                ui.label("3. ");
                ui.label(&app.home_batter3).on_hover_text(home_info3);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 4 {
                ui.label(RichText::new("4. ").strong());
                ui.label(RichText::new(app.home_batter4.clone()).strong())
                    .on_hover_text(home_info4);
            } else {
                ui.label("4. ");
                ui.label(&app.home_batter4).on_hover_text(home_info4);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 5 {
                ui.label(RichText::new("5. ").strong());
                ui.label(RichText::new(app.home_batter5.clone()).strong())
                    .on_hover_text(home_info5);
            } else {
                ui.label("5. ");
                ui.label(&app.home_batter5).on_hover_text(home_info5);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 6 {
                ui.label(RichText::new("6. ").strong());
                ui.label(RichText::new(app.home_batter6.clone()).strong())
                    .on_hover_text(home_info6);
            } else {
                ui.label("6. ");
                ui.label(&app.home_batter6).on_hover_text(home_info6);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 7 {
                ui.label(RichText::new("7. ").strong());
                ui.label(RichText::new(app.home_batter7.clone()).strong())
                    .on_hover_text(home_info7);
            } else {
                ui.label("7. ");
                ui.label(&app.home_batter7).on_hover_text(home_info7);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 8 {
                ui.label(RichText::new("8. ").strong());
                ui.label(RichText::new(app.home_batter8.clone()).strong())
                    .on_hover_text(home_info8);
            } else {
                ui.label("8. ");
                ui.label(&app.home_batter8).on_hover_text(home_info8);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 9 {
                ui.label(RichText::new("9. ").strong());
                ui.label(RichText::new(app.home_batter9.clone()).strong())
                    .on_hover_text(home_info9);
            } else {
                ui.label("9. ");
                ui.label(&app.home_batter9).on_hover_text(home_info9);
            }
        });
    });
}
