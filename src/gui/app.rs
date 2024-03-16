use crate::characters::ballparks::{BallparkAncient, BallparkModern};
/*==============================================================================================
 * IMPORTS
 * ===========================================================================================*/
// LOCAL IMPORTS
use super::draw_fn::*;
use crate::characters::{players::*, teams::*};
use crate::core::file_locations::{load_databases, DeadballDatabases};
//use deadball::core::file_locations::*;
use super::gui_functions::{
    batter_tooltip, update_debug_textedits, CreateBallparkWindow, CreatePlayerWindow,
    CreateTeamWindow, ToastData,
};
use crate::core::game_functions::{
    bunt, find_by_position, hit_and_run, init_new_game_state, modern_game_flow,
    new_game_state_struct, process_steals, GameModern, GameState, GameStatus, InningTB, Outs,
    RunnersOn, StealType,
};
use crate::{
    gui::debug::DebugConfig,
    gui::gui_functions::{runners_on_bool, update_player_labels},
};

use std::usize;

use eframe::egui::Image;
// EXTERNAL IMPORTS
use eframe::{
    egui::{self, Context},
    epaint::{pos2, Color32},
};
use egui::{Rect, RichText};
use egui_file::FileDialog;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use std::path::PathBuf;

/*==============================================================================================
 * CONSTANTS
 * ===========================================================================================*/
pub const ABOUT_DEABALL: &str =
    "Deadball: Baseball With Dice is a tabletop game developed by W.M. Akers.  For more information about the game, or to purchase the rules, please visit the Deadball website.";

pub const ABOUT_APP: &str = "This application was developed as practice with the Rust programming language.  All credit goes to the creator of Deadball, W.M. Akers.  Please purchase and consult the official rulebooks for questions about game mechanics.";

pub const CUSTOM_TOAST: u32 = 0;
fn custom_toast_contents(ui: &mut egui::Ui, toast: &mut Toast) -> egui::Response {
    egui::Frame::window(ui.style())
        .show(ui, |ui| {
            ui.label(toast.text.clone());
        })
        .response
}

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
/// info for GUI scoreboard display
pub struct Score {
    pub current_inning: String,
    pub current_outs: String,
    pub away_hits: String,
    pub away_errors: String,
    pub away_runs: String,
    pub home_hits: String,
    pub home_errors: String,
    pub home_runs: String,
}

impl Default for Score {
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
        }
    }
}

/// images for GUI
pub struct GuiImages<'a> {
    pub diamond_image: Image<'a>,
    pub helmet_image: Image<'a>,
}

impl Default for GuiImages<'_> {
    fn default() -> Self {
        Self {
            diamond_image: Image::new(egui::include_image!("images/baseball_diamond.png")),
            helmet_image: Image::new(egui::include_image!("images/helmet.png")),
        }
    }
}

/// labels for baseball diamond
pub struct DiamondLabels {
    pub pitcher_label: String,
    pub catcher_label: String,
    pub firstbase_label: String,
    pub secondbase_label: String,
    pub shortstop_label: String,
    pub thirdbase_label: String,
    pub rightfield_label: String,
    pub centerfield_label: String,
    pub leftfield_label: String,
}

impl Default for DiamondLabels {
    fn default() -> Self {
        Self {
            pitcher_label: "P: Seth Loveall".to_string(),
            catcher_label: "C: Seth Loveall".to_string(),
            firstbase_label: "1B: Seth Loveall".to_string(),
            secondbase_label: "2B: Seth Loveall".to_string(),
            shortstop_label: "SS: Seth Loveall".to_string(),
            thirdbase_label: "3B: Seth Loveall".to_string(),
            rightfield_label: "RF: Seth Loveall".to_string(),
            centerfield_label: "CF: Seth Loveall".to_string(),
            leftfield_label: "LF: Seth Loveall".to_string(),
        }
    }
}

/// contains bools for controlling sub windows
pub struct GuiWindows {
    pub version_window: bool,
    pub about_deadball_window: bool,
    pub about_app_window: bool,
    pub create_game_window: bool,
    pub debug_window: bool,
    pub debug_roll_window: bool,
    pub console_window: bool,
}

impl Default for GuiWindows {
    fn default() -> Self {
        Self {
            version_window: false,
            about_deadball_window: false,
            about_app_window: false,
            create_game_window: false,
            debug_window: false,
            debug_roll_window: false,
            console_window: true,
        }
    }
}

