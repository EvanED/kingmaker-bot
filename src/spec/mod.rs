use enum_map::EnumMap;
use crate::roll_result::RollResult;

pub mod attributes;
use self::attributes::Attribute;

pub mod skills;
use self::skills::{Skill, TrainingLevel};

pub type AttributeMap = EnumMap<Attribute, i8>;
pub type SkillMap = EnumMap<Skill, TrainingLevel>;
pub use enum_map::enum_map;

#[derive(Debug)]
pub struct Kingdom {
    pub attributes: AttributeMap,
    pub invested: EnumMap<Attribute, bool>,
    pub skills: EnumMap<Skill, TrainingLevel>,
    pub level: i8,
}



impl Kingdom {
    pub fn roll(&self, skill: Skill, d20: i8) -> RollResult {
        let attribute = skill.attribute();
        let attribute_mod = self.attributes[attribute];

        let invested_mod = if self.invested[attribute] {1} else {0};
        
        let proficiency = self.skills[skill].modifier(self.level);

        let roll = d20 + attribute_mod + invested_mod + proficiency;

        RollResult(roll)
    }
}