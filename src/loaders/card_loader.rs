use crate::components::card::Card;
use crate::enums::{card_set::CardSet, card_type::CardType, game_tag::GameTag, race::Race};

use legion::*;
use serde_json::Value;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Display, Path, PathBuf};
use std::str::FromStr;

pub struct CardLoader {
    // Has nothing
}

impl CardLoader {
    pub fn load(world: &mut World) {
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

            let id = card["id"].as_str().unwrap();
            let dbf_id = if card["dbfID"].is_null() {
                0
            } else {
                card["dbfID"].as_i64().unwrap() as i32
            };
            let normal_dbf_id = if card["battlegroundsNormalDbfId"].is_null() {
                0
            } else {
                card["battlegroundsNormalDbfId"].as_i64().unwrap() as i32
            };
            let premium_dbf_id = if card["battlegroundsPremiumDbfId"].is_null() {
                0
            } else {
                card["battlegroundsPremiumDbfId"].as_i64().unwrap() as i32
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
                card["techLevel"].as_i64().unwrap() as i32
            };
            let attack = if card["attack"].is_null() {
                0
            } else {
                card["attack"].as_i64().unwrap() as i32
            };
            let health = if card["health"].is_null() {
                0
            } else {
                card["health"].as_i64().unwrap() as i32
            };

            let mut game_tags = HashMap::new();

            if card["mechanics"].is_array() {
                for mechanic in card["mechanics"].as_array().unwrap().iter() {
                    let game_tag = GameTag::from_str(mechanic.as_str().unwrap()).unwrap();
                    game_tags.insert(game_tag, 1);
                }
            }

            let _ = world.push((Card {
                id: String::from(id),
                dbf_id,
                normal_dbf_id,
                premium_dbf_id,
                name: String::from(name),
                text: String::from(text),
                game_tags,
                is_cur_hero: is_battlegrounds_hero,
            },));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn number_of_cards() {
        use legion::*;
        use crate::components::card::Card;
        use crate::loaders::card_loader::CardLoader;

        let mut world = World::default();

        CardLoader::load(&mut world);

        let mut query = <&Card>::query();

        assert_eq!(query.iter(&world).count(), 13100);
    }
}
