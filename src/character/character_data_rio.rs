use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CharacterDataFromRio {
    name: String,
    race: String,
    class: String,
    active_spec_name: String,
    active_spec_role: String,
    gender: String,
    faction: String,
    achievement_points: i64,
    thumbnail_url: String,
    region: String,
    realm: String,
    last_crawled_at: String,
    profile_url: String,
    profile_banner: String,
    mythic_plus_scores_by_season: Vec<MythicPlusScoresBySeason>,
    gear: Gear,
    #[serde(rename = "talentLoadout")]
    talent_loadout: TalentLoadout,
    raid_progression: RaidProgression,
    guild: Guild,
}

#[derive(Serialize, Deserialize)]
pub struct Gear {
    updated_at: String,
    item_level_equipped: i64,
    item_level_total: i64,
    artifact_traits: i64,
    corruption: GearCorruption,
    items: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearCorruption {
    added: i64,
    resisted: i64,
    total: i64,
    cloak_rank: i64,
    spells: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    item_id: i64,
    item_level: i64,
    enchant: Option<i64>,
    icon: String,
    name: String,
    item_quality: i64,
    is_legendary: bool,
    is_azerite_armor: bool,
    azerite_powers: Vec<Option<AzeritePower>>,
    corruption: ItemCorruption,
    domination_shards: Vec<Option<serde_json::Value>>,
    gems: Vec<Option<serde_json::Value>>,
    enchants: Vec<i64>,
    bonuses: Vec<i64>,
    tier: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AzeritePower {
    id: i64,
    spell: Spell,
    tier: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    id: i64,
    school: i64,
    icon: String,
    name: String,
    rank: Option<String>,
    has_cooldown: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemCorruption {
    added: i64,
    resisted: i64,
    total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Guild {
    name: String,
    realm: String,
}

#[derive(Serialize, Deserialize)]
pub struct MythicPlusScoresBySeason {
    season: String,
    scores: Scores,
    segments: Segments,
}

#[derive(Serialize, Deserialize)]
pub struct Scores {
    all: f64,
    dps: f64,
    healer: i64,
    tank: i64,
    spec_0: i64,
    spec_1: f64,
    spec_2: i64,
    spec_3: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Segments {
    all: All,
    dps: All,
    healer: All,
    tank: All,
    spec_0: All,
    spec_1: All,
    spec_2: All,
    spec_3: All,
}

#[derive(Serialize, Deserialize)]
pub struct All {
    score: f64,
    color: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RaidProgression {
    blackrock_depths: BlackrockDepths,
    nerubar_palace: BlackrockDepths,
}

#[derive(Serialize, Deserialize)]
pub struct BlackrockDepths {
    summary: String,
    expansion_id: i64,
    total_bosses: i64,
    normal_bosses_killed: i64,
    heroic_bosses_killed: i64,
    mythic_bosses_killed: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TalentLoadout {
    loadout_spec_id: i64,
    loadout_text: String,
    loadout: Vec<Loadout>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loadout {
    node: Node,
    entry_index: i64,
    rank: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    id: i64,
    tree_id: i64,
    sub_tree_id: i64,
    #[serde(rename = "type")]
    node_type: i64,
    entries: Vec<Entry>,
    important: bool,
    pos_x: i64,
    pos_y: i64,
    row: i64,
    col: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    id: i64,
    trait_definition_id: i64,
    trait_sub_tree_id: i64,
    #[serde(rename = "type")]
    entry_type: i64,
    max_ranks: i64,
    spell: Option<Spell>,
}
