use std::fs;

use eframe::egui::{self, Color32, Context, RichText};
use egui_file::FileDialog;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

use crate::{
    characters::{
        ballparks::{
            generate_modern_ballpark, load_park_ancient, load_park_modern, write_ballpark_modern,
        },
        players::{generate_player, write_player, PlayerClass, Position},
        teams::{generate_team, load_team, write_team, Era},
    },
    core::game_functions::{create_modern_game, GameStatus, InningTB, Outs, RunnersOn},
    DeadballApp, ABOUT_APP, ABOUT_DEABALL,
};

/// populates ui for the version window
pub fn draw_version_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Version")
        .open(&mut app.gui_windows.version_window)
        .show(ctx, |ui| {
            ui.label("Version 0.3.5");
        });
}

/// populates ui for the "About Deadball Game" window
pub fn draw_about_deadball_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("About Deadball Game")
        .open(&mut app.gui_windows.about_deadball_window)
        .show(ctx, |ui| {
            ui.label(ABOUT_DEABALL);
            ui.hyperlink("http://wmakers.net/deadball");
        });
}

/// populates ui for the "About this app" window
pub fn draw_about_app_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("About this app")
        .open(&mut app.gui_windows.about_app_window)
        .show(ctx, |ui| {
            ui.label(ABOUT_APP);
        });
}

/// draws the console for displaying game text
pub fn draw_console_window(ctx: &Context, app: &mut DeadballApp) {
    let mut console_text: String;
    if app.game_state.is_some() {
        console_text = app.game_state.clone().unwrap().game_text;
    } else {
        console_text = "No game is currently active.".to_string();
    }
    egui::Window::new("Console")
        .open(&mut app.gui_windows.console_window)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut console_text);
                });
        });
}

