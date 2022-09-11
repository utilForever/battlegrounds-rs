use crate::components::card::Card;

use crate::enums::game_race::GameRace;
use legion::*;
use std::collections::HashMap;

use rand::Rng;

pub struct HeroLoader {
    // Has nothing
}

impl HeroLoader {
    pub fn load(world: &mut World) {
        let mut query = <&Card>::query();
        let mut heroes: Vec<&Card> = Vec::new();
        let mut rng = rand::thread_rng();
        const RACES: [&str; 8] = [
            "MURLOC",
            "DEMON",
            "MECHANICAL",
            "ELEMENTAL",
            "BEAST",
            "PIRATE",
            "DRAGON",
            "QUILBOAR",
        ];
        const BAN_COUNT: usize = 3;
        let random_indices = rand::seq::index::sample(&mut rng, RACES.len(), BAN_COUNT).into_vec();
        let mut bans: Vec<&str> = Vec::new();
        for index in random_indices {
            bans.push(RACES[index])
        }
        for card in query.iter(world) {
            if card.is_cur_hero {
                // TODO :: race ban check logic
                let hero_race_map: HashMap<&str, Vec<&String>> = HashMap::new();
                let mut is_banned = false;
                for ban in &bans {
                    match hero_race_map.get(ban) {
                        Some(vec) => {
                            if vec.contains(&&card.name) {
                                is_banned = true;
                            }
                        }
                        None => {
                            panic!("Hero Ban Map is Incorrect!")
                        }
                    }
                }
                if is_banned == false {
                    heroes.push(card);
                }
            }
        }

        let mut pick_map: HashMap<String, Vec<&Card>> = HashMap::new();
        let mut rng = rand::thread_rng();
        const PLAYERS: usize = 8;
        const AVAILABLE_CHOICES: usize = 4;
        let random_indices =
            rand::seq::index::sample(&mut rng, heroes.len(), PLAYERS * AVAILABLE_CHOICES)
                .into_vec();
        for i in 0..8 {
            let mut picks: Vec<&Card> = Vec::new();
            for j in 0..4 {
                match heroes.get(random_indices[i * AVAILABLE_CHOICES + j]) {
                    Some(pick) => picks.push(pick),
                    None => {
                        panic!("Some kinds of pick error")
                    }
                }
            }
            let key = format!("Player{}", i + 1);
            pick_map.insert(key, picks);
        }

        // world.push(pick_map);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn number_of_heroes() {
        use crate::components::card::Card;
        use crate::loaders::card_loader::CardLoader;
        use crate::loaders::hero_loader::HeroLoader;
        use legion::*;

        use std::collections::HashMap;

        let mut world = World::default();
        let mut pick_map: HashMap<String, String> = HashMap::new();

        CardLoader::load(&mut world);
        HeroLoader::load(&mut world);

        let mut query = <&Card>::query();

        let mut hero_count = 0;
        for card in query.iter(&world) {
            if card.is_cur_hero {
                hero_count += 1;
            }
        }
        assert_eq!(hero_count, 64);
        assert_eq!(pick_map.len(), 32);
    }

    #[test]
    fn redundant_heroes() {
        use crate::loaders::card_loader::CardLoader;
        use crate::loaders::hero_loader::HeroLoader;
        use legion::*;

        use std::collections::HashMap;

        let mut world = World::default();
        let mut pick_map: HashMap<String, String> = HashMap::new();

        CardLoader::load(&mut world);
        HeroLoader::load(&mut world);

        let mut heroes: Vec<&String> = Vec::new();
        for i in 1..9 {
            for j in 1..5 {
                let key = format!("Player{} - {}", i, j);
                let card = pick_map.get(&key).expect("no card");
                heroes.push(card);
            }
        }
        for i in 0..heroes.len() {
            for j in i + 1..heroes.len() {
                assert_ne!(heroes[i], heroes[j]);
            }
        }
    }

    #[test]
    fn banned_heroes() {
        // TODO
    }
}
