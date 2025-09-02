use std::fmt::Write;
use std::str::FromStr;
use std::fmt::Display;
use bitflags::bitflags;
use hashbrown::{HashMap, HashSet};
use log::error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use serde_with::serde_as;
use serde_with::DefaultOnError;
use compact_str::CompactString;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum EntityType {
    #[default]
    UNKNOWN,
    MONSTER,
    BOSS,
    GUARDIAN,
    PLAYER,
    NPC,
    ESTHER,
    PROJECTILE,
    SUMMON,
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EntityType::UNKNOWN => "UNKNOWN".to_string(),
            EntityType::MONSTER => "MONSTER".to_string(),
            EntityType::BOSS => "BOSS".to_string(),
            EntityType::GUARDIAN => "GUARDIAN".to_string(),
            EntityType::PLAYER => "PLAYER".to_string(),
            EntityType::NPC => "NPC".to_string(),
            EntityType::ESTHER => "ESTHER".to_string(),
            EntityType::PROJECTILE => "PROJECTILE".to_string(),
            EntityType::SUMMON => "SUMMON".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for EntityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNKNOWN" => Ok(EntityType::UNKNOWN),
            "MONSTER" => Ok(EntityType::MONSTER),
            "BOSS" => Ok(EntityType::BOSS),
            "GUARDIAN" => Ok(EntityType::GUARDIAN),
            "PLAYER" => Ok(EntityType::PLAYER),
            "NPC" => Ok(EntityType::NPC),
            "ESTHER" => Ok(EntityType::ESTHER),
            _ => Ok(EntityType::UNKNOWN),
        }
    }
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub last_combat_packet: i64,
    pub fight_start: i64,
    pub local_player: CompactString,
    pub entities: HashMap<CompactString, EncounterEntity>,
    pub current_boss_name: CompactString,
    pub current_boss: Option<EncounterEntity>,
    pub encounter_damage_stats: EncounterDamageStats,
    pub duration: i64,
    pub difficulty: Option<CompactString>,
    pub favorite: bool,
    pub cleared: bool,
    pub boss_only_damage: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<CompactString>,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDamageStats {
    pub total_damage_dealt: i64,
    pub top_damage_dealt: i64,
    pub total_damage_taken: i64,
    pub top_damage_taken: i64,
    pub dps: i64,
    pub buffs: HashMap<u32, StatusEffect>,
    pub debuffs: HashMap<u32, StatusEffect>,
    pub total_shielding: u64,
    pub total_effective_shielding: u64,
    pub applied_shield_buffs: HashMap<u32, StatusEffect>,
    #[serde(skip)]
    pub unknown_buffs: HashSet<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<EncounterMisc>,
    pub boss_hp_log: HashMap<CompactString, Vec<BossHpLog>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterEntity {
    pub id: u64,
    pub character_id: u64,
    pub npc_id: u32,
    pub name: CompactString,
    pub entity_type: EntityType,
    pub class_id: u32,
    pub class: CompactString,
    pub gear_score: f32,
    pub current_hp: i64,
    pub max_hp: i64,
    pub current_shield: u64,
    pub is_dead: bool,
    pub skills: HashMap<u32, Skill>,
    pub damage_stats: DamageStats,
    pub skill_stats: SkillStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engraving_data: Option<Vec<CompactString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark_passive_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark_passive_data: Option<ArkPassiveData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loadout_hash: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combat_power: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Skill {
    pub id: u32,
    pub name: CompactString,
    pub icon: CompactString,
    pub total_damage: i64,
    pub max_damage: i64,
    pub max_damage_cast: i64,
    pub buffed_by: HashMap<u32, i64>,
    pub debuffed_by: HashMap<u32, i64>,
    pub buffed_by_support: i64,
    pub buffed_by_identity: i64,
    pub buffed_by_hat: i64,
    pub debuffed_by_support: i64,
    pub casts: i64,
    pub hits: i64,
    pub crits: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_crit: Option<f64>,
    pub crit_damage: i64,
    pub back_attacks: i64,
    pub front_attacks: i64,
    pub back_attack_damage: i64,
    pub front_attack_damage: i64,
    pub dps: i64,
    pub cast_log: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tripod_index: Option<TripodIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tripod_level: Option<TripodLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gem_cooldown: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gem_tier: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gem_damage: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gem_tier_dmg: Option<u8>,
    pub rdps_damage_received: i64,
    pub rdps_damage_received_support: i64,
    pub rdps_damage_given: i64,
    pub skill_cast_log: Vec<SkillCast>,

    #[serde(default)]
    pub is_hyper_awakening: bool,
    // for skills that cannot crit or be buffed
    // like hyper awakening, paradise orb, transcendence, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<bool>,

    #[serde(skip)]
    pub last_timestamp: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_available: Option<i64>, // total time skill was available to cast
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodData {
    pub index: u8,
    pub options: Vec<SkillFeatureOption>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodLevel {
    pub first: u16,
    pub second: u16,
    pub third: u16,
}

impl PartialEq for TripodLevel {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second && self.third == other.third
    }
}

impl Eq for TripodLevel {}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodIndex {
    pub first: u8,
    pub second: u8,
    pub third: u8,
}

impl PartialEq for TripodIndex {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second && self.third == other.third
    }
}

impl Eq for TripodIndex {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveData {
    pub evolution: Option<Vec<ArkPassiveNode>>,
    pub enlightenment: Option<Vec<ArkPassiveNode>>,
    pub leap: Option<Vec<ArkPassiveNode>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArkPassiveNode {
    pub id: u32,
    pub lv: u8,
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct DamageStats {
    pub damage_dealt: i64,
    pub hyper_awakening_damage: i64,
    pub damage_taken: i64,
    pub buffed_by: HashMap<u32, i64>,
    pub debuffed_by: HashMap<u32, i64>,
    pub buffed_by_support: i64,
    pub buffed_by_identity: i64,
    pub debuffed_by_support: i64,
    pub buffed_by_hat: i64,
    pub crit_damage: i64,
    pub back_attack_damage: i64,
    pub front_attack_damage: i64,
    pub shields_given: u64,
    pub shields_received: u64,
    pub damage_absorbed: u64,
    pub damage_absorbed_on_others: u64,
    pub shields_given_by: HashMap<u32, u64>,
    pub shields_received_by: HashMap<u32, u64>,
    pub damage_absorbed_by: HashMap<u32, u64>,
    pub damage_absorbed_on_others_by: HashMap<u32, u64>,
    pub deaths: i64,
    pub death_time: i64,
    pub dps: i64,
    #[serde(default)]
    pub dps_average: Vec<i64>,
    #[serde(default)]
    pub dps_rolling_10s_avg: Vec<i64>,
    pub rdps_damage_received: i64,
    pub rdps_damage_received_support: i64,
    pub rdps_damage_given: i64,
    #[serde(default)]
    pub incapacitations: Vec<IncapacitatedEvent>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillStats {
    pub casts: i64,
    pub hits: i64,
    pub crits: i64,
    pub back_attacks: i64,
    pub front_attacks: i64,
    pub counters: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_stats: Option<CompactString>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillCast {
    pub timestamp: i64,
    pub last: i64,
    pub hits: Vec<SkillHit>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillHit {
    pub timestamp: i64,
    pub damage: i64,
    pub crit: bool,
    pub back_attack: bool,
    pub front_attack: bool,
    pub buffed_by: Vec<u32>,
    pub debuffed_by: Vec<u32>,
    pub rdps_damage_received: i64,
    pub rdps_damage_received_support: i64,
}

#[derive(Debug)]
pub struct DamageData {
    pub skill_id: u32,
    pub skill_effect_id: u32,
    pub damage: i64,
    pub shield_damage: Option<i64>,
    pub modifier: i32,
    pub target_current_hp: i64,
    pub target_max_hp: i64,
    pub damage_attribute: Option<u8>,
    pub damage_type: u8,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub gauge1: u32,
    pub gauge2: u32,
    pub gauge3: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncapacitatedEvent {
    #[serde(rename = "type")]
    pub event_type: IncapacitationEventType,
    pub timestamp: i64,
    pub duration: i64, // in a live meter, this might be retroactively updated to be shortened if the user uses get up or gets incapacitated with the same type again
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IncapacitationEventType {
    FALL_DOWN,
    CROWD_CONTROL,
}

pub type IdentityLog = Vec<(i64, (u32, u32, u32))>;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityArcanist {
    // timestamp, (percentage, card, card)
    pub log: Vec<(i32, (f32, u32, u32))>,
    pub average: f64,
    pub card_draws: HashMap<u32, u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityArtistBard {
    // timestamp, (percentage, bubble)
    pub log: Vec<(i32, (f32, u32))>,
    pub average: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityGeneric {
    // timestamp, percentage
    pub log: Vec<(i32, f32)>,
    pub average: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
#[serde_as]
pub struct EncounterMisc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boss_hp_log: Option<HashMap<CompactString, Vec<BossHpLog>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raid_clear: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_info: Option<HashMap<i32, Vec<CompactString>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdps_valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdps_message: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_fight_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_save: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BossHpLog {
    pub time: i32,
    pub hp: i64,
    #[serde(default)]
    pub p: f32,
}

impl BossHpLog {
    pub fn new(time: i32, hp: i64, p: f32) -> Self {
        Self { time, hp, p }
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Npc {
    pub id: i32,
    pub name: Option<CompactString>,
    pub grade: CompactString,
    #[serde(rename = "type")]
    pub npc_type: CompactString,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Esther {
    pub name: CompactString,
    pub icon: CompactString,
    pub skills: Vec<i32>,
    #[serde(alias = "npcs")]
    pub npc_ids: Vec<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillData {
    pub id: i32,
    pub name: Option<CompactString>,
    #[serde(rename = "type", default)]
    #[serde(deserialize_with = "int_or_string_as_string")]
    pub skill_type: CompactString,
    pub desc: Option<CompactString>,
    pub class_id: u32,
    pub icon: Option<CompactString>,
    pub identity_category: Option<CompactString>,
    #[serde(alias = "groups")]
    pub groups: Option<Vec<i32>>,
    pub summon_source_skills: Option<Vec<u32>>,
    pub source_skills: Option<Vec<u32>>,
    #[serde(default)]
    pub is_hyper_awakening: bool,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillEffectData {
    pub id: i32,
    pub comment: CompactString,
    #[serde(skip)]
    pub stagger: i32,
    pub source_skills: Option<Vec<u32>>,
    pub directional_mask: Option<i32>,
    pub item_name: Option<CompactString>,
    pub item_desc: Option<CompactString>,
    pub item_type: Option<CompactString>,
    pub icon: Option<CompactString>,
    pub values: Vec<i32>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillBuffData {
    pub id: i32,
    pub name: Option<CompactString>,
    pub desc: Option<CompactString>,
    pub icon: Option<CompactString>,
    pub icon_show_type: Option<CompactString>,
    pub duration: i32,
    // buff | debuff
    pub category: CompactString,
    #[serde(rename(deserialize = "type"))]
    #[serde(deserialize_with = "int_or_string_as_string")]
    pub buff_type: CompactString,
    pub status_effect_values: Option<Vec<i32>>,
    pub buff_category: Option<CompactString>,
    pub target: CompactString,
    pub unique_group: u32,
    #[serde(rename(deserialize = "overlap"))]
    pub overlap_flag: i32,
    pub per_level_data: HashMap<CompactString, PerLevelData>,
    pub source_skills: Option<Vec<u32>>,
    pub set_name: Option<CompactString>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerLevelData {
    pub passive_options: Vec<PassiveOption>,
    // pub status_effect_values: Vec<i32>
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassiveOption {
    #[serde(rename(deserialize = "type"))]
    pub option_type: CompactString,
    pub key_stat: CompactString,
    pub key_index: i32,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusEffect {
    pub target: StatusEffectTarget,
    pub category: CompactString,
    pub buff_category: CompactString,
    pub buff_type: u32,
    pub unique_group: u32,
    pub source: StatusEffectSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum StatusEffectTarget {
    #[default]
    OTHER,
    PARTY,
    SELF,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEffectSource {
    pub name: CompactString,
    pub desc: CompactString,
    pub icon: CompactString,
    pub skill: Option<SkillData>,
    pub set_name: Option<CompactString>,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusEffectBuffTypeFlags: u32 {
        const NONE = 0;
        const DMG = 1;
        const CRIT = 1 << 1;
        const ATKSPEED = 1 << 2;
        const MOVESPEED = 1 << 3;
        const HP = 1 << 4;
        const DEFENSE = 1 << 5;
        const RESOURCE = 1 << 6;
        const COOLDOWN = 1 << 7;
        const STAGGER = 1 << 8;
        const SHIELD = 1 << 9;

        const ANY = 1 << 20;
    }
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CombatEffectData {
    pub effects: Vec<CombatEffectDetail>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CombatEffectDetail {
    pub ratio: i32,
    pub cooldown: i32,
    pub conditions: Vec<CombatEffectCondition>,
    pub actions: Vec<CombatEffectAction>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectCondition {
    #[serde(rename(deserialize = "type"))]
    pub condition_type: CompactString,
    pub actor_type: CompactString,
    pub arg: i32,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct CombatEffectAction {
    pub action_type: CompactString,
    pub actor_type: CompactString,
    pub args: Vec<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct SkillFeatureOption {
    #[serde(rename(deserialize = "type"))]
    pub effect_type: CompactString,
    pub level: u16,
    #[serde(rename(deserialize = "paramtype"))]
    pub param_type: CompactString,
    pub param: Vec<i32>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct EngravingData {
    pub id: u32,
    pub name: Option<CompactString>,
    pub icon: Option<CompactString>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterPreview {
    pub id: i32,
    pub fight_start: i64,
    pub boss_name: CompactString,
    pub duration: i64,
    pub classes: Vec<i32>,
    pub names: Vec<CompactString>,
    pub difficulty: Option<CompactString>,
    pub local_player: CompactString,
    pub my_dps: i64,
    pub favorite: bool,
    pub cleared: bool,
    pub spec: Option<CompactString>,
    pub support_ap: Option<f32>,
    pub support_brand: Option<f32>,
    pub support_identity: Option<f32>,
    pub support_hyper: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncountersOverview {
    pub encounters: Vec<EncounterPreview>,
    pub total_encounters: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchFilter {
    pub bosses: Vec<CompactString>,
    pub min_duration: i32,
    pub max_duration: i32,
    pub cleared: bool,
    pub favorite: bool,
    pub difficulty: CompactString,
    pub boss_only_damage: bool,
    pub sort: CompactString,
    pub order: CompactString,
    pub raids_only: bool,
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDbInfo {
    pub size: CompactString,
    pub total_encounters: i32,
    pub total_encounters_filtered: i32,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum HitOption {
    NONE,
    BACK_ATTACK,
    FRONTAL_ATTACK,
    FLANK_ATTACK,
    MAX,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum HitFlag {
    NORMAL,
    CRITICAL,
    MISS,
    INVINCIBLE,
    DOT,
    IMMUNE,
    IMMUNE_SILENCED,
    FONT_SILENCED,
    DOT_CRITICAL,
    DODGE,
    REFLECT,
    DAMAGE_SHARE,
    DODGE_HIT,
    MAX,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalInfo {
    pub client_id: CompactString,
    pub local_players: HashMap<u64, LocalPlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalPlayer {
    pub name: CompactString,
    pub count: i32,
}

fn int_or_string_as_string<'de, D>(deserializer: D) -> Result<CompactString, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(CompactString::from(s)),
        Value::Number(n) => {
            let mut s = CompactString::default();
            write!(&mut s, "{}", n).unwrap();
            Ok(s)
        },
        _ => Err(serde::de::Error::custom("Expected a string or an integer")),
    }
}
