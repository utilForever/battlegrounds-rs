use std::path::Path;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Card {
    artist: Option<String>,
    attack: Option<u64>,
    battlegrounds_premium_dbf_id: Option<u64>,
    card_class: Option<String>,
    collectible: Option<bool>,
    cost: Option<u8>,
    dbf_id: u32,
    flavor: Option<String>,
    health: Option<u16>,
    id: String,
    mechanics: Option<Vec<String>>,
    name: String,
    race: Option<String>,
    rarity: Option<String>,
    referenced_tags: Option<Vec<String>>,
    set: String,
    tech_level: Option<u8>,
    spell_school: Option<String>,
    text: Option<String>,
    r#type: String,
}

fn main() {
    let path = Path::new("res/cards_kor.json");
    let file = File::open(path).expect("file not found");
    let mut cards = Vec::new();

    let raw_cards: Vec<Card> = serde_json::from_reader(file).expect("Invalid format");
    
    for card in raw_cards.iter() {
        if card.battlegrounds_premium_dbf_id.is_some() {
            cards.push(card)
        }
    }
    println!("{}", cards.len());
}

