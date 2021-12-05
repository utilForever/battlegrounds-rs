use strum_macros::EnumString;

/// An enumerator for identifying the type of the card.
#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum CardType {
    INVALID,
    GAME,
    PLAYER,
    HERO,
    MINION,
    SPELL,
    ENCHANTMENT,
    WEAPON,
    ITEM,
    TOKEN,
    HERO_POWER,
    BLANK,
    GAME_MODE_BUTTON,
    MOVE_MINION_HOVER_TARGET,
    LETTUCE_ABILITY,
}
