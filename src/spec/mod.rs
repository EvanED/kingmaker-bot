use std::fmt;
use std::marker::PhantomData;

use enum_map::{EnumMap, EnumArray};
use serde::de::{Visitor, MapAccess};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use crate::rolls::bonus::{self, KingdomAction};
use crate::rolls::roll_result::{DieRoll, NaturalRoll, TotalRoll};
use crate::rolls::roll_context::RollContext;
use crate::state::KingdomState;
use crate::Markdownable;

pub mod attributes;
use self::attributes::Attribute;

pub mod skills;
use self::skills::{Skill, TrainingLevel};

pub mod terrain;

pub type AttributeMap = EnumMap<Attribute, i8>;
pub type SkillMap = EnumMap<Skill, TrainingLevel>;
pub use enum_map::enum_map;

struct MyMapVisitor<K: IntoEnumIterator + EnumArray<V>, V: Default> {
    marker: PhantomData<fn() -> EnumMap<K, V>>
}

impl<K: IntoEnumIterator + EnumArray<V>, V: Default> MyMapVisitor<K, V> {
    fn new() -> Self {
        MyMapVisitor {
            marker: PhantomData
        }
    }
}

impl<'de, K: IntoEnumIterator + EnumArray<V>, V: Default> Visitor<'de> for MyMapVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    // The type that our Visitor is going to produce.
    type Value = EnumMap<K, V>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an EnumMap<K, V>")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = EnumMap::<K, V>::default();
        while let Some((key, value)) = access.next_entry()? {
            map[key] = value;
        }
        Ok(map)
    }
}

pub mod enum_map_serde {
    use enum_map::{EnumMap, EnumArray};
    use serde::{Serializer, Serialize, Deserialize};
    use serde::ser::SerializeMap;
    use strum::IntoEnumIterator;
    use serde::de::Deserializer;
    use super::MyMapVisitor;

    pub fn serialize<K: Copy + Serialize + IntoEnumIterator + EnumArray<V>, V: Copy + Serialize, S>(map: &EnumMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let count = K::iter().count();
        let mut smap = serializer.serialize_map(Some(count))?;
        for skill in K::iter() {
            let value = map[skill];
            smap.serialize_entry(&skill, &value)?;
        }
        smap.end()
    }

    pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<EnumMap<K, V>, D::Error>
    where
        K: Copy + EnumArray<V> + IntoEnumIterator + Deserialize<'de>,
        V: Default + Deserialize<'de>,
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(MyMapVisitor::new())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kingdom {
    pub name: String,
    #[serde(with="enum_map_serde")]
    pub attributes: AttributeMap,
    #[serde(with="enum_map_serde")]
    pub invested: EnumMap<Attribute, bool>,
    #[serde(with="enum_map_serde")]
    pub skills: EnumMap<Skill, TrainingLevel>,
    pub level: i8,
}

fn make_unrest_penalty(modifier: i8, label: &str) -> bonus::Bonus {
    bonus::Bonus {
        type_: bonus::BonusType::Status,
        applies_to: bonus::AppliesTo::Everything,
        applies_until: bonus::AppliesUntil::NextApplicableRoll,
        modifier,
        reason: format!("{label} unrest"),
    }
}

fn get_unrest_penalties(unrest: i8) -> Vec<bonus::Bonus> {
    match unrest {
        ..=-1   => panic!("Bad unrest value"),
        0       => vec![],
        1..=4   => vec![make_unrest_penalty(-1, "minor")],
        5..=9   => vec![make_unrest_penalty(-2, "moderate")],
        10..=15 => vec![make_unrest_penalty(-3, "major")],
        16..    => vec![make_unrest_penalty(-4, "severe")],
    }
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

    pub fn roll(&self, state: &KingdomState, action: KingdomAction, skill: Skill, context: &RollContext) -> DieRoll {
        println!("roll({action:?}, ...)");
        for b in &context.bonuses {
            println!("  {}", b.to_markdown());
        }
        let attribute = skill.attribute();
        let attribute_mod = self.attributes[attribute];

        let invested_mod = if self.invested[attribute] {1} else {0};
        
        let proficiency = self.skills[skill].modifier(self.level);

        let applicable_bonuses = &context.bonuses;
        let unrest_penalties = get_unrest_penalties(state.unrest);
        let mut bonuses_mod: i8 = 0;
        let mut bonuses_desc = String::new();
        for bonus in applicable_bonuses.iter().chain(unrest_penalties.iter()) {
            if bonus.applies(attribute, skill, action) {
                bonuses_mod += bonus.modifier;
                bonuses_desc.push_str(
                    format!(" + {} ({})", bonus.modifier, bonus.reason).as_str()
                );
            }
        }

        let natural = context.d20.roll();
        let total = TotalRoll(natural + attribute_mod + invested_mod + proficiency + bonuses_mod);

        // TODO: the "trained" should be replaced, maybe split up
        let description = format!("{natural} (nat) + {attribute_mod} ({attribute:?}) + {invested_mod} (invested) + {proficiency} (training){bonuses_desc}");

        DieRoll {
            total,
            natural: NaturalRoll(natural),
            description,
        }
    }
}