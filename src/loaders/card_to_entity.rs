mod json_reader;
mod components;

use legion::*;
use crate::json_reader::*;
use crate::components::card::Minion;


fn main() {

    let cards = card_container();
    let mut world = World::default();

    for card in cards {
        let attack = if card.attack.is_some() {
            card.attack.unwrap()
        } else{
            0
        };
        let battlegrounds_premium_dbf_id = card.battlegrounds_premium_dbf_id.unwrap();
        let card_class = if card.card_class.is_none() {
            ""
        } else {
            card.card_class.as_deref().unwrap()
        };
        let dbf_id = card.dbf_id;
        let health = if card.health.is_some() { 
            card.health.unwrap()
         } else {
            0
        };
        let id = card.id;
        let mechanics = if card.mechanics.is_none() {
            Vec::new()
        } else {
            card.mechanics.unwrap()
        };
        let name = card.name;
        let race = if card.race.is_some() {
            card.race.as_deref().unwrap()
        } else {
            ""
        };
        let referenced_tags = if card.referenced_tags.is_none() {
            Vec::new()
        } else {
            card.referenced_tags.unwrap()
        };
        let tech_level = if card.tech_level.is_some() {
            card.tech_level.unwrap()
        } else {
            0
        };

        let _ = world.push(
            (
                Minion {
                    attack,
                    battlegrounds_premium_dbf_id,
                    card_class: String::from(card_class),
                    dbf_id,
                    health,
                    id: String::from(id),
                    mechanics,
                    name: String::from(name),
                    race: String::from(race),
                    referenced_tags,
                    tech_level
                },
            )
        );

    }

    let mut query = <&Minion>::query();
    for minion in query.iter(&world) {
        println!("{:?}", minion.name);
    }
}