/// team batting order string structure
pub struct BattingOrderStrings {
    pub batter1: String,
    pub batter2: String,
    pub batter3: String,
    pub batter4: String,
    pub batter5: String,
    pub batter6: String,
    pub batter7: String,
    pub batter8: String,
    pub batter9: String,
}

impl Default for BattingOrderStrings {
    fn default() -> Self {
        Self {
            batter1: "".to_string(),
            batter2: "".to_string(),
            batter3: "".to_string(),
            batter4: "".to_string(),
            batter5: "".to_string(),
            batter6: "".to_string(),
            batter7: "".to_string(),
            batter8: "".to_string(),
            batter9: "".to_string(),
        }
    }
}

pub struct DebugSettings {
    pub debug_copied: bool, // copy game state to debug state first time window is opened
    pub debug_state: GameState,
    pub debug_game_state_text: String,
    pub debug_inning_text: String,
    pub debug_inning_half_text: String,
    pub debug_outs_text: String,
    pub debug_runners_text: String,
    pub debug_batting1_text: String,
    pub debug_batting2_text: String,
    pub debug_pitched1_text: String,
    pub debug_pitched2_text: String,
    pub debug_runs1_text: String,
    pub debug_runs2_text: String,
    pub debug_hits1_text: String,
    pub debug_hits2_text: String,
    pub debug_errors1_text: String,
    pub debug_errors2_text: String,
    pub debug_roll_state: DebugConfig,
    pub debug_roll_text: String,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
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

pub struct DeadballApp<'a> {
    // score information
    pub score: Score,
    // ballfield interface
    pub gui_images: GuiImages<'a>,
    pub diamond_labels: DiamondLabels,
    // batting order interface
    pub away_team_name: String,
    pub home_team_name: String,
    // menu/controls interface
    bottom_panel: Panel,
    // tracking for other windows
    pub gui_windows: GuiWindows,
    // create game interface
    pub create_game_era: Era,
    pub away_team_file: Option<PathBuf>,
    pub away_team_file_dialog: Option<FileDialog>,
    pub home_team_file: Option<PathBuf>,
    pub home_team_file_dialog: Option<FileDialog>,
    pub ballpark_file: Option<PathBuf>,
    pub ballpark_file_dialog: Option<FileDialog>,
    pub oddity: bool,
    pub create_game_error: String,
    // game data
    pub away_team: Option<Team>,
    pub away_team_active: Option<ActiveTeam>,
    pub away_bo: BattingOrderStrings,
    pub home_team: Option<Team>,
    pub home_team_active: Option<ActiveTeam>,
    pub home_bo: BattingOrderStrings,
    pub ballpark_modern: Option<BallparkModern>,
    pub ballpark_ancient: Option<BallparkAncient>,
    pub game_modern: Option<GameModern>,
    pub game_state: Option<GameState>,
    // TODO: add ancient game
    // debug settings
    pub debug_settings: DebugSettings,
    pub toast_options: ToastData,
    pub create_team: CreateTeamWindow,
    pub create_player: CreatePlayerWindow,
    pub create_ballpark: CreateBallparkWindow,
    pub databases: DeadballDatabases,
}

impl Default for DeadballApp<'_> {
    fn default() -> Self {
        Self {
            score: Score::default(),
            gui_images: GuiImages::default(),
            diamond_labels: DiamondLabels::default(),
            away_team_name: "Away Team".to_owned(),
            home_team_name: "Home Team".to_owned(),
            bottom_panel: Panel::Menu,
            gui_windows: GuiWindows::default(),
            create_game_era: Era::None,
            away_team_file: None,
            away_team_file_dialog: None,
            home_team_file: None,
            home_team_file_dialog: None,
            ballpark_file: None,
            ballpark_file_dialog: None,
            oddity: false,
            create_game_error: "".to_owned(),
            away_team: None,
            away_team_active: None,
            away_bo: BattingOrderStrings::default(),
            home_team: None,
            home_team_active: None,
            home_bo: BattingOrderStrings::default(),
            ballpark_modern: None,
            ballpark_ancient: None,
            game_modern: None,
            game_state: None,
            debug_settings: DebugSettings::default(),
            toast_options: ToastData::default(),
            create_team: CreateTeamWindow::default(),
            create_player: CreatePlayerWindow::default(),
            create_ballpark: CreateBallparkWindow::default(),
            databases: DeadballDatabases::default(),
        }
    }
}

impl eframe::App for DeadballApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // toast notification stuff
        let mut toasts = Toasts::new()
            .anchor(self.toast_options.alignment, self.toast_options.offset)
            .direction(self.toast_options.direction)
            .custom_contents(CUSTOM_TOAST, custom_toast_contents);

        // check if databases need to be loaded
        if !self.databases.loaded {
            self.databases = load_databases(&mut toasts);
        }

        // app state updates
        update_debug_textedits(self);
        // draw other windows (if needed)
        draw_version_window(ctx, self);
        draw_about_deadball_window(ctx, self);
        draw_about_app_window(ctx, self);
        draw_debug_window(ctx, self);
        draw_create_new_game(ctx, self, &mut toasts);
        draw_debug_roll_window(ctx, self);
        draw_console_window(ctx, self);
        draw_create_team_window(ctx, self, &mut toasts);
        draw_create_player_window(ctx, self, &mut toasts);
        draw_create_ballpark_window(ctx, self, &mut toasts);

        // main window
        draw_bottom_panel(ctx, self, &mut toasts);
        draw_left_panel(ctx, self);
        draw_right_panel(ctx, self);
        egui::CentralPanel::default().show(ctx, |ui| {
            // update GUI scoreboard values (if game is in progress)
            let mut on_first = false;
            let mut on_second = false;
            let mut on_third = false;
            if self.game_state.is_some() {
                let inning_number = self.game_state.as_ref().unwrap().inning.to_string();
                let inning_top_bottom: &str = match self.game_state.as_ref().unwrap().inning_half {
                    InningTB::Top => "^",
                    InningTB::Bottom => "v",
                };
                self.score.current_inning = inning_number + inning_top_bottom;
                self.score.away_hits = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .away_state
                    .hits
                    .to_string();
                self.score.away_errors = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .away_state
                    .errors
                    .to_string();
                self.score.away_runs = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .away_state
                    .runs
                    .to_string();
                let out_string: String = match self.game_state.as_ref().unwrap().outs {
                    Outs::None => "0".to_string(),
                    Outs::One => "1".to_string(),
                    Outs::Two => "2".to_string(),
                    Outs::Three => "3".to_string(),
                };
                self.score.current_outs = out_string;
                self.score.home_hits = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .home_state
                    .hits
                    .to_string();
                self.score.home_errors = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .home_state
                    .errors
                    .to_string();
                self.score.home_runs = self
                    .game_state
                    .as_ref()
                    .unwrap()
                    .home_state
                    .runs
                    .to_string();
                (on_first, on_second, on_third) =
                    runners_on_bool(self.game_state.clone().unwrap().runners);
            }
            // score line
            ui.horizontal(|ui| {
                ui.label("Inning:");
                ui.label(&self.score.current_inning);
                ui.label("AWAY");
                ui.label("hits:");
                ui.label(&self.score.away_hits);
                ui.label("errors:");
                ui.label(&self.score.away_errors);
                ui.label("runs:");
                ui.label(&self.score.away_runs);
            });
            ui.horizontal(|ui| {
                ui.label("Outs:");
                ui.label(&self.score.current_outs);
                ui.label("HOME");
                ui.label("hits:");
                ui.label(&self.score.home_hits);
                ui.label("errors:");
                ui.label(&self.score.home_errors);
                ui.label("runs:");
                ui.label(&self.score.home_runs);
            });
            // draw baseball field and label players
            ui.add(
                self.gui_images
                    .diamond_image
                    .clone()
                    .max_size(egui::Vec2 { x: 511.8, y: 445.2 }),
            );
            // draw helmets to indicate runners on base
            if on_first {
                // no error, game state should already exist
                ui.put(
                    Rect {
                        min: pos2(490.0, 260.0),
                        max: pos2(590.0, 360.0),
                    },
                    self.gui_images
                        .helmet_image
                        .clone()
                        .max_size(egui::Vec2 { x: 51.2, y: 51.2 }),
                )
                .on_hover_text(batter_tooltip(
                    &self.game_state.as_ref().unwrap().runner1.clone().unwrap(),
                ));
            }
            if on_second {
                ui.put(
                    Rect {
                        min: pos2(340.0, 120.0),
                        max: pos2(440.0, 220.0),
                    },
                    self.gui_images
                        .helmet_image
                        .clone()
                        .max_size(egui::Vec2 { x: 51.2, y: 51.2 }),
                )
                .on_hover_text(batter_tooltip(
                    &self.game_state.as_ref().unwrap().runner2.clone().unwrap(),
                ));
            }
            if on_third {
                ui.put(
                    Rect {
                        min: pos2(205.0, 270.0),
                        max: pos2(305.0, 370.0),
                    },
                    self.gui_images
                        .helmet_image
                        .clone()
                        .max_size(egui::Vec2 { x: 51.2, y: 51.2 }),
                )
                .on_hover_text(batter_tooltip(
                    &self.game_state.as_ref().unwrap().runner3.clone().unwrap(),
                ));
            }
            if self.game_state.is_some() {
                // always draw batter
                let batter: &Player = match self.game_state.as_ref().unwrap().inning_half {
                    InningTB::Top => {
                        &self.game_modern.as_ref().unwrap().away_active.batting_order
                            [self.game_state.as_ref().unwrap().away_state.current_batter as usize]
                    }
                    InningTB::Bottom => {
                        &self.game_modern.as_ref().unwrap().home_active.batting_order
                            [self.game_state.as_ref().unwrap().home_state.current_batter as usize]
                    }
                };
                ui.put(
                    Rect {
                        min: pos2(340.0, 475.0),
                        max: pos2(440.0, 495.0),
                    },
                    self.gui_images.helmet_image.clone(),
                )
                .on_hover_text(batter_tooltip(batter));
            }
            // update player labels
            if self.home_team_active.is_some()
                && self.away_team_active.is_some()
                && self.game_state.is_some()
            {
                let labels: Vec<String> = match self.game_state.as_ref().unwrap().inning_half {
                    InningTB::Top => update_player_labels(self.home_team_active.as_ref().unwrap()),
                    InningTB::Bottom => {
                        update_player_labels(self.away_team_active.as_ref().unwrap())
                    }
                };
                self.diamond_labels.firstbase_label = labels[0].clone();
                self.diamond_labels.secondbase_label = labels[1].clone();
                self.diamond_labels.shortstop_label = labels[2].clone();
                self.diamond_labels.thirdbase_label = labels[3].clone();
                self.diamond_labels.catcher_label = labels[4].clone();
                self.diamond_labels.leftfield_label = labels[5].clone();
                self.diamond_labels.centerfield_label = labels[6].clone();
                self.diamond_labels.rightfield_label = labels[7].clone();
                self.diamond_labels.pitcher_label = labels[8].clone();
            }
            // put player names
            ui.put(
                Rect {
                    min: pos2(460.0, 260.0),
                    max: pos2(560.0, 280.0),
                },
                eframe::egui::Label::new(
                    RichText::new(&self.diamond_labels.firstbase_label)
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
                    RichText::new(&self.diamond_labels.secondbase_label)
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
                    RichText::new(&self.diamond_labels.pitcher_label)
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
                    RichText::new(&self.diamond_labels.catcher_label)
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
                    RichText::new(&self.diamond_labels.thirdbase_label)
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
                    RichText::new(&self.diamond_labels.shortstop_label)
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
                    RichText::new(&self.diamond_labels.rightfield_label)
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
                    RichText::new(&self.diamond_labels.centerfield_label)
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
                    RichText::new(&self.diamond_labels.leftfield_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE), //.size(16.0),
                ),
            );
        });
        toasts.show(ctx);
    }
}

