use crate::components::card::Card;

use legion::*;
use std::collections::HashMap;

use rand::Rng;

pub struct HeroLoader {
    // Has nothing
}

impl HeroLoader {
    pub fn load(world: &mut World, pick_map_out: &mut HashMap<String, String>) {
        let mut query = <&Card>::query();
        let mut heroes: Vec<&Card> = Vec::new();

        for card in query.iter(world) {
            if card.is_cur_hero {
                // TODO :: race ban check logic
                let bans = ["PIRATE", "DEMON", "DRAGON"];
                let hero_race_map: HashMap<&str, Vec<&String>> = HashMap::new();
                let mut is_banned = false;
                for ban in bans {
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
        for i in 1..9 {
            let random = rand::thread_rng().gen_range(1..101);
            let mut picks: Vec<&Card> = Vec::new();
            for j in 1..5 {
                let max = heroes.iter().count();
                let index = (random * j) % max;
                picks.push(heroes.remove(index));
            }
            let key = format!("Player{}", i);
            pick_map.insert(key, picks);
        }

        /***************** */
        for v in pick_map {
            println!("{}", v.0);
            let mut index = 1;
            for i in v.1.iter() {
                println!("{}", i.name);
                let key = format!("{} - {}", v.0, index);
                index += 1;
                pick_map_out.insert(key, i.name.to_string());
            }
            print!("\n");
        }
        /***************** */
        // TODO :: pick and push
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
        HeroLoader::load(&mut world, &mut pick_map);

        let mut query = <&Card>::query();

        let mut hero_count = 0;
        for card in query.iter(&world) {
            if card.is_cur_hero {
                hero_count += 1;
            }
        }
        assert_eq!(hero_count, 64);
        assert_eq!(pick_map.iter().count(), 32);
    }

    #[test]
    fn redundant_heroes() {
        use crate::components::card::Card;
        use crate::loaders::card_loader::CardLoader;
        use crate::loaders::hero_loader::HeroLoader;
        use legion::*;

        use std::collections::HashMap;

        let mut world = World::default();
        let mut pick_map: HashMap<String, String> = HashMap::new();

        CardLoader::load(&mut world);
        HeroLoader::load(&mut world, &mut pick_map);

        let mut heroes: Vec<&String> = Vec::new();
        for i in 1..9 {
            for j in 1..5 {
                let key = format!("Player{} - {}", i, j);
                let card = pick_map.get(&key).expect("no card");
                heroes.push(card);
            }
        }
        for i in 0..heroes.iter().count() {
            for j in i + 1..heroes.iter().count() {
                assert_ne!(heroes[i], heroes[j]);
            }
        }
    }

    #[test]
    fn banned_heroes() {
        // TODO
    }
}
