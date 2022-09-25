use crate::enums::game_tag::GameTag;

use std::collections::HashMap;

pub struct Card {
    pub id: String,
    pub dbf_id: i32,
    pub normal_dbf_id: i32,
    pub premium_dbf_id: i32,

    pub name: String,
    pub text: String,

    pub game_tags: HashMap<GameTag, i32>,

    pub is_cur_hero: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Minion {
    pub attack: u64,
    pub battlegrounds_premium_dbf_id: u64,
    pub card_class: String,
    pub dbf_id: u32,
    pub health: u64,
    pub id: String,
    pub mechanics: Vec<String>,
    pub name: String,
    pub race: String,
    pub referenced_tags: Vec<String>,
    pub tech_level: u8,
}
