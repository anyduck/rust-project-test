#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use serde::de::DeserializeOwned;
use hashbrown::{HashMap, HashSet};

use crate::models::*;

fn load_json<T: DeserializeOwned>(path: &str) -> T {
    let string = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&string).unwrap()
}

pub struct AssetPreloader {
    pub combat_effect_data: HashMap<i32, CombatEffectData>,
    pub engraving_data: HashMap<u32, EngravingData>,
    pub skill_buff_data: HashMap<u32, SkillBuffData>,
    pub skill_data: HashMap<u32, SkillData>,
    pub skill_effect_data: HashMap<u32, SkillEffectData>,
    pub support_ap_group: HashSet<u32>,
    pub support_identity_group: HashSet<u32>,
    pub stat_type_map: HashMap<String, u32>,
    pub esther_data: Vec<Esther>,
    pub npc_data: HashMap<u32, Npc>,
    pub gem_skill_map: HashMap<u32, Vec<u32>>,
    pub raid_map: HashMap<String, String>,
}

impl AssetPreloader {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            combat_effect_data: load_json("meter-data/CombatEffect.json"),
            engraving_data: load_json("meter-data/Ability.json"),
            skill_buff_data: load_json("meter-data/SkillBuff.json"),
            skill_data: load_json("meter-data/Skill.json"),
            skill_effect_data: load_json("meter-data/SkillEffect.json"),
            stat_type_map: load_json("meter-data/StatType.json"),
            esther_data: load_json("meter-data/Esther.json"),
            npc_data: load_json("meter-data/Npc.json"),
            gem_skill_map: {
                let raw_map: HashMap<String, (String, String, Vec<u32>)> =
                    load_json("meter-data/GemSkillGroup.json");
                raw_map
                    .into_iter()
                    .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
                    .collect()
            },
            raid_map: {
                let encounters: HashMap<String, HashMap<String, Vec<String>>> =
                    load_json("meter-data/encounters.json");
                encounters
                    .values()
                    .flat_map(|raid| raid.iter())
                    .flat_map(|(gate, bosses)| bosses.iter().map(move |boss| (boss.clone(), gate.clone())))
                    .collect()
            },
            support_ap_group: HashSet::from([101204, 101105, 314004, 480030]),
            support_identity_group: HashSet::from([211400, 368000, 310501, 480018]),
        })
    }
}
