use crate::enums::{card_set::CardSet, card_type::CardType, game_tag::GameTag, race::Race};

use serde_json::Value;
use specs::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path, PathBuf};
use std::str::FromStr;

pub struct CardLoader {
    // Has nothing
}

impl CardLoader {
    pub fn load(world: &World) {
        let mut res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        res_path.push("res/cards.json");

        // Create a path to the desired file
        let path: &Path = Path::new(res_path.as_path());
        let display: Display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file: File = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut str = String::new();
        if let Err(why) = file.read_to_string(&mut str) {
            panic!("couldn't read {}: {}", display, why)
        }

        // Deserialize an value from a string of JSON text
        let json_str: Value = match serde_json::from_str(&str) {
            Err(why) => panic!("couldn't parse cards.json: {}", why),
            Ok(s) => s,
        };

        // Parse card data from an value
        let cards: &Vec<Value> = json_str.as_array().unwrap();
        for card in cards {
            let card_set = if card["set"].is_null() {
                1
            } else {
                CardSet::from_str(card["set"].as_str().unwrap()).unwrap() as i32
            };

            if card_set == CardSet::LETTUCE as i32 {
                continue;
            }

            let id = card["id"].as_str();
            let dbf_id = if card["dbfID"].is_null() {
                0
            } else {
                FromStr::from_str(card["dbfID"].as_str().unwrap()).unwrap()
            };
            let normal_dbf_id = if card["battlegroundsNormalDbfId"].is_null() {
                0
            } else {
                FromStr::from_str(card["battlegroundsNormalDbfId"].as_str().unwrap()).unwrap()
            };
            let premium_dbf_id = if card["battlegroundsPremiumDbfId"].is_null() {
                0
            } else {
                FromStr::from_str(card["battlegroundsPremiumDbfId"].as_str().unwrap()).unwrap()
            };
            let is_battlegrounds_hero = !card["battlegroundsHero"].is_null();

            let name = if card["name"].is_null() {
                ""
            } else {
                card["name"].as_str().unwrap()
            };
            let text = if card["text"].is_null() {
                ""
            } else {
                card["text"].as_str().unwrap()
            };

            let card_type = if card["type"].is_null() {
                CardType::INVALID
            } else {
                CardType::from_str(card["type"].as_str().unwrap()).unwrap()
            };
            let race = if card["race"].is_null() {
                Race::INVALID
            } else {
                Race::from_str(card["race"].as_str().unwrap()).unwrap()
            };

            let tech_level = if card["techLevel"].is_null() {
                0
            } else {
                FromStr::from_str(card["techLevel"].as_str().unwrap()).unwrap()
            };
            let attack = if card["attack"].is_null() {
                0
            } else {
                FromStr::from_str(card["attack"].as_str().unwrap()).unwrap()
            };
            let health = if card["health"].is_null() {
                0
            } else {
                FromStr::from_str(card["health"].as_str().unwrap()).unwrap()
            };

            let mut game_tags = HashMap::new();
            for mechanic in card["mechanics"].as_array().unwrap().iter() {
                let game_tag = GameTag::from_str(mechanic.as_str().unwrap()).unwrap();
                game_tags.insert(game_tag, 1);
            }
        }
    }
}
