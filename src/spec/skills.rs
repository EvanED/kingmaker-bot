use enum_map::Enum;
use poise::ChoiceParameter;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};
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
    fn untrained_improvisation_modifier(kingdom_level: i8) -> i8 {
        match kingdom_level {
            1..=6  => kingdom_level / 2,
            7..=20 => kingdom_level,
            _      => panic!("bad kingdom_level"),
        }
    }
    pub fn modifier(self, kingdom_level: i8) -> i8 {
        use TrainingLevel::*;
        match self {
            Untrained => TrainingLevel::untrained_improvisation_modifier(kingdom_level),
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

    use super::TrainingLevel;
    #[test]
    fn test_trained_proficiencies_for_level_1_are_correct() {
        assert!(TrainingLevel::Trained  .modifier(1) == 1 + 2);
        assert!(TrainingLevel::Expert   .modifier(1) == 1 + 4);
        assert!(TrainingLevel::Master   .modifier(1) == 1 + 6);
        assert!(TrainingLevel::Legendary.modifier(1) == 1 + 8);
    }

    #[test]
    fn test_trained_proficiencies_for_level_20_are_correct() {
        assert!(TrainingLevel::Trained  .modifier(20) == 20 + 2);
        assert!(TrainingLevel::Expert   .modifier(20) == 20 + 4);
        assert!(TrainingLevel::Master   .modifier(20) == 20 + 6);
        assert!(TrainingLevel::Legendary.modifier(20) == 20 + 8);
    }

    #[test]
    fn test_untrained_improvisation_applies_at_half_level() {
        let untrained = TrainingLevel::Untrained;
        assert!(untrained.modifier(1) == 0);
        assert!(untrained.modifier(2) == 1);
        assert!(untrained.modifier(3) == 1);
        assert!(untrained.modifier(4) == 2);
        assert!(untrained.modifier(5) == 2);
        assert!(untrained.modifier(6) == 3);
    }

    #[test]
    fn test_untrained_improvisation_applies_at_full_level() {
        let untrained = TrainingLevel::Untrained;
        assert!(untrained.modifier(7)  ==  7);
        assert!(untrained.modifier(20) == 20);
    }
}