use crate::enums::game_tag::GameTag;

use std::collections::HashMap;

struct Card {
    id: String,
    dbf_id: i32,
    normal_dbf_id: i32,
    premium_dbf_id: i32,

    name: String,
    text: String,

    game_tags: HashMap<GameTag, i32>,

    is_cur_hero: bool,
}
