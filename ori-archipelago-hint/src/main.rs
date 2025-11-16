#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::fs;
use serde::{Deserialize, Serialize};
use regex::Regex;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([750.0, 350.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ori Archipelago Hint",
        options,
        Box::new(|cc|{
            Ok(Box::<ArchipelagoWorld>::default())
        }),
    )
}

#[derive(Serialize, Deserialize)]
struct GamesLocation {
    game_name: String,
    areas: Vec<AreaLocation>
}

#[derive(Serialize, Deserialize)]
struct AreaLocation {
    area_name: String,
    locations: Vec<String>
}

struct Player {
    player_name: String,
    game_name: String
}

struct Hint {
    internal_name: String,
    displayed_name: String,
    location: String,
    unlocked: bool
}

struct GroupHint {
    button_label: String,
    unlock_all: bool,
    hints: Vec<Hint>
}

struct GameHint {
    game_name: String,
    player_name: String,
    groups_hints: Vec<GroupHint>
}

struct ArchipelagoWorld {
    games: Vec<GameHint>,
    spoiler_path: String
}

impl Default for ArchipelagoWorld {
    fn default() -> Self {
        let mut r = ArchipelagoWorld {
            games: Vec::new(),
            spoiler_path: "".to_owned()
        };

        return r;
    }
}

