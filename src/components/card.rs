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
