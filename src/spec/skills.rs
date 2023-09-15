use enum_map::Enum;
use strum_macros::EnumString;
use super::attributes::Attribute;

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum TrainingLevel {
    Untrained,
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

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, EnumString)]
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