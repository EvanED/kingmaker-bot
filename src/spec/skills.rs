use enum_map::Enum;
use poise::ChoiceParameter;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumString, IntoStaticStr, EnumIter, EnumCount};
use super::attributes::Attribute;

// TODO: Can we remove Default?
#[derive(Debug, Clone, Copy, EnumIter, EnumCount, Serialize, Deserialize, Default, ChoiceParameter)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum TrainingLevel {
    #[default] Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl TrainingLevel {
    pub fn modifier(self, kingdom_level: i8) -> i8 {
        use TrainingLevel::*;
        match self {
            Untrained => 0,
            Trained   => 2 + kingdom_level,
            Expert    => 4 + kingdom_level,
            Master    => 6 + kingdom_level,
            Legendary => 8 + kingdom_level,
        }
    }
}

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, IntoStaticStr, EnumString, EnumIter, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Skill {
    // Culture
    Arts,
    Folklore,
    Magic,
    Scholarship,
    
    // Economy
    Boating,
    Exploration,
    Industry,
    Trade,
    
    // Loyalty
    Intrigue,
    Politics,
    Statecraft,
    Warfare,
    
    // Stability
    Agriculture,
    Defense,
    Engineering,
    Wilderness,
}

impl Skill {
    pub fn autocomplete_matches(self, prompt: &str) -> bool {
        self.to_markdown().contains(prompt)
    }

    pub fn autocomplete_matching(prompt: &str) -> Vec<&'static str> {
        Skill::iter()
            .filter(|skill| skill.autocomplete_matches(prompt))
            .map(|skill| skill.to_markdown())
            .collect()
    }

    pub fn to_markdown(self) -> &'static str {
        self.into()
    }

    pub fn attribute(self) -> Attribute {
        use Attribute::*;
        use Skill::*;
        match self {
            Arts        => Culture,
            Folklore    => Culture,
            Magic       => Culture,
            Scholarship => Culture,
            
            Boating     => Economy,
            Exploration => Economy,
            Industry    => Economy,
            Trade       => Economy,
            
            Intrigue    => Loyalty,
            Politics    => Loyalty,
            Statecraft  => Loyalty,
            Warfare     => Loyalty,
            
            Agriculture => Stability,
            Defense     => Stability,
            Engineering => Stability,
            Wilderness  => Stability,      
        }
    }
}

#[cfg(test)]
pub mod tests {
    use assert2::assert;
    #[test]
    fn foo() {
        assert!(1 == 1);
    }
}