/// renders the new game window
pub fn draw_create_new_game(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
    egui::Window::new("Create new game")
        .open(&mut app.gui_windows.create_game_window)
        .show(ctx, |ui| {
            // selectable value for game era
            ui.horizontal(|ui| {
                ui.label("Era:");
                ui.selectable_value(&mut app.create_game_era, Era::None, "None");
                ui.selectable_value(&mut app.create_game_era, Era::Modern, "Modern");
                ui.selectable_value(&mut app.create_game_era, Era::Ancient, "Ancient");
            });
            // selectable value for oddities
            ui.horizontal(|ui| {
                ui.label("Oddities:");
                ui.selectable_value(&mut app.oddity, false, "Disabled");
                ui.selectable_value(&mut app.oddity, true, "Enabled");
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
                            app.away_team_file = Some(file.to_path_buf());
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
                            app.home_team_file = Some(file.to_path_buf());
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
                            app.ballpark_file = Some(file.to_path_buf());
                        }
                    }
                }
            });
            ui.separator();
            // button to create game and return to main screen
            if ui.button("Create").clicked() {
                app.create_game_error = "".to_owned();
                // check and make sure options are set properly
                if app.away_team_file.is_some()
                    && app.home_team_file.is_some()
                    && app.ballpark_file.is_some()
                {
                    // try to load teams and ballpark files
                    match fs::read_to_string(app.away_team_file.as_ref().unwrap().as_path()) {
                        Ok(contents) => {
                            app.away_team = Some(load_team(contents));
                        }
                        Err(err) => {
                            app.create_game_error = app.create_game_error.clone()
                                + "Failed to read Away team file."
                                + &format!("{:?}", err);
                        }
                    }
                    match fs::read_to_string(app.home_team_file.as_ref().unwrap().as_path()) {
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
                            match fs::read_to_string(app.ballpark_file.as_ref().unwrap().as_path())
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
                            match fs::read_to_string(app.ballpark_file.as_ref().unwrap().as_path())
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
                    toasts.add(Toast {
                        text: "Game created.".into(),
                        kind: ToastKind::Info,
                        options: ToastOptions::default()
                            .duration_in_seconds(3.0)
                            .show_progress(true)
                            .show_icon(true),
                    });
                } else {
                    // update error message and display error window
                    if app.away_team_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbt file for away team.\n";
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Must select a *.dbt file for away team.".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                    if app.home_team_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbt file for home team.\n";
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Must select a *.dbt file for home team.".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                    if app.ballpark_file.is_none() {
                        app.create_game_error = app.create_game_error.clone()
                            + "Must select a *.dbb file for ballpark.\n";
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Must select a *.dbb file for ballpark.".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
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
                                app.oddity,
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

/// draws and handles logic for "Create Team" window
pub fn draw_create_team_window(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
    egui::Window::new("Create New Team")
        .open(&mut app.create_team.is_visible)
        .show(ctx, |ui| {
            ui.heading("New Team");
            // TODO: add Era selector
            ui.horizontal(|ui| {
                ui.label("Team Name: ");
                ui.text_edit_singleline(&mut app.create_team.name);
                ui.checkbox(&mut app.create_team.name_override, "override")
                    .on_hover_text("will generate random name if unchecked");
            });
            ui.horizontal(|ui| {
                ui.label("Location: ");
                ui.text_edit_singleline(&mut app.create_team.location);
                ui.checkbox(&mut app.create_team.location_override, "override")
                    .on_hover_text("will generate random location if unchecked");
            });
            ui.horizontal(|ui| {
                ui.label("Save location: ");
                ui.text_edit_singleline(&mut app.create_team.save_location);
            });
            if ui.button("Create").clicked() {
                // generate and write team
                let name = if app.create_team.name_override {
                    &app.create_team.name
                } else {
                    "New Team"
                };
                let new_team = generate_team(
                    app.create_team.era.clone(),
                    8,
                    4,
                    1,
                    5,
                    name,
                    &app.databases.first_names,
                    &app.databases.last_names,
                    &app.databases.logos,
                    &app.databases.mascots,
                    &app.databases.mottos,
                    &app.databases.personalities,
                    &app.databases.backgrounds,
                    &app.databases.park1,
                    &app.databases.park2,
                );
                match write_team(new_team, &app.create_team.save_location) {
                    Ok(()) => {
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Team created!".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                    Err(e) => {
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: format!("Create failed: {}", e).into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                }
            }
        });
}

/// draws and handles logic for "Create Player" Window
pub fn draw_create_player_window(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
    egui::Window::new("Create New Player")
        .open(&mut app.create_player.is_visible)
        .show(ctx, |ui| {
            ui.heading("New Player");
            // TODO: add Era selector
            ui.horizontal(|ui| {
                ui.label("First Name:");
                ui.text_edit_singleline(&mut app.create_player.first_name);
            });
            ui.horizontal(|ui| {
                ui.label("Nickname:");
                ui.text_edit_singleline(&mut app.create_player.nickname);
            });
            ui.horizontal(|ui| {
                ui.label("Last Name:");
                ui.text_edit_singleline(&mut app.create_player.last_name);
                ui.checkbox(&mut app.create_player.name_override, "override")
                    .on_hover_text("will generate random name if unchecked");
            });
            ui.horizontal(|ui| {
                ui.label("Class:");
                ui.selectable_value(
                    &mut app.create_player.class,
                    PlayerClass::StartingHitter,
                    "Batter",
                );
                ui.selectable_value(
                    &mut app.create_player.class,
                    PlayerClass::Pitchers,
                    "Pitcher",
                );
                ui.selectable_value(
                    &mut app.create_player.class,
                    PlayerClass::PinchHitter,
                    "Bench",
                );
            });
            egui::ComboBox::from_label("Position")
                .selected_text(format!("{:?}", &app.create_player.position))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.create_player.position, Position::Firstbase, "1B");
                    ui.selectable_value(
                        &mut app.create_player.position,
                        Position::Secondbase,
                        "2B",
                    );
                    ui.selectable_value(&mut app.create_player.position, Position::Shortstop, "SS");
                    ui.selectable_value(&mut app.create_player.position, Position::Thirdbase, "3B");
                    ui.selectable_value(&mut app.create_player.position, Position::Catcher, "C");
                    ui.selectable_value(&mut app.create_player.position, Position::Pitcher, "P");
                    ui.selectable_value(
                        &mut app.create_player.position,
                        Position::Rightfield,
                        "RF",
                    );
                    ui.selectable_value(
                        &mut app.create_player.position,
                        Position::Centerfield,
                        "CF",
                    );
                    ui.selectable_value(&mut app.create_player.position, Position::Leftfield, "LF");
                });
            ui.horizontal(|ui| {
                ui.label("Save location:");
                ui.text_edit_singleline(&mut app.create_player.save_location);
            });
            if ui.button("Create").clicked() {
                // TODO: need to handle nicknames
                let player = if app.create_player.name_override {
                    generate_player(
                        app.create_player.class.clone(),
                        app.create_player.position.clone(),
                        &[app.create_player.first_name.clone()],
                        &[app.create_player.last_name.clone()],
                    )
                } else {
                    generate_player(
                        app.create_player.class.clone(),
                        app.create_player.position.clone(),
                        &app.databases.first_names,
                        &app.databases.last_names,
                    )
                };
                match write_player(&player, &app.create_player.save_location) {
                    Ok(()) => {
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Player created!".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                    Err(e) => {
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: format!("Create failed: {}", e).into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                }
            }
        });
}

/// draws and handles logic for "Create Ballpark" window
pub fn draw_create_ballpark_window(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
    egui::Window::new("Create New Ballpark")
        .open(&mut app.create_ballpark.is_visible)
        .show(ctx, |ui| {
            ui.heading("New Ballpark");
            // TODO: add Era selector
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut app.create_ballpark.name);
                ui.checkbox(&mut app.create_ballpark.name_override, "override")
                    .on_hover_text("will generate random name if unchecked");
            });
            ui.horizontal(|ui| {
                ui.label("Save location:");
                ui.text_edit_singleline(&mut app.create_ballpark.save_location);
            });
            if ui.button("Create").clicked() {
                let ballpark = if app.create_ballpark.name_override {
                    generate_modern_ballpark(&[app.create_ballpark.name.clone()], &["".to_string()])
                } else {
                    generate_modern_ballpark(&app.databases.park1, &app.databases.park2)
                };
                match write_ballpark_modern(&ballpark, &app.create_ballpark.save_location) {
                    Ok(()) => {
                        toasts.add(Toast {
                            kind: ToastKind::Info,
                            text: "Ballpark created".into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true)
                                .show_icon(true),
                        });
                    }
                    Err(e) => {
                        toasts.add(Toast {
                            kind: ToastKind::Error,
                            text: format!("Create failed: {}", e).into(),
                            options: ToastOptions::default()
                                .duration_in_seconds(3.0)
                                .show_progress(true),
                        });
                    }
                }
            }
        });
}

