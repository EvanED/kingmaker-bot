use enum_map::EnumMap;
use crate::roll_result::RollResult;
use crate::roll_context::RollContext;

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
    pub fn roll(&self, skill: Skill, context: RollContext) -> RollResult {
        let attribute = skill.attribute();
        let attribute_mod = self.attributes[attribute];

        let invested_mod = if self.invested[attribute] {1} else {0};
        
        let proficiency = self.skills[skill].modifier(self.level);

        let natural = context.d20;
        let total = natural + attribute_mod + invested_mod + proficiency;

        // TODO: the "trained" should be replaced, maybe split up
        let description = format!("{natural} (nat) + {attribute_mod} (culture) + {invested_mod} (invested) + {proficiency} (arts: trained)");

        RollResult { total, natural, description }
    }
}