//use deadball::characters::players::*;

/*
use deadball::characters::teams::*;
use deadball::core::file_locations::*;
use deadball::core::game_functions::{create_modern_game, init_new_game_state, modern_game_flow};

use std::fs;
*/

use eframe::{
    egui,
    epaint::{pos2, Color32},
};
use egui::{Rect, RichText};
use egui_extras::RetainedImage;

struct DeadballApp<'a> {
    heading1: &'a str,
    heading2: &'a str,
    heading3: &'a str,
    heading4: &'a str,
    diamond_image: RetainedImage,
    pitcher_label: &'a str,
    catcher_label: &'a str,
    firstbase_label: &'a str,
    secondbase_label: &'a str,
    shortstop_label: &'a str,
    thirdbase_label: &'a str,
    rightfield_label: &'a str,
    centerfield_label: &'a str,
    leftfield_label: &'a str,
}

impl Default for DeadballApp<'_> {
    fn default() -> Self {
        Self {
            heading1: "Left Panel",
            heading2: "Right Panel",
            heading3: "Central Panel",
            heading4: "Bottom Panel",
            diamond_image: RetainedImage::from_image_bytes(
                "baseball_diamond.png",
                include_bytes!("images/baseball_diamond.png"),
            )
            .unwrap(),
            pitcher_label: "Seth Loveall",
            catcher_label: "Seth Loveall",
            firstbase_label: "Seth Loveall",
            secondbase_label: "Seth Loveall",
            shortstop_label: "Seth Loveall",
            thirdbase_label: "Seth Loveall",
            rightfield_label: "Seth Loveall",
            centerfield_label: "Seth Loveall",
            leftfield_label: "Seth Loveall",
        }
    }
}

impl eframe::App for DeadballApp<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // GUI logic here
        // draw GUI here
        egui::TopBottomPanel::bottom("Control Panel").show(ctx, |ui| {
            ui.label(self.heading4);
        });
        egui::SidePanel::left("Away Team").show(ctx, |ui| {
            ui.label(self.heading1);
        });
        egui::SidePanel::right("Home Team").show(ctx, |ui| {
            ui.label(self.heading2);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.heading3);
            // draw baseball field and label players
            ui.add(egui::Image::new(
                self.diamond_image.texture_id(ctx),
                self.diamond_image.size_vec2() * 0.2,
            ));
            // put player names
            ui.put(
                Rect {
                    min: pos2(460.0, 240.0),
                    max: pos2(560.0, 260.0),
                },
                eframe::egui::Label::new(
                    RichText::new(self.firstbase_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE)
                        .size(16.0),
                ),
            );
            ui.put(
                Rect {
                    min: pos2(400.0, 180.0),
                    max: pos2(500.0, 200.0),
                },
                eframe::egui::Label::new(
                    RichText::new(self.secondbase_label)
                        .color(Color32::BLACK)
                        .strong()
                        .background_color(Color32::WHITE)
                        .size(16.0),
                ),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
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