/// draws the debug roll window
pub fn draw_debug_roll_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Roll Debug Mode")
        .open(&mut app.gui_windows.debug_roll_window)
        .show(ctx, |ui| {
            ui.checkbox(
                &mut app.debug_settings.debug_roll_state.mode,
                "Enable roll override.",
            )
            .on_hover_text("Check to replace rolls with predetermined values.");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut app.debug_settings.debug_roll_text);
                if ui
                    .button("Add")
                    .on_hover_text("Add value to roll list.")
                    .clicked()
                {
                    if app.debug_settings.debug_roll_state.rolls.len() == 1
                        && app.debug_settings.debug_roll_state.rolls[0] == 0
                    {
                        if let Ok(val) = app.debug_settings.debug_roll_text.parse::<i32>() {
                            app.debug_settings.debug_roll_state.rolls[0] = val;
                        }
                    } else if let Ok(val) = app.debug_settings.debug_roll_text.parse::<i32>() {
                        app.debug_settings.debug_roll_state.rolls.push(val);
                    }
                }
                if ui
                    .button("Clear")
                    .on_hover_text("Clear roll list.")
                    .clicked()
                {
                    app.debug_settings.debug_roll_state.rolls = vec![0];
                }
            });
            ui.horizontal(|ui| {
                ui.label("Rolls:");
                for roll in app.debug_settings.debug_roll_state.rolls.iter() {
                    ui.label(roll.to_string());
                }
            });
        });
}

