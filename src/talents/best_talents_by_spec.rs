use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestTalentsBasedOnSpec {
    pub rankings: Rankings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub ranked_characters: Vec<RankedCharacter>,
    pub ui: Ui,
    pub region: Region3,
    pub realm: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedCharacter {
    pub rank: i64,
    pub score: f64,
    pub score_color: String,
    pub runs: Vec<Run>,
    pub character: Character,
    pub guild: Option<Guild>,
    pub patron_level: Option<PatronLevel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub zone_id: i64,
    pub keystone_run_id: i64,
    pub clear_time_ms: i64,
    pub mythic_level: i64,
    pub score: f64,
    pub period: i64,
    pub affixes: Vec<i64>,
    pub logged_run_id: i64,
    pub num_chests: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: i64,
    #[serde(rename = "persona_id")]
    pub persona_id: i64,
    pub name: String,
    pub class: Class,
    pub race: Race,
    pub faction: String,
    pub level: i64,
    pub spec: Spec,
    pub path: String,
    pub realm: Realm,
    pub region: Region,
    pub stream: Value,
    pub recruitment_profiles: Vec<RecruitmentProfile>,
    pub talent_loadout_text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub faction: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[serde(rename = "class_id")]
    pub class_id: i64,
    pub role: String,
    #[serde(rename = "is_melee")]
    pub is_melee: bool,
    pub patch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realm {
    pub id: i64,
    pub connected_realm_id: i64,
    pub wow_realm_id: Option<i64>,
    pub wow_connected_realm_id: i64,
    pub name: String,
    pub alt_name: Option<String>,
    pub slug: String,
    pub alt_slug: String,
    pub locale: String,
    pub is_connected: bool,
    pub realm_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub name: String,
    pub slug: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecruitmentProfile {
    #[serde(rename = "activity_type")]
    pub activity_type: String,
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    #[serde(rename = "recruitment_profile_id")]
    pub recruitment_profile_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: i64,
    pub name: String,
    pub faction: String,
    pub realm: Realm2,
    pub region: Region2,
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realm2 {
    pub id: i64,
    pub connected_realm_id: i64,
    pub wow_realm_id: i64,
    pub wow_connected_realm_id: i64,
    pub name: String,
    pub alt_name: Option<String>,
    pub slug: String,
    pub alt_slug: String,
    pub locale: String,
    pub is_connected: bool,
    pub realm_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region2 {
    pub name: String,
    pub slug: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatronLevel {
    pub level: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui {
    pub region: String,
    pub season: String,
    pub class: String,
    pub spec: String,
    pub page: i64,
    pub page_size: i64,
    pub last_page: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region3 {
    pub name: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    pub slug: String,
}
