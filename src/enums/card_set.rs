use strum_macros::EnumString;

/// An enumerator for identifying the set of the card.
#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum CardSet {
    ALL,
    INVALID,
    TEST_TEMPORARY,
    BASIC,
    EXPERT1,
    HOF,
    MISSIONS,
    DEMO,
    NONE,
    CHEAT,
    BLANK,
    DEBUG_SP,
    PROMO,
    NAXX,
    GVG,
    BRM,
    TGT,
    CREDITS,
    HERO_SKINS,
    TB,
    SLUSH,
    LOE,
    OG,
    OG_RESERVE,
    KARA,
    KARA_RESERVE,
    GANGS,
    GANGS_RESERVE,
    UNGORO,
    ICECROWN,
    LOOTAPALOOZA,
    GILNEAS,
    BOOMSDAY,
    TROLL,
    DALARAN,
    ULDUM,
    DRAGONS,
    YEAR_OF_THE_DRAGON,
    BLACK_TEMPLE,
    WILD_EVENT,
    SCHOLOMANCE,
    BATTLEGROUNDS,
    DEMON_HUNTER_INITIATE,
    DARKMOON_FAIRE,
    THE_BARRENS,
    WAILING_CAVERNS,
    STORMWIND,
    LETTUCE,
    ALTERAC_VALLEY,
    LEGACY,
    CORE,
    VANILLA,
    TAVERNS_OF_TIME,
}
