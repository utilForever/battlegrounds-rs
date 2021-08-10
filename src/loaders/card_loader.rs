use serde_json::Value;
use specs::prelude::*;

use std::fs::File;
use std::io::Read;
use std::path::{Display, Path, PathBuf};

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
            if card["set"].as_str().unwrap() == "BATTLEGROUNDS"
                || !card["battlegroundsNormalDbfId"].is_null()
                || !card["battlegroundsPremiumDbfId"].is_null()
            {
                println!("{}", card["name"].as_str().unwrap());
            }
        }
    }
}
