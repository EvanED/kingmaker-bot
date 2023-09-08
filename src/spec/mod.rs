use enum_map::{EnumMap, Enum};

#[derive(Debug)]
pub struct RollResult(pub i8);

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

#[derive(Debug, Enum, Clone, Copy)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

#[derive(Debug, Clone, Copy)]
pub enum TrainingLevel {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl TrainingLevel {
    fn modifier(self, kingdom_level: i8) -> i8 {
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

#[derive(Debug, Enum, Clone, Copy)]
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
    fn attribute(self) -> Attribute {
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

impl Kingdom {
    pub fn roll(&self, skill: Skill, d20: i8) -> RollResult {
        let attribute = skill.attribute();
        let attribute_mod = self.attributes[attribute];

        let invested_mod = if self.invested[attribute] {1} else {0};
        
        let proficiency = self.skills[skill].modifier(self.level);

        println!("Roll: {d20} + {attribute_mod} + {proficiency}");
        
        let roll = d20 + attribute_mod + invested_mod + proficiency;

        RollResult(roll)
    }
}