fn readSpoiler(spoiler_path: &str, world: &mut ArchipelagoWorld){
    world.games = Vec::new();
    world.spoiler_path = spoiler_path.to_owned();

    let json_localisation: Vec<GamesLocation> = serde_json::from_str(&fs::read_to_string("locations.json").expect("Should be able to read locations.json")).unwrap();

    let mut player = "";
    let r_player = Regex::new("Player [0-9]*:.*").unwrap();

    for line in fs::read_to_string(spoiler_path).unwrap().lines(){
        if player != "" && line.contains("Game:") {
            let game = line.split(':').collect::<Vec<&str>>()[1].trim();
            world.games.push(GameHint{
                game_name: game.to_string(),
                player_name: player.to_string(),
                groups_hints: createHintsStructure(game)
            });
        }
        else if r_player.is_match(line){
            player = line.split(':').collect::<Vec<&str>>()[1].trim();
        }
        else{
            player = "";
        }

        if line.contains(": GinsoKey (") {
            let line_split = line.split(": GinsoKey (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "GinsoKey", &area_loc);
        }
        else if line.contains(": ForlornKey (") {
            let line_split = line.split(": ForlornKey (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "ForlornKey", &area_loc);
        }
        else if line.contains(": HoruKey (") {
            let line_split = line.split(": HoruKey (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "HoruKey", &area_loc);
        }
        else if line.contains(": Stomp (") {
            let line_split = line.split(": Stomp (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Stomp", &area_loc);
        }
        else if line.contains(": Grenade (") {
            let line_split = line.split(": Grenade (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Grenade", &area_loc);
        }
        else if line.contains(": Bash (") {
            let line_split = line.split(": Bash (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Bash", &area_loc);
        }
        else if line.contains(": Bow (") {
            let line_split = line.split(": Bow (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Bow", &area_loc);
        }
        else if line.contains(": Flap (") {
            let line_split = line.split(": Flap (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Flap", &area_loc);
        }
        else if line.contains(": Glide (") {
            let line_split = line.split(": Glide (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Glide", &area_loc);
        }
        else if line.contains(": Clean Water (") {
            let line_split = line.split(": Clean Water (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Clean Water", &area_loc);
        }
        else if line.contains(": Water Dash (") {
            let line_split = line.split(": Water Dash (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Water Dash", &area_loc);
        }
        else if line.contains(": Burrow (") {
            let line_split = line.split(": Burrow (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Burrow", &area_loc);
        }
        else if line.contains(": Flash (") {
            let line_split = line.split(": Flash (").collect::<Vec<&str>>();
            let for_player = line_split[1].trim().replace(")","");
            let area_loc = getAreaHint(&line_split, &world.games, &json_localisation);

            setHintLocation(&mut world.games, &for_player, "Flash", &area_loc);
        }
    }
}

fn setHintLocation(games: &mut Vec<GameHint>, for_player: &str, skill: &str, area_loc: &str){
    for game in games {
        if game.player_name == for_player{
            for gh in &mut game.groups_hints {
                for h in &mut gh.hints{
                    if h.internal_name == skill {
                        h.location = area_loc.to_owned();
                    }
                }
            }
        }
    }
}

fn getAreaHint(line_split: &Vec<&str>, games: &Vec<GameHint>, json_localisation: &Vec<GamesLocation>) -> String {
    let pickup = line_split[0].split(" (").collect::<Vec<&str>>();
    let pickup_location = pickup[0].trim();
    let pickup_player = pickup[1].trim().replace(")","");

    for game in games {
        if game.player_name == pickup_player{
            for g in json_localisation{
                if g.game_name == game.game_name {
                    for a in &g.areas {
                        if a.locations.contains(&pickup_location.to_string()) {
                            return a.area_name.to_owned() + " (" + &pickup_player + ")";
                        }
                    }
                    return g.game_name.to_owned() + " (" + &pickup_player + ")";
                }
            }
        }
    }
    return "Not found".to_string();
}

fn createHintsStructure(game: &str) -> Vec<GroupHint> {
    if game == "Ori and the Blind Forest" {
        return createBFHints();
    }
    else if game == "Ori and the Will of the Wisps" {
        return createWotWHints();
    }
    return Vec::new();
}

fn createBFHints() -> Vec<GroupHint> {
    let mut r: Vec<GroupHint> = Vec::new();

    let mut keys = GroupHint{
        button_label: "Keys hint".to_owned(),
        unlock_all: false,
        hints: Vec::new()
    };
    let mut forlorn = GroupHint{
        button_label: "Forlorn hint".to_owned(),
        unlock_all: true,
        hints: Vec::new()
    };

    keys.hints.push(Hint{
        internal_name: "GinsoKey".to_owned(),
        displayed_name: "Water Vein".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    keys.hints.push(Hint{
        internal_name: "ForlornKey".to_owned(),
        displayed_name: "Gummon Seal".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    keys.hints.push(Hint{
        internal_name: "HoruKey".to_owned(),
        displayed_name: "Sunstone".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });

    forlorn.hints.push(Hint{
        internal_name: "Stomp".to_owned(),
        displayed_name: "Stomp".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    forlorn.hints.push(Hint{
        internal_name: "Grenade".to_owned(),
        displayed_name: "Grenade".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });

    r.push(keys);
    r.push(forlorn);

    return r;
}

fn createWotWHints() -> Vec<GroupHint> {
    let mut r: Vec<GroupHint> = Vec::new();

    let mut twillen = GroupHint{
        button_label: "Twillen hint".to_owned(),
        unlock_all: true,
        hints: Vec::new()
    };
    let mut opher = GroupHint{
        button_label: "Opher hint".to_owned(),
        unlock_all: true,
        hints: Vec::new()
    };
    let mut lupo = GroupHint{
        button_label: "Lupo hint".to_owned(),
        unlock_all: true,
        hints: Vec::new()
    };

    twillen.hints.push(Hint{
        internal_name: "Bash".to_owned(),
        displayed_name: "Bash".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    twillen.hints.push(Hint{
        internal_name: "Bow".to_owned(),
        displayed_name: "Bow".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    
    opher.hints.push(Hint{
        internal_name: "Flap".to_owned(),
        displayed_name: "Flap".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    opher.hints.push(Hint{
        internal_name: "Glide".to_owned(),
        displayed_name: "Glide".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    opher.hints.push(Hint{
        internal_name: "Clean Water".to_owned(),
        displayed_name: "Clean Water".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });

    lupo.hints.push(Hint{
        internal_name: "Water Dash".to_owned(),
        displayed_name: "Water Dash".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    lupo.hints.push(Hint{
        internal_name: "Burrow".to_owned(),
        displayed_name: "Burrow".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    lupo.hints.push(Hint{
        internal_name: "Grenade".to_owned(),
        displayed_name: "Grenade".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });
    lupo.hints.push(Hint{
        internal_name: "Flash".to_owned(),
        displayed_name: "Flash".to_owned(),
        location: "Starting".to_owned(),
        unlocked: false
    });

    r.push(twillen);
    r.push(opher);
    r.push(lupo);

    return r;
}

fn getLocationSkill(h: &Hint) -> String {
    if h.unlocked {
        return h.location.clone();
    }
    return "?".to_owned();
}

impl eframe::App for ArchipelagoWorld {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui|{
                    ui.label("Select a spoiler file: ");
                    if ui.button("Open file…").clicked() && let Some(path) = rfd::FileDialog::new().pick_file()
                    {
                        let spoiler_path = &path.display().to_string();
                        readSpoiler(spoiler_path, self);
                    }
                    if self.spoiler_path != "" {
                        ui.label("Loded file ".to_owned() + &self.spoiler_path);
                    }
                });
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);

                for game in &mut self.games {
                    ui.horizontal(|ui| {
                        ui.heading(game.game_name.clone() + " (" + &game.player_name + ")");    
                        for group in &mut game.groups_hints {
                            if ui.button(group.button_label.clone()).clicked(){
                                if group.unlock_all {
                                    for hint in &mut group.hints{
                                        hint.unlocked = true;
                                    }
                                }
                                else{
                                    let mut hintLeft = false;
                                    for h in &group.hints{
                                        if !h.unlocked{
                                            hintLeft = true;
                                            break;
                                        } 
                                    }
                                    if hintLeft {
                                        let mut rng = ChaCha20Rng::from_os_rng();
                                        let mut i = rng.random_range(0..group.hints.len());
                                        while group.hints[i].unlocked{
                                            i = rng.random_range(0..group.hints.len());
                                        }
                                        group.hints[i].unlocked = true;
                                    }
                                }
                            }
                        }
                    });
                    ui.add_space(5.0);
                    for group in &game.groups_hints {
                        ui.horizontal(|ui| {
                            for h in &group.hints {
                                ui.vertical(|ui| {
                                    ui.label(&h.displayed_name);
                                    ui.label(format!("{}",getLocationSkill(h)));
                                });
                                ui.add_space(20.0);
                            }
                        });
                        ui.add_space(10.0);
                    }
                    ui.add_space(10.0);
                }
            });
        });
    }
}