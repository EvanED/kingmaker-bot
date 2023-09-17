use enum_map::EnumMap;
use crate::rolls::roll_result::{DieRoll, NaturalRoll, TotalRoll};
use crate::rolls::roll_context::RollContext;

pub mod attributes;
use self::attributes::Attribute;

pub mod skills;
use self::skills::{Skill, TrainingLevel};

pub type AttributeMap = EnumMap<Attribute, i8>;
pub type SkillMap = EnumMap<Skill, TrainingLevel>;
pub use enum_map::enum_map;

#[derive(Debug)]
pub struct Kingdom {
    pub name: String,
    pub attributes: AttributeMap,
    pub invested: EnumMap<Attribute, bool>,
    pub skills: EnumMap<Skill, TrainingLevel>,
    pub level: i8,
}

impl Kingdom {
    fn markdown_uteml(&self, skill: Skill) -> &'static str {
        match self.skills[skill] {
            TrainingLevel::Untrained => "",
            TrainingLevel::Trained   => " [T]",
            TrainingLevel::Expert    => " [E]",
            TrainingLevel::Master    => " [M]",
            TrainingLevel::Legendary => " [L]",
        }
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "
# {} (Level {})

**Attributes:** Culture +{} / Economy +{} / Loyalty +{} / Stability +{}  \n\
**Skills:**
* Culture: Arts{} / Folklore{} / Magic{} / Scholarship{}
* Economy: Boating{} / Exploration{} / Industry{} / Trade{}
* Loyalty: Intrigue{} / Politics{} / Statecraft{} / Warfare{}
* Stability: Agriculture{} / Defense{} / Engineering{} / Wilderness{}
            ",
            self.name,
            self.level,
            self.attributes[Attribute::Culture],
            self.attributes[Attribute::Economy],
            self.attributes[Attribute::Loyalty],
            self.attributes[Attribute::Stability],
            self.markdown_uteml(Skill::Arts),
            self.markdown_uteml(Skill::Folklore),
            self.markdown_uteml(Skill::Magic),
            self.markdown_uteml(Skill::Scholarship),
            self.markdown_uteml(Skill::Boating),
            self.markdown_uteml(Skill::Exploration),
            self.markdown_uteml(Skill::Industry),
            self.markdown_uteml(Skill::Trade),
            self.markdown_uteml(Skill::Intrigue),
            self.markdown_uteml(Skill::Politics),
            self.markdown_uteml(Skill::Statecraft),
            self.markdown_uteml(Skill::Warfare),
            self.markdown_uteml(Skill::Agriculture),
            self.markdown_uteml(Skill::Defense),
            self.markdown_uteml(Skill::Engineering),
            self.markdown_uteml(Skill::Wilderness),
        )
    }

    pub fn roll(&self, skill: Skill, context: &RollContext) -> DieRoll {
        let attribute = skill.attribute();
        let attribute_mod = self.attributes[attribute];

        let invested_mod = if self.invested[attribute] {1} else {0};
        
        let proficiency = self.skills[skill].modifier(self.level);

        let applicable_bonuses = &context.bonuses;
        let mut bonuses_mod: i8 = 0;
        let mut bonuses_desc = String::new();
        for bonus in applicable_bonuses.iter() {
            if bonus.applies(attribute, skill) {
                bonuses_mod += bonus.modifier;
                bonuses_desc.push_str(
                    format!(" + {} ({})", bonus.modifier, bonus.reason).as_str()
                );
            }
        }

        let natural = context.d20.roll();
        let total = TotalRoll(natural + attribute_mod + invested_mod + proficiency + bonuses_mod);

        // TODO: the "trained" should be replaced, maybe split up
        let description = format!("{natural} (nat) + {attribute_mod} (culture) + {invested_mod} (invested) + {proficiency} (training){bonuses_desc}");

        DieRoll {
            total,
            natural: NaturalRoll(natural),
            description,
        }
    }
}