/// renders the bottom panel
fn draw_bottom_panel(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
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
                            app.gui_windows.create_game_window = true;
                            ui.close_menu();
                        }
                        if ui.button("Start Game").clicked() {
                            // check if there are active teams loaded
                            if app.home_team_active.is_some() && app.away_team_active.is_some() {
                                app.game_state = Some(init_new_game_state(
                                    app.home_team_active.clone().unwrap().pitching[0].clone(),
                                    app.away_team_active.clone().unwrap().pitching[0].clone(),
                                ));
                                toasts.add(Toast {
                                    kind: ToastKind::Info,
                                    text: "Play ball!".into(),
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                });
                            } else {
                                println!("Load teams first.");
                                toasts.add(Toast {
                                    kind: ToastKind::Info,
                                    text: "Create a game first.".into(),
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                });
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
                            app.gui_windows.version_window = true;
                            ui.close_menu();
                        }
                        if ui.button("Help").clicked() {
                            // TODO: add help window
                            println!("No help menu available at this time.");
                            ui.close_menu();
                        }
                        if ui.button("About Deadball").clicked() {
                            app.gui_windows.about_deadball_window = true;
                            ui.close_menu();
                        }
                        if ui.button("About This App").clicked() {
                            app.gui_windows.about_app_window = true;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Teams", |ui| {
                        // create/edit/find teams
                        if ui.button("Create New Team").clicked() {
                            app.create_team.is_visible = true;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Players", |ui| {
                        // create/edit/find players
                        if ui.button("Create New Player").clicked() {
                            // TODO: player creation window - after figuring out file structure
                            app.create_player.is_visible = true;
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Ballparks", |ui| {
                        // create/edit/find ballparks
                        if ui.button("Create New Ballpark").clicked() {
                            // TODO: ballpark creation window - after figuring out file structure
                            app.create_ballpark.is_visible = true;
                            ui.close_menu();
                        }
                    });
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
                                        app.debug_settings.debug_roll_state.clone(),
                                    ));
                                    println!("{:?}", app.game_state);
                                }
                                GameStatus::Over => {
                                    toasts.add(Toast {
                                        kind: ToastKind::Info,
                                        text: "That's game!".into(),
                                        options: ToastOptions::default()
                                            .duration_in_seconds(3.0)
                                            .show_progress(true)
                                            .show_icon(true),
                                    });
                                }
                            }
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
                                    if runner3.is_some() && runner3.unwrap().speedy() {
                                        steal4 = true;
                                    }
                                }
                                RunnersOn::Runner110 => {
                                    steal3 = true;
                                    double_steal = true;
                                }
                                RunnersOn::Runner101 => {
                                    steal2 = true;
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() && runner3.unwrap().speedy() {
                                        steal4 = true;
                                    }
                                }
                                RunnersOn::Runner011 => {
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() && runner3.unwrap().speedy() {
                                        steal4 = true;
                                    }
                                }
                                RunnersOn::Runner111 => {
                                    let runner3 = app.game_state.as_ref().unwrap().runner3.clone();
                                    if runner3.is_some() && runner3.unwrap().speedy() {
                                        steal4 = true;
                                    }
                                }
                            }
                            // NOTE: I think it is okay to unwrap here, positions should exist when
                            // game is created/roster is loaded
                            let catcher: Player = match app.game_state.as_ref().unwrap().inning_half
                            {
                                InningTB::Top => find_by_position(
                                    Position::Catcher,
                                    &app.game_modern.as_ref().unwrap().home_active.roster,
                                )
                                .unwrap(),
                                InningTB::Bottom => find_by_position(
                                    Position::Catcher,
                                    &app.game_modern.as_ref().unwrap().away_active.roster,
                                )
                                .unwrap(),
                            };
                            if steal2 && ui.button("Steal 2nd").clicked() {
                                app.game_state = Some(process_steals(
                                    StealType::Second,
                                    app.game_state.clone().unwrap(),
                                    app.debug_settings.debug_roll_state.clone(),
                                    &catcher,
                                ));
                            }
                            if steal3 && ui.button("Steal 3rd").clicked() {
                                app.game_state = Some(process_steals(
                                    StealType::Third,
                                    app.game_state.clone().unwrap(),
                                    app.debug_settings.debug_roll_state.clone(),
                                    &catcher,
                                ));
                            }
                            if steal4 && ui.button("Steal Home").clicked() {
                                app.game_state = Some(process_steals(
                                    StealType::Home,
                                    app.game_state.clone().unwrap(),
                                    app.debug_settings.debug_roll_state.clone(),
                                    &catcher,
                                ));
                            }
                            if double_steal && ui.button("Double Steal").clicked() {
                                app.game_state = Some(process_steals(
                                    StealType::Double,
                                    app.game_state.clone().unwrap(),
                                    app.debug_settings.debug_roll_state.clone(),
                                    &catcher,
                                ));
                            }
                            if !steal2 && !steal3 && !steal4 && !double_steal {
                                toasts.add(Toast {
                                    kind: ToastKind::Info,
                                    text: "No runners on base.".into(),
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                });
                            }
                        } else {
                            // NOTE: I think the fact that these are in a menu means a bunch get
                            // spammed, going to have to do something different
                            toasts.add(Toast {
                                kind: ToastKind::Info,
                                text: "No active game.".into(),
                                options: ToastOptions::default()
                                    .duration_in_seconds(3.0)
                                    .show_progress(true)
                                    .show_icon(true),
                            });
                        }
                    });
                    if ui.button("Bunt").clicked() {
                        if app.game_state.is_some() && app.game_modern.is_some() {
                            // TODO: check and make sure base runners make sense
                            if app.game_state.as_ref().unwrap().runners == RunnersOn::Runner000 {
                                toasts.add(Toast {
                                    kind: ToastKind::Info,
                                    text: "No runners on, why bunt?".into(),
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                });
                            }
                            let batter = match app.game_state.as_ref().unwrap().inning_half {
                                InningTB::Top => {
                                    let bat_num =
                                        app.game_state.as_ref().unwrap().away_state.current_batter;
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [bat_num as usize]
                                        .clone()
                                }
                                InningTB::Bottom => {
                                    let bat_num =
                                        app.game_state.as_ref().unwrap().home_state.current_batter;
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [bat_num as usize]
                                        .clone()
                                }
                            };
                            app.game_state = Some(bunt(
                                app.game_state.clone().unwrap(),
                                app.game_modern.as_ref().unwrap(),
                                app.debug_settings.debug_roll_state.clone(),
                                batter,
                            ));
                        } else {
                            toasts.add(Toast {
                                kind: ToastKind::Info,
                                text: "No active game.".into(),
                                options: ToastOptions::default()
                                    .duration_in_seconds(3.0)
                                    .show_progress(true)
                                    .show_icon(true),
                            });
                        }
                    }
                    if ui.button("Hit & Run").clicked() {
                        if app.game_state.is_some() && app.game_modern.is_some() {
                            if app.game_state.as_ref().unwrap().runners == RunnersOn::Runner100 {
                                let batter: Player;
                                match app.game_state.as_ref().unwrap().inning_half {
                                    InningTB::Top => {
                                        let bat_num = app
                                            .game_state
                                            .as_ref()
                                            .unwrap()
                                            .away_state
                                            .current_batter;
                                        batter = app
                                            .game_modern
                                            .as_ref()
                                            .unwrap()
                                            .away_active
                                            .batting_order
                                            [bat_num as usize]
                                            .clone();
                                    }
                                    InningTB::Bottom => {
                                        let bat_num = app
                                            .game_state
                                            .as_ref()
                                            .unwrap()
                                            .home_state
                                            .current_batter;
                                        batter = app
                                            .game_modern
                                            .as_ref()
                                            .unwrap()
                                            .home_active
                                            .batting_order
                                            [bat_num as usize]
                                            .clone();
                                    }
                                }
                                app.game_state = Some(hit_and_run(
                                    app.game_state.clone().unwrap(),
                                    app.game_modern.as_ref().unwrap(),
                                    &mut app.debug_settings.debug_roll_state,
                                    batter,
                                ));
                            } else {
                                toasts.add(Toast {
                                    kind: ToastKind::Info,
                                    text: "Hit and run only available with a runner on first."
                                        .into(),
                                    options: ToastOptions::default()
                                        .duration_in_seconds(3.0)
                                        .show_progress(true)
                                        .show_icon(true),
                                });
                            }
                        } else {
                            toasts.add(Toast {
                                kind: ToastKind::Info,
                                text: "No active game.".into(),
                                options: ToastOptions::default()
                                    .duration_in_seconds(3.0)
                                    .show_progress(true)
                                    .show_icon(true),
                            });
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
                        app.gui_windows.debug_window = true;
                        app.debug_settings.debug_copied = false;
                    }
                    if ui.button("Roll").clicked() {
                        app.gui_windows.debug_roll_window = true;
                    }
                    if ui.button("Console").clicked() {
                        app.gui_windows.console_window = true;
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
            // TODO: probably should switch to batting order
            let away_team = app.away_team.as_ref().unwrap();
            app.away_team_name = away_team.name.to_string();
            let batter1 = &app.game_modern.clone().unwrap().away_active.roster[0];
            app.away_bo.batter1 = format!("{} {}", &batter1.first_name, &batter1.last_name);
            let batter2 = &app.game_modern.clone().unwrap().away_active.roster[1];
            app.away_bo.batter2 = format!("{} {}", &batter2.first_name, &batter2.last_name);
            let batter3 = &app.game_modern.clone().unwrap().away_active.roster[2];
            app.away_bo.batter3 = format!("{} {}", &batter3.first_name, &batter3.last_name);
            let batter4 = &app.game_modern.clone().unwrap().away_active.roster[3];
            app.away_bo.batter4 = format!("{} {}", &batter4.first_name, &batter4.last_name);
            let batter5 = &app.game_modern.clone().unwrap().away_active.roster[4];
            app.away_bo.batter5 = format!("{} {}", &batter5.first_name, &batter5.last_name);
            let batter6 = &app.game_modern.clone().unwrap().away_active.roster[5];
            app.away_bo.batter6 = format!("{} {}", &batter6.first_name, &batter6.last_name);
            let batter7 = &app.game_modern.clone().unwrap().away_active.roster[6];
            app.away_bo.batter7 = format!("{} {}", &batter7.first_name, &batter7.last_name);
            let batter8 = &app.game_modern.clone().unwrap().away_active.roster[7];
            app.away_bo.batter8 = format!("{} {}", &batter8.first_name, &batter8.last_name);
            let batter9 = &app.game_modern.clone().unwrap().away_active.pitching[0];
            app.away_bo.batter9 = format!("{} {}", &batter9.first_name, &batter9.last_name);
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
        }
        let mut away_at_bat = 1;
        if app.game_state.is_some() {
            away_at_bat = app.game_state.clone().unwrap().away_state.current_batter + 1;
            // array indexing :/
        }
        ui.horizontal(|ui| {
            if away_at_bat == 1 {
                ui.label(RichText::new("1. ").strong());
                ui.label(RichText::new(app.away_bo.batter1.clone()).strong())
                    .on_hover_text(away_info1);
            // TODO: figure out a way to put baseball icon to indicate current batter
            } else {
                ui.label("1. ");
                ui.label(&app.away_bo.batter1).on_hover_text(away_info1);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 2 {
                ui.label(RichText::new("2. ").strong());
                ui.label(RichText::new(app.away_bo.batter2.clone()).strong())
                    .on_hover_text(away_info2);
            } else {
                ui.label("2. ");
                ui.label(&app.away_bo.batter2).on_hover_text(away_info2);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 3 {
                ui.label(RichText::new("3. ").strong());
                ui.label(RichText::new(app.away_bo.batter3.clone()).strong())
                    .on_hover_text(away_info3);
            } else {
                ui.label("3. ");
                ui.label(&app.away_bo.batter3).on_hover_text(away_info3);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 4 {
                ui.label(RichText::new("4. ").strong());
                ui.label(RichText::new(app.away_bo.batter4.clone()).strong())
                    .on_hover_text(away_info4);
            } else {
                ui.label("4. ");
                ui.label(&app.away_bo.batter4).on_hover_text(away_info4);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 5 {
                ui.label(RichText::new("5. ").strong());
                ui.label(RichText::new(app.away_bo.batter5.clone()).strong())
                    .on_hover_text(away_info5);
            } else {
                ui.label("5. ");
                ui.label(&app.away_bo.batter5).on_hover_text(away_info5);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 6 {
                ui.label(RichText::new("6. ").strong());
                ui.label(RichText::new(app.away_bo.batter6.clone()).strong())
                    .on_hover_text(away_info6);
            } else {
                ui.label("6. ");
                ui.label(&app.away_bo.batter6).on_hover_text(away_info6);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 7 {
                ui.label(RichText::new("7. ").strong());
                ui.label(RichText::new(app.away_bo.batter7.clone()).strong())
                    .on_hover_text(away_info7);
            } else {
                ui.label("7. ");
                ui.label(&app.away_bo.batter7).on_hover_text(away_info7);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 8 {
                ui.label(RichText::new("8. ").strong());
                ui.label(RichText::new(app.away_bo.batter8.clone()).strong())
                    .on_hover_text(away_info8);
            } else {
                ui.label("8. ");
                ui.label(&app.away_bo.batter8).on_hover_text(away_info8);
            }
        });
        ui.horizontal(|ui| {
            if away_at_bat == 9 {
                ui.label(RichText::new("9. ").strong());
                ui.label(RichText::new(app.away_bo.batter9.clone()).strong())
                    .on_hover_text(away_info9);
            } else {
                ui.label("9. ");
                ui.label(&app.away_bo.batter9).on_hover_text(away_info9);
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
            app.home_bo.batter1 = format!("{} {}", &batter1.first_name, &batter1.last_name);
            let batter2 = &app.game_modern.clone().unwrap().home_active.roster[1];
            app.home_bo.batter2 = format!("{} {}", &batter2.first_name, &batter2.last_name);
            let batter3 = &app.game_modern.clone().unwrap().home_active.roster[2];
            app.home_bo.batter3 = format!("{} {}", &batter3.first_name, &batter3.last_name);
            let batter4 = &app.game_modern.clone().unwrap().home_active.roster[3];
            app.home_bo.batter4 = format!("{} {}", &batter4.first_name, &batter4.last_name);
            let batter5 = &app.game_modern.clone().unwrap().home_active.roster[4];
            app.home_bo.batter5 = format!("{} {}", &batter5.first_name, &batter5.last_name);
            let batter6 = &app.game_modern.clone().unwrap().home_active.roster[5];
            app.home_bo.batter6 = format!("{} {}", &batter6.first_name, &batter6.last_name);
            let batter7 = &app.game_modern.clone().unwrap().home_active.roster[6];
            app.home_bo.batter7 = format!("{} {}", &batter7.first_name, &batter7.last_name);
            let batter8 = &app.game_modern.clone().unwrap().home_active.roster[7];
            app.home_bo.batter8 = format!("{} {}", &batter8.first_name, &batter8.last_name);
            let batter9 = &app.game_modern.clone().unwrap().home_active.pitching[0];
            app.home_bo.batter9 = format!("{} {}", &batter9.first_name, &batter9.last_name);
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
            home_at_bat = app.game_state.clone().unwrap().home_state.current_batter + 1;
            // array indexing :/
        }
        ui.horizontal(|ui| {
            if home_at_bat == 1 {
                ui.label(RichText::new("1. ").strong());
                ui.label(RichText::new(app.home_bo.batter1.clone()).strong())
                    .on_hover_text(home_info1);
            } else {
                ui.label("1. ");
                ui.label(&app.home_bo.batter1).on_hover_text(home_info1);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 2 {
                ui.label(RichText::new("2. ").strong());
                ui.label(RichText::new(app.home_bo.batter2.clone()).strong())
                    .on_hover_text(home_info2);
            } else {
                ui.label("2. ");
                ui.label(&app.home_bo.batter2).on_hover_text(home_info2);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 3 {
                ui.label(RichText::new("3. ").strong());
                ui.label(RichText::new(app.home_bo.batter3.clone()).strong())
                    .on_hover_text(home_info3);
            } else {
                ui.label("3. ");
                ui.label(&app.home_bo.batter3).on_hover_text(home_info3);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 4 {
                ui.label(RichText::new("4. ").strong());
                ui.label(RichText::new(app.home_bo.batter4.clone()).strong())
                    .on_hover_text(home_info4);
            } else {
                ui.label("4. ");
                ui.label(&app.home_bo.batter4).on_hover_text(home_info4);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 5 {
                ui.label(RichText::new("5. ").strong());
                ui.label(RichText::new(app.home_bo.batter5.clone()).strong())
                    .on_hover_text(home_info5);
            } else {
                ui.label("5. ");
                ui.label(&app.home_bo.batter5).on_hover_text(home_info5);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 6 {
                ui.label(RichText::new("6. ").strong());
                ui.label(RichText::new(app.home_bo.batter6.clone()).strong())
                    .on_hover_text(home_info6);
            } else {
                ui.label("6. ");
                ui.label(&app.home_bo.batter6).on_hover_text(home_info6);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 7 {
                ui.label(RichText::new("7. ").strong());
                ui.label(RichText::new(app.home_bo.batter7.clone()).strong())
                    .on_hover_text(home_info7);
            } else {
                ui.label("7. ");
                ui.label(&app.home_bo.batter7).on_hover_text(home_info7);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 8 {
                ui.label(RichText::new("8. ").strong());
                ui.label(RichText::new(app.home_bo.batter8.clone()).strong())
                    .on_hover_text(home_info8);
            } else {
                ui.label("8. ");
                ui.label(&app.home_bo.batter8).on_hover_text(home_info8);
            }
        });
        ui.horizontal(|ui| {
            if home_at_bat == 9 {
                ui.label(RichText::new("9. ").strong());
                ui.label(RichText::new(app.home_bo.batter9.clone()).strong())
                    .on_hover_text(home_info9);
            } else {
                ui.label("9. ");
                ui.label(&app.home_bo.batter9).on_hover_text(home_info9);
            }
        });
    });
}
