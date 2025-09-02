#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use hashbrown::{HashMap, HashSet};

use crate::models::*;

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
            combat_effect_data: serde_json::from_str(include_str!("../meter-data/CombatEffect.json")).unwrap(),
            engraving_data: serde_json::from_str(include_str!("../meter-data/Ability.json")).unwrap(),
            skill_buff_data: serde_json::from_str(include_str!("../meter-data/SkillBuff.json")).unwrap(),
            skill_data: serde_json::from_str(include_str!("../meter-data/Skill.json")).unwrap(),
            skill_effect_data: serde_json::from_str(include_str!("../meter-data/SkillEffect.json")).unwrap(),
            stat_type_map: serde_json::from_str(include_str!("../meter-data/StatType.json")).unwrap(),
            esther_data: serde_json::from_str(include_str!("../meter-data/Esther.json")).unwrap(),
            npc_data: serde_json::from_str(include_str!("../meter-data/Npc.json")).unwrap(),
            gem_skill_map: {
                let raw_map: HashMap<String, (String, String, Vec<u32>)> =
                    serde_json::from_str(include_str!("../meter-data/GemSkillGroup.json")).unwrap();
                raw_map
                    .into_iter()
                    .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
                    .collect()
            },
            raid_map: {
                let encounters: HashMap<String, HashMap<String, Vec<String>>> =
                    serde_json::from_str(include_str!("../meter-data/encounters.json")).unwrap();
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