/// draw the debug window
pub fn draw_debug_window(ctx: &Context, app: &mut DeadballApp) {
    egui::Window::new("Debug Mode")
        .open(&mut app.gui_windows.debug_window)
        .show(ctx, |ui| {
            if ui.button("Print Game State").clicked() {
                if app.game_state.is_some() {
                    println!("{:?}", app.game_state.as_ref().unwrap());
                } else {
                    println!("No active game state.");
                }
            }
            // set debug state to current game state (if it exists)
            if app.game_state.is_some() && !app.debug_settings.debug_copied {
                app.debug_settings.debug_state = app.game_state.clone().unwrap();
                app.debug_settings.debug_copied = true;
                app.debug_settings.debug_inning_text =
                    app.debug_settings.debug_state.inning.clone().to_string();
            }
            ui.horizontal(|ui| {
                ui.label("Game Status:");
                egui::ComboBox::from_label("Select status.")
                    .selected_text(&app.debug_settings.debug_game_state_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.status,
                            GameStatus::NotStarted,
                            "Not Started",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.status,
                            GameStatus::Ongoing,
                            "Ongoing",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.status,
                            GameStatus::Over,
                            "Over",
                        );
                    })
            });
            ui.horizontal(|ui| {
                ui.label("Inning:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_inning_text);
            });
            ui.horizontal(|ui| {
                ui.label("Inning Half:");
                egui::ComboBox::from_label("Select inning half.")
                    .selected_text(app.debug_settings.debug_inning_half_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.inning_half,
                            InningTB::Top,
                            "^",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.inning_half,
                            InningTB::Bottom,
                            "v",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Outs:");
                egui::ComboBox::from_label("Select outs.")
                    .selected_text(app.debug_settings.debug_outs_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.outs,
                            Outs::None,
                            "None",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.outs,
                            Outs::One,
                            "One",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.outs,
                            Outs::Two,
                            "Two",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.outs,
                            Outs::Three,
                            "Three",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Runners On:");
                egui::ComboBox::from_label("Select base runners.")
                    .selected_text(app.debug_settings.debug_runners_text.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner000,
                            "000",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner001,
                            "001",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner010,
                            "010",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner100,
                            "100",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner011,
                            "011",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner110,
                            "110",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner101,
                            "101",
                        );
                        ui.selectable_value(
                            &mut app.debug_settings.debug_state.runners,
                            RunnersOn::Runner111,
                            "111",
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Batting Team 1:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_batting1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Batting Team 2:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_batting2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Pitched Team 1:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_pitched1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Pitched Team 2:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_pitched2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Runs Team 1:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_runs1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Runs Team 2:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_runs2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Hits Team 1:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_hits1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Hits Team 2:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_hits2_text);
            });
            ui.horizontal(|ui| {
                ui.label("Errors Team 1:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_errors1_text);
            });
            ui.horizontal(|ui| {
                ui.label("Errors Team 2:");
                ui.text_edit_singleline(&mut app.debug_settings.debug_errors2_text);
            });
            // update debug game state combo box text
            match &app.debug_settings.debug_state.status {
                GameStatus::NotStarted => {
                    app.debug_settings.debug_game_state_text = "Not Started".to_string()
                }
                GameStatus::Ongoing => {
                    app.debug_settings.debug_game_state_text = "Ongoing".to_string()
                }
                GameStatus::Over => app.debug_settings.debug_game_state_text = "Over".to_string(),
            }
            // update inning half combo box text
            match &app.debug_settings.debug_state.inning_half {
                InningTB::Top => app.debug_settings.debug_inning_half_text = "^".to_string(),
                InningTB::Bottom => app.debug_settings.debug_inning_half_text = "v".to_string(),
            }
            // update outs combo box text
            match &app.debug_settings.debug_state.outs {
                Outs::None => app.debug_settings.debug_outs_text = "None".to_string(),
                Outs::One => app.debug_settings.debug_outs_text = "One".to_string(),
                Outs::Two => app.debug_settings.debug_outs_text = "Two".to_string(),
                Outs::Three => app.debug_settings.debug_outs_text = "Three".to_string(),
            }
            // update runners on text
            match &app.debug_settings.debug_state.runners {
                RunnersOn::Runner000 => app.debug_settings.debug_runners_text = "000".to_string(),
                RunnersOn::Runner001 => app.debug_settings.debug_runners_text = "001".to_string(),
                RunnersOn::Runner010 => app.debug_settings.debug_runners_text = "010".to_string(),
                RunnersOn::Runner100 => app.debug_settings.debug_runners_text = "100".to_string(),
                RunnersOn::Runner011 => app.debug_settings.debug_runners_text = "011".to_string(),
                RunnersOn::Runner110 => app.debug_settings.debug_runners_text = "110".to_string(),
                RunnersOn::Runner101 => app.debug_settings.debug_runners_text = "101".to_string(),
                RunnersOn::Runner111 => app.debug_settings.debug_runners_text = "111".to_string(),
            }
            // button to write changes to game state
            ui.separator();
            if ui.button("Write Changes").clicked() {
                // put players in the runner fields to avoid crashes
                let current_batter: i32;
                match app.debug_settings.debug_state.inning_half {
                    InningTB::Top => {
                        current_batter =
                            app.debug_settings.debug_state.away_state.current_batter as i32;
                        match app.debug_settings.debug_state.runners {
                            RunnersOn::Runner000 => {}
                            RunnersOn::Runner100 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [(current_batter - 1) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner010 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner001 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().away_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner3 = Some(
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
                                app.debug_settings.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner2 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter3 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().away_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                        }
                    }
                    InningTB::Bottom => {
                        current_batter =
                            app.debug_settings.debug_state.home_state.current_batter as i32;
                        match app.debug_settings.debug_state.runners {
                            RunnersOn::Runner000 => {}
                            RunnersOn::Runner100 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner1 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner010 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner2 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [(current_batter - 2) as usize]
                                            .clone(),
                                    );
                                }
                            }
                            RunnersOn::Runner001 => {
                                if current_batter == 1 {
                                    app.debug_settings.debug_state.runner3 = Some(
                                        app.game_modern.as_ref().unwrap().home_active.batting_order
                                            [8]
                                        .clone(),
                                    );
                                } else {
                                    app.debug_settings.debug_state.runner3 = Some(
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
                                app.debug_settings.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner2 = Some(
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
                                app.debug_settings.debug_state.runner3 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter3 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner2 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter2 - 1) as usize]
                                        .clone(),
                                );
                                app.debug_settings.debug_state.runner1 = Some(
                                    app.game_modern.as_ref().unwrap().home_active.batting_order
                                        [(batter1 - 1) as usize]
                                        .clone(),
                                );
                            }
                        }
                    }
                }
                app.game_state = Some(app.debug_settings.debug_state.clone());
            }
        });
}

/// renders the roster edit window to change lineup or current pitcher during game
pub fn draw_active_team_edit(ctx: &Context, app: &mut DeadballApp, toasts: &mut Toasts) {
    egui::Window::new("Edit Team")
        .open(&mut app.gui_windows.edit_roster_window)
        .show(ctx, |ui| {
            if app.game_state.is_some() {
                let team = if app.active_team_edit.is_home {
                    &app.game_modern.as_ref().unwrap().home_active
                } else {
                    &app.game_modern.as_ref().unwrap().away_active
                };
                if app.active_team_edit.is_batter {
                    // TODO: also display current game performance
                    // TODO: display streak/slump
                    ui.heading("Current Lineup");
                    for (i, player) in team.roster.iter().enumerate() {
                        ui.radio_value(
                            &mut (
                                &mut app.active_team_edit.current_select,
                                &mut app.active_team_edit.current_num,
                            ),
                            (&mut Some(player.clone()), &mut Some(i)),
                            format!(
                                "{} {} | {:?} | {} | {} | {:?}",
                                player.first_name,
                                player.last_name,
                                player.position,
                                player.batter_target,
                                player.on_base_target,
                                player.handedness
                            ),
                        );
                    }
                    ui.separator();
                    ui.heading("Bench");
                    for (i, player) in team.bench.iter().enumerate() {
                        ui.radio_value(
                            &mut (
                                &mut app.active_team_edit.bench_select,
                                &mut app.active_team_edit.bench_num,
                            ),
                            (&mut Some(player.clone()), &mut Some(i)),
                            format!(
                                "{} {} | {:?} | {} | {} | {:?}",
                                player.first_name,
                                player.last_name,
                                player.position,
                                player.batter_target,
                                player.on_base_target,
                                player.handedness
                            ),
                        );
                    }
                } else {
                    // TODO: show innings pitched, streak/slump, etc.
                    ui.heading("On the Mound");
                    let player = team.pitching[0].clone();
                    app.active_team_edit.current_num = Some(0);
                    ui.radio_value(
                        &mut app.active_team_edit.current_select,
                        Some(player.clone()),
                        format!(
                            "{} {} | {} | {:?}",
                            player.first_name,
                            player.last_name,
                            player.pitch_die,
                            player.handedness
                        ),
                    );
                    ui.separator();
                    ui.heading("Bullpen");
                    for (i, player) in team.bullpen.iter().enumerate() {
                        ui.radio_value(
                            &mut (
                                &mut app.active_team_edit.bench_select,
                                &mut app.active_team_edit.bench_num,
                            ),
                            (&mut Some(player.clone()), &mut Some(i)),
                            format!(
                                "{} {} | {} | {:?}",
                                player.first_name,
                                player.last_name,
                                player.pitch_die,
                                player.handedness
                            ),
                        );
                    }
                    ui.separator();
                    if ui.button("Swap").clicked() && app.active_team_edit.bench_select.is_some() {
                        if app.active_team_edit.is_home {
                            app.game_modern.as_ref().unwrap().home_active.pitching[0] =
                                app.active_team_edit.bench_select.clone().unwrap();
                        }
                        team.pitching[0] = app.active_team_edit.bench_select.unwrap();
                    }
                }
            }
        });
}
