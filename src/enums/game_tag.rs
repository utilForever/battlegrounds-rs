use strum_macros::EnumString;

/// An enumerator for identifying the game tag of the card.
#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, EnumString)]
pub enum GameTag {
    TAG_NOT_SET,
    TAG_SCRIPT_DATA_NUM_1,
    TAG_SCRIPT_DATA_NUM_2,
    TAG_SCRIPT_DATA_ENT_1,
    TAG_SCRIPT_DATA_ENT_2,
    MISSION_EVENT,
    TIMEOUT,
    TURN_START,
    TURN_TIMER_SLUSH,
    PREMIUM,
    GOLD_REWARD_STATE,
    PLAYSTATE,
    LAST_AFFECTED_BY,
    STEP,
    TURN,
    FATIGUE,
    CURRENT_PLAYER,
    FIRST_PLAYER,
    RESOURCES_USED,
    RESOURCES,
    HERO_ENTITY,
    MAXHANDSIZE,
    STARTHANDSIZE,
    PLAYER_ID,
    TEAM_ID,
    TRIGGER_VISUAL,
    RECENTLY_ARRIVED,
    PROTECTED,
    PROTECTING,
    DEFENDING,
    PROPOSED_DEFENDER,
    ATTACKING,
    PROPOSED_ATTACKER,
    ATTACHED,
    EXHAUSTED,
    DAMAGE,
    HEALTH,
    ATK,
    COST,
    ZONE,
    CONTROLLER,
    OWNER,
    DEFINITION,
    ENTITY_ID,
    HISTORY_PROXY,
    ELITE,
    MAXRESOURCES,
    CARD_SET,
    CARDTEXT,
    DURABILITY,
    SILENCED,
    WINDFURY,
    TAUNT,
    STEALTH,
    SPELLPOWER,
    DIVINE_SHIELD,
    CHARGE,
    NEXT_STEP,
    CLASS,
    CARDRACE,
    FACTION,
    CARDTYPE,
    RARITY,
    STATE,
    SUMMONED,
    FREEZE,
    ENRAGED,
    OVERLOAD,
    LOYALTY,
    DEATHRATTLE,
    BATTLECRY,
    SECRET,
    COMBO,
    CANT_HEAL,
    CANT_DAMAGE,
    CANT_SET_ASIDE,
    CANT_REMOVE_FROM_GAME,
    CANT_READY,
    CANT_ATTACK,
    CANT_DISCARD,
    CANT_PLAY,
    CANT_DRAW,
    CANT_BE_HEALED,
    IMMUNE,
    CANT_BE_SET_ASIDE,
    CANT_BE_REMOVED_FROM_GAME,
    CANT_BE_READIED,
    CANT_BE_ATTACKED,
    CANT_BE_TARGETED,
    CANT_BE_DESTROYED,
    CANT_BE_SUMMONING_SICK,
    FROZEN,
    JUST_PLAYED,
    LINKED_ENTITY,
    ZONE_POSITION,
    CANT_BE_FROZEN,
    COMBO_ACTIVE,
    CARD_TARGET,
    NUM_CARDS_PLAYED_THIS_TURN,
    CANT_BE_TARGETED_BY_OPPONENTS,
    NUM_TURNS_IN_PLAY,
    NUM_TURNS_LEFT,
    NUM_TURNS_IN_HAND,
    CURRENT_SPELLPOWER,
    ARMOR,
    MORPH,
    IS_MORPHED,
    TEMP_RESOURCES,
    OVERLOAD_OWED,
    NUM_ATTACKS_THIS_TURN,
    NEXT_ALLY_BUFF,
    MAGNET,
    FIRST_CARD_PLAYED_THIS_TURN,
    MULLIGAN_STATE,
    TAUNT_READY,
    STEALTH_READY,
    CHARGE_READY,
    CANT_BE_TARGETED_BY_SPELLS,
    SHOULDEXITCOMBAT,
    CREATOR,
    CANT_BE_SILENCED,
    PARENT_CARD,
    NUM_MINIONS_PLAYED_THIS_TURN,
    PREDAMAGE,
    COLLECTIBLE,
    HEALING_DOES_DAMAGE,
    DATABASE_ID,
    ENCHANTMENT_BIRTH_VISUAL,
    ENCHANTMENT_IDLE_VISUAL,
    CANT_BE_TARGETED_BY_HERO_POWERS,
    HEALTH_MINIMUM,
    TAG_ONE_TURN_EFFECT,
    SILENCE,
    COUNTER,
    ZONES_REVEALED,
    ADJACENT_BUFF,
    FORCED_PLAY,
    LOW_HEALTH_THRESHOLD,
    SPELLPOWER_DOUBLE,
    SPELL_HEALING_DOUBLE,
    NUM_OPTIONS_PLAYED_THIS_TURN,
    TO_BE_DESTROYED,
    AURA,
    POISONOUS,
    HERO_POWER_DOUBLE,
    AI_MUST_PLAY,
    NUM_MINIONS_PLAYER_KILLED_THIS_TURN,
    NUM_MINIONS_KILLED_THIS_TURN,
    AFFECTED_BY_SPELL_POWER,
    EXTRA_MINION_DEATHRATTLES_BASE,
    START_WITH_1_HEALTH,
    IMMUNE_WHILE_ATTACKING,
    MULTIPLY_HERO_DAMAGE,
    MULTIPLY_BUFF_VALUE,
    CUSTOM_KEYWORD_EFFECT,
    TOPDECK,
    CANT_BE_TARGETED_BY_BATTLECRIES,
    HERO_POWER,
    DEATHRATTLE_RETURN_ZONE,
    STEADY_SHOT_CAN_TARGET,
    DISPLAYED_CREATOR,
    POWERED_UP,
    SPARE_PART,
    FORGETFUL,
    CAN_SUMMON_MAXPLUSONE_MINION,
    OBFUSCATED,
    BURNING,
    OVERLOAD_LOCKED,
    NUM_TIMES_HERO_POWER_USED_THIS_GAME,
    CURRENT_HEROPOWER_DAMAGE_BONUS,
    HEROPOWER_DAMAGE,
    NUM_FRIENDLY_MINIONS_THAT_DIED_THIS_TURN,
    NUM_CARDS_DRAWN_THIS_TURN,
    AI_ONE_SHOT_KILL,
    EVIL_GLOW,
    HIDE_STATS,
    INSPIRE,
    RECEIVES_DOUBLE_SPELLDAMAGE_BONUS,
    HEROPOWER_ADDITIONAL_ACTIVATIONS,
    HEROPOWER_ACTIVATIONS_THIS_TURN,
    REVEALED,
    EXTRA_BATTLECRIES_BASE,
    NUM_FRIENDLY_MINIONS_THAT_DIED_THIS_GAME,
    CANNOT_ATTACK_HEROES,
    LOCK_AND_LOAD,
    DISCOVER,
    SHADOWFORM,
    NUM_FRIENDLY_MINIONS_THAT_ATTACKED_THIS_TURN,
    NUM_RESOURCES_SPENT_THIS_GAME,
    CHOOSE_BOTH,
    ELECTRIC_CHARGE_LEVEL,
    HEAVILY_ARMORED,
    DONT_SHOW_IMMUNE,
    RITUAL,
    PREHEALING,
    APPEAR_FUNCTIONALLY_DEAD,
    OVERLOAD_THIS_GAME,
    SPELLS_COST_HEALTH,
    HISTORY_PROXY_NO_BIG_CARD,
    IGNORE_TAUNT,
    PROXY_CTHUN,
    TRANSFORMED_FROM_CARD,
    CTHUN,
    CAST_RANDOM_SPELLS,
    SHIFTING,
    JADE_GOLEM,
    EMBRACE_THE_SHADOW,
    CHOOSE_ONE,
    EXTRA_ATTACKS_THIS_TURN,
    SEEN_CTHUN,
    MINION_TYPE_REFERENCE,
    UNTOUCHABLE,
    RED_MANA_GEM,
    SCORE_LABELID_1,
    SCORE_VALUE_1,
    SCORE_LABELID_2,
    SCORE_LABELID_3,
    SCORE_VALUE_2,
    SCORE_VALUE_3,
    CANT_BE_FATIGUED,
    AUTOATTACK,
    ARMS_DEALING,
    QUEST,
    TAG_LAST_KNOWN_COST_IN_HAND,
    DEFINING_ENCHANTMENT,
    FINISH_ATTACK_SPELL_ON_DAMAGE,
    MODULAR_ENTITY_PART_1,
    MODULAR_ENTITY_PART_2,
    MODIFY_DEFINITION_ATTACK,
    MODIFY_DEFINITION_HEALTH,
    MODIFY_DEFINITION_COST,
    MULTIPLE_CLASSES,
    ALL_TARGETS_RANDOM,
    MULTI_CLASS_GROUP,
    CARD_COSTS_HEALTH,
    GRIMY_GOONS,
    JADE_LOTUS,
    KABAL,
    ADDITIONAL_PLAY_REQS_1,
    ADDITIONAL_PLAY_REQS_2,
    ELEMENTAL_POWERED_UP,
    QUEST_PROGRESS,
    QUEST_PROGRESS_TOTAL,
    QUEST_CONTRIBUTOR,
    ADAPT,
    IS_CURRENT_TURN_AN_EXTRA_TURN,
    EXTRA_TURNS_TAKEN_THIS_GAME,
    TREASURE,
    TREASURE_DEFINTIONAL_ATTACK,
    TREASURE_DEFINTIONAL_COST,
    TREASURE_DEFINTIONAL_HEALTH,
    ACTS_LIKE_A_SPELL,
    SHIFTING_MINION,
    SHIFTING_WEAPON,
    DEATH_KNIGHT,
    BOSS,
    STAMPEDE,
    EMPOWERED_TREASURE,
    ONE_SIDED_GHOSTLY,
    CURRENT_NEGATIVE_SPELLPOWER,
    IS_VAMPIRE,
    CORRUPTED,
    HIDE_HEALTH,
    HIDE_ATTACK,
    HIDE_COST,
    LIFESTEAL,
    OVERRIDE_EMOTE_0,
    OVERRIDE_EMOTE_1,
    OVERRIDE_EMOTE_2,
    OVERRIDE_EMOTE_3,
    OVERRIDE_EMOTE_4,
    OVERRIDE_EMOTE_5,
    SCORE_FOOTERID,
    RECRUIT,
    LOOT_CARD_1,
    LOOT_CARD_2,
    LOOT_CARD_3,
    HERO_POWER_DISABLED,
    VALEERASHADOW,
    OVERRIDECARDNAME,
    OVERRIDECARDTEXTBUILDER,
    DUNGEON_PASSIVE_BUFF,
    GHOSTLY,
    DISGUISED_TWIN,
    SECRET_DEATHRATTLE,
    RUSH,
    REVEAL_CHOICES,
    HERO_DECK_ID,
    HIDDEN_CHOICE,
    ZOMBEAST,
    HERO_EMOTE_SILENCED,
    MINION_IN_HAND_BUFF,
    ECHO,
    MODULAR,
    IGNORE_HIDE_STATS_FOR_BIG_CARD,
    REAL_TIME_TRANSFORM,
    WAIT_FOR_PLAYER_RECONNECT_PERIOD,
    ETHEREAL,
    EXTRA_DEATHRATTLES_BASE,
    PHASED_RESTART,
    HEALTH_DISPLAY,
    ENABLE_HEALTH_DISPLAY,
    VOODOO_LINK,
    OVERKILL,
    PROPHECY,
    ATTACKABLE_BY_RUSH,
    SHIFTING_SPELL,
    USE_ALTERNATE_CARD_TEXT,
    SUPPRESS_DEATH_SOUND,
    ECHOING_OOZE_SPELL,
    COLLECTIONMANAGER_FILTER_MANA_EVEN,
    COLLECTIONMANAGER_FILTER_MANA_ODD,
    AMOUNT_HEALED_THIS_GAME,
    ZOMBEAST_DEBUG_CURRENT_BEAST_DATABASE_ID,
    ZOMBEAST_DEBUG_CURRENT_ITERATION,
    ZOMBEAST_DEBUG_MAX_ITERATIONS,
    START_OF_GAME,
    ENCHANTMENT_INVISIBLE,
    PUZZLE,
    PUZZLE_PROGRESS,
    PUZZLE_PROGRESS_TOTAL,
    PUZZLE_TYPE,
    PUZZLE_COMPLETED,
    CONCEDE_BUTTON_ALTERNATIVE_TEXT,
    HIDE_RESTART_BUTTON,
    WILD,
    HALL_OF_FAME,
    MARK_OF_EVIL,
    DECK_RULE_MOD_DECK_SIZE,
    FAST_BATTLECRY,
    END_TURN_BUTTON_ALTERNATIVE_APPEARANCE,
    WAND,
    TREAT_AS_PLAYED_HERO_CARD,
    NUM_HERO_POWER_DAMAGE_THIS_GAME,
    PUZZLE_NAME,
    TURN_INDICATOR_ALTERNATIVE_APPEARANCE,
    PREVIOUS_PUZZLE_COMPLETED,
    GLORIOUSGLOOP,
    HEALTH_DISPLAY_COLOR,
    HEALTH_DISPLAY_NEGATIVE,
    WHIZBANG_DECK_ID,
    HIDE_OUT_OF_CARDS_WARNING,
    GEARS,
    LUNAHIGHLIGHTHINT,
    SUPPRESS_JOBS_DONE_VO,
    SHRINE,
    ALL_HEALING_DOUBLE,
    BLOCK_ALL_INPUT,
    PUZZLE_MODE,
    CARD_DOES_NOTHING,
    CASTSWHENDRAWN,
    DISPLAY_CARD_ON_MOUSEOVER,
    DECK_POWER_UP,
    REBORN,
    SQUELCH_NON_GAME_TRIGGERS_AND_MODIFIERS,
    QUEST_REWARD_DATABASE_ID,
    DORMANT_VISUAL,
    CUSTOMTEXT1,
    CUSTOMTEXT2,
    CUSTOMTEXT3,
    FLOOPY,
    PLAYER_BASE_SHRINE_DECK_ID,
    HIDE_WATERMARK,
    EXTRA_MINION_BATTLECRIES_BASE,
    RUN_PROGRESS,
    NON_KEYWORD_ECHO,
    PLAYER_TAG_THRESHOLD_TAG_ID,
    PLAYER_TAG_THRESHOLD_VALUE,
    HEALING_DOES_DAMAGE_HINT,
    AFFECTED_BY_HEALING_DOES_DAMAGE,
    DECK_LIST_SORT_ORDER,
    EXTRA_BATTLECRIES_ADDITIONAL,
    EXTRA_DEATHRATTLES_ADDITIONAL,
    ALTERNATE_MOUSE_OVER_CARD,
    ENCHANTMENT_BANNER_TEXT,
    MOUSE_OVER_CARD_APPEARANCE,
    IS_ADVENTURE_SCENARIO,
    TWINSPELL_COPY,
    PROXY_GALAKROND,
    SIDEQUEST,
    TWINSPELL,
    GALAKROND_IN_PLAY,
    COIN_MANA_GEM,
    MEGA_WINDFURY,
    EMPOWER,
    EMPOWER_PRIEST,
    EMPOWER_ROGUE,
    EMPOWER_SHAMAN,
    EMPOWER_WARLOCK,
    EMPOWER_WARRIOR,
    TWINSPELLPENDING,
    DRUSTVAR_HORROR_DEBUG_CURRENT_SPELL_DATABASE_ID,
    DRUSTVAR_HORROR_DEBUG_CURRENT_ITERATION,
    HEROIC_HERO_POWER,
    DRUSTVAR_HORROR_DEBUG_MAX_ITERATIONS,
    CREATOR_DBID,
    FATIGUEREFERENCE,
    HERO_FLYING,
    UI_BUFF_HEALTH_UP,
    UI_BUFF_SET_COST_ZERO,
    UI_BUFF_COST_DOWN,
    UI_BUFF_ATK_UP,
    UI_BUFF_COST_UP,
    DEBUG_DISPLAY_TAG_BOTTOM_RIGHT,
    DEBUG_DISPLAY_TAG_TOP_RIGHT,
    SMART_DISCOVER_DEBUG_ENTITY_1,
    SMART_DISCOVER_DEBUG_ENTITY_2,
    SMART_DISCOVER_DEBUG_ENTITY_3,
    SMART_DISCOVER_DEBUG_TEST_COMPLETE,
    SMART_DISCOVER_DEBUG_PASSIVE_EVAL_RESULT_1,
    SMART_DISCOVER_DEBUG_PASSIVE_EVAL_RESULT_2,
    SMART_DISCOVER_DEBUG_PASSIVE_EVAL_RESULT_3,
    COPIED_BY_KHADGAR,
    OUTCAST,
    ALTERNATE_CHAPTER_VO,
    AI_MAKES_DECISIONS_FOR_PLAYER,
    HAS_BEEN_REBORN,
    USE_DISCOVER_VISUALS,
    DOUBLE_FATIGUE_DAMAGE,
    BOARD_VISUAL_STATE,
    BACON_DUMMY_PLAYER,
    SQUELCH_LIFETIME_EFFECTS,
    ALLOW_MOVE_MINION,
    TAG_TB_RANDOM_DECK_TIME_ID,
    NEXT_OPPONENT_PLAYER_ID,
    MAIN_GALAKROND,
    GOOD_OL_GENERIC_FRIENDLY_DRAGON_DISCOVER_VISUALS,
    GALAKROND_HERO_CARD,
    INVOKE_COUNTER,
    PLAYER_LEADERBOARD_PLACE,
    PLAYER_TECH_LEVEL,
    BACON_HERO_POWER_ACTIVATED,
    USE_FAST_ACTOR_TRANSITION_ANIMATIONS,
    DECK_RULE_COUNT_AS_COPY_OF_CARD_ID,
    STUDY,
    BACON_ODD_PLAYER_OUT,
    BACON_MINION_IS_LEVEL_TWO,
    BACON_IS_KEL_THUZAD,
    HIGHLIGHT_ATTACKING_MINION_DURING_COMBAT,
    SPELLBURST,
    RULEBOOK,
    FX_DATANUM_1,
    BACON_ACTION_CARD,
    GAME_MODE_BUTTON_SLOT,
    TECH_LEVEL,
    TECH_LEVEL_MANA_GEM,
    UI_BUFF_DURABILITY_UP,
    PLAYER_TRIPLES,
    DISABLE_TURN_INDICATORS,
    COLLECTION_RELATED_CARD_DATABASE_ID,
    IS_BACON_POOL_MINION,
    SUPPRESS_ALL_SUMMON_VO,
    BACON_TRIPLE_CANDIDATE,
    BATTLEGROUNDS_PREMIUM_EMOTES,
    MOVE_MINION_HOVER_TARGET_SLOT,
    BACON_COIN_ON_ENEMY_MINIONS,
    ALWAYS_USE_FAST_ACTOR_TRIGGERS,
    PIECE_OF_CTHUN,
    BACON_HERO_CAN_BE_DRAFTED,
    DISABLE_GOLDEN_ANIMATIONS,
    WATERMARK_OVERRIDE_CARD_SET,
    DORMANT,
    DORMANT_AWAKEN_CONDITION_ENCHANT,
    SUPPRESS_SUMMON_VO_FOR_PLAYER,
    CORRUPT,
    ALLOW_GAME_SPEEDUP,
    POISONOUS_INSTANT,
    FORCE_NO_CUSTOM_SPELLS,
    START_OF_COMBAT,
    CORRUPTEDCARD,
    BACON_HERO_EARLY_ACCESS,
    SPAWN_TIME_COUNT,
    SKIP_MULLIGAN,
    COPIED_FROM_ENTITY_ID,
    BACON_VERDANTSPHERES,
    OPPONENT_SIDE_GHOSTLY,
    FORCE_NO_CUSTOM_LIFETIME_SPELLS,
    FORCE_NO_CUSTOM_SUMMON_SPELLS,
    FORCE_NO_CUSTOM_KEYWORD_SPELLS,
    USE_LEADERBOARD_AS_SPAWN_ORIGIN,
    BACON_MUKLA_BANANA_SPAWN_COUNT,
    REPLACEMENT_ENTITY,
    SPELL_SCHOOL,
    FRENZY,
    COIN_MANA_GEM_FOR_CHOICE_CARDS,
    METAMORPHOSIS,
    HERO_POWER_ENTITY,
    DISCOVER_STUDIES_VISUAL,
    LETTUCE_CONTROLLER,
    LETTUCE_ABILITY_OWNER,
    LETTUCE_SELECTED_TARGET,
    LETTUCE_SELECTED_SUBCARD_INDEX,
    LETTUCE_MERCENARY,
    LETTUCE_ROLE,
    LETTUCE_IS_COMBAT_ACTION_TAKEN,
    LETTUCE_COOLDOWN_CONFIG,
    LETTUCE_CURRENT_COOLDOWN,
    LETTUCE_PASSIVE_ABILITY,
    LIFESTEAL_DAMAGES_OPPOSING_HERO,
    LETTUCE_ABILITY_SUMMONED_MINION,
    SPELLS_CAST_TWICE,
    CHOICE_NAME_DISPLAY_TYPE,
    CHOICE_ACTOR_TYPE,
    FORCE_GREEN_GLOW_ACTIVE,
    SOURCE_OVERRIDE_FOR_MODIFIER_TEXT,
    LETTUCE_ABILITY_TILE_VISUAL_SELF_ONLY,
    LETTUCE_ABILITY_TILE_VISUAL_ALL_VISIBLE,
    ACTION_STEP_TYPE,
    FAKE_ZONE,
    FAKE_ZONE_POSITION,
    LETTUCE_MAX_IN_PLAY_MERCENARIES,
    LETTUCE_MERCENARIES_TO_NOMINATE,
    LETTUCE_COOLDOWN_WHILE_BENCHED,
    PENDING_TRANSFORM_TO_CARD,
    TRANSFORMED_FROM_CARD_VISUAL_TYPE,
    TRADEABLE,
    TOOL,
    QUESTLINE,
    LETTUCE_MERCENARY_RESERVE,
    LETTUCE_SKIP_MERCENARY_RESERVE,
    PLAYER_ID_LOOKUP,
    TRADE_COST,
    BACON_AVALANCHE,
    SIGIL,
    LETTUCE_DISABLE_AUTO_SELECT_NEXT_MERC,
    PLAYED_CTHUN_EYE,
    PLAYED_CTHUN_BODY,
    PLAYED_CTHUN_MAW,
    PLAYED_CTHUN_HEART,
    PROXY_CTHUN_SHATTERED,
    PROGRESSBAR_TOTAL,
    PROGRESSBAR_PROGRESS,
    PROGRESSBAR_CARDID,
    PROGRESSBAR_SHOW,
    PROGRESSBAR_TEXT,
    LIFESTEAL_DOES_DAMAGE_HINT,
    DARKMOON_TICKET,
    NUM_SPELLS_PLAYED_THIS_GAME,
    BACON_COMEONECOMEALL,
    LETTUCE_ABILITY_USED_LAST_TURN,
    LETTUCE_NODE_TYPE,
    SHOW_DISCOVER_FROM_DECK,
    MINI_SET,
    ARMOR_GAINED_THIS_GAME,
    CANT_TRIGGER_DEATHRATTLE,
    CANT_MOVE_MINION,
    LETTUCE_MERCENARY_EXPERIENCE,
    LETTUCE_IS_EQUPIMENT,
    LETTUCE_EQUIPMENT_ID,
    DARKMOON_FAIRE_PRIZES_ACTIVE,
    IGNORE_DECK_RULESET,
    HONORABLEKILL,
    HAS_DIAMOND_QUALITY,
    CURRENT_SPELLPOWER_ARCANE,
    CURRENT_SPELLPOWER_FIRE,
    CURRENT_SPELLPOWER_FROST,
    CURRENT_SPELLPOWER_NATURE,
    CURRENT_SPELLPOWER_HOLY,
    CURRENT_SPELLPOWER_SHADOW,
    CURRENT_SPELLPOWER_FEL,
    CURRENT_SPELLPOWER_PHYSICAL,
    NON_KEYWORD_POISONOUS,
    SPELLPOWER_ARCANE,
    SPELLPOWER_FIRE,
    SPELLPOWER_FROST,
    SPELLPOWER_NATURE,
    SPELLPOWER_HOLY,
    SPELLPOWER_SHADOW,
    SPELLPOWER_FEL,
    SPELLPOWER_PHYSICAL,
    ENRAGED_TOOLTIP,
    BLOOD_GEM,
    LETTUCE_HAS_MANUALLY_SELECTED_ABILITY,
    LETTUCE_KEEP_LAST_STANDING_MINION_ACTOR,
    GOLDSPARKLES_HINT,
    LETTUCE_USE_DETERMINISTIC_TEAM_ABILITY_QUEUING,
    QUESTLINE_FINAL_REWARD_DATABASE_ID,
    QUESTLINE_PART,
    QUESTLINE_REQUIREMENT_MET_1,
    QUESTLINE_REQUIREMENT_MET_2,
    QUESTLINE_REQUIREMENT_MET_3,
    DONT_SHOW_IN_HISTORY,
    FAKE_CONTROLLER,
    BACON_SKIN,
    BACON_SKIN_PARENT_ID,
    GAME_SEED,
    IS_USING_TRADE_OPTION,
    BACON_BOB_SKIN,
    BACON_COMBAT_DAMAGE_CAP,
    REFRESH,
    TARGETING_ARROW_TYPE,
    LETTUCE_CURRENT_BOUNTY_ID,
    AVENGE,
    SPELLRESISTANCE_ARCANE,
    SPELLRESISTANCE_FIRE,
    SPELLRESISTANCE_FROST,
    SPELLRESISTANCE_NATURE,
    SPELLRESISTANCE_HOLY,
    SPELLRESISTANCE_SHADOW,
    SPELLRESISTANCE_FEL,
    SPELLWEAKNESS_ARCANE,
    SPELLWEAKNESS_FIRE,
    SPELLWEAKNESS_FROST,
    SPELLWEAKNESS_NATURE,
    SPELLWEAKNESS_HOLY,
    SPELLWEAKNESS_SHADOW,
    SPELLWEAKNESS_FEL,
    BACON_STARSTOBOUNCEOFF,
    LETTUCE_ATTACK,
    LETTUCE_SPELLCOMBO,
    LETTUCE_BOUNTY_BOSS,
    LETTUCE_IS_TREASURE_CARD,
    LETTUCE_SPELLDAMAGEARCANE,
    LETTUCE_SPELLDAMAGEFEL,
    LETTUCE_SPELLDAMAGEFIRE,
    LETTUCE_SPELLDAMAGEFROST,
    LETTUCE_SPELLDAMAGEHOLY,
    LETTUCE_SPELLDAMAGENATURE,
    LETTUCE_SPELLDAMAGESHADOW,
    ROOTED,
    VULNERABLE,
    DEATHBLOW,
    BLEED,
    CRITICALDAMAGE,
    ROOT,
    LETTUCE_SHOW_OPPOSING_FAKE_HAND,
    BACON_DIABLO_FIGHT_DIABLO_PLAYER_ID,
    LETTUCE_VERSUS_SPELL_STATE,
    LETTUCE_START_OF_GAME_ABILITY,
    CURRENT_TEMP_SPELLPOWER_ARCANE,
    CURRENT_TEMP_SPELLPOWER_FEL,
    CURRENT_TEMP_SPELLPOWER_FIRE,
    CURRENT_TEMP_SPELLPOWER_FROST,
    CURRENT_TEMP_SPELLPOWER_NATURE,
    CURRENT_TEMP_SPELLPOWER_HOLY,
    CURRENT_TEMP_SPELLPOWER_PHYSICAL,
    CURRENT_TEMP_SPELLPOWER_SHADOW,
    CURRENT_TEMP_SPELLPOWER,
    BACON_CHOSEN_BOARD_SKIN_ID,
    LETTUCE_ALLIANCE,
    LETTUCE_HORDE,
    OBJECTIVE,
    LETTUCE_REFRESH,
    LETTUCE_ELVES,
    AVFACTION,
    AVRANK,
    OBJECTIVE_AURA,
    CURRENT_HEALING_POWER,
    InvisibleDeathrattle,
    ImmuneToSpellpower,
    AttackVisualType,
    DevState,
    GrantCharge,
    HealTarget,
}
