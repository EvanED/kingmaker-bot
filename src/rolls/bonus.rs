use crate::spec::{attributes, skills};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusType {
    Circumstance,
    Item,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppliesTo {
    Attribute(attributes::Attribute),
    Skill(skills::Skill),
    RandomEventResolutions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppliesUntil {
    NextApplicableRoll,
    StartOfTheNextTurn,
    EndOfTheNextTurn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bonus {
    pub type_: BonusType,
    pub applies_to: AppliesTo,
    pub applies_until: AppliesUntil,
    pub modifier: i8,
    pub reason: String,
}

impl Bonus {
    pub fn applies(&self, attribute: attributes::Attribute, skill: skills::Skill) -> bool {
        match self.applies_to {
            AppliesTo::Attribute(a) => a == attribute,
            AppliesTo::Skill(s)         => s == skill,
            AppliesTo::RandomEventResolutions  => false,            // TODO... what?
        }
    }

    pub fn expiring(&self, attribute: attributes::Attribute, skill: skills::Skill) -> bool {
        match self.applies_until {
            AppliesUntil::NextApplicableRoll => self.applies(attribute, skill),
            AppliesUntil::StartOfTheNextTurn => false,
            AppliesUntil::EndOfTheNextTurn   => false,
        }
    }
}

pub fn filter_from_roll(bonuses: &Vec<Bonus>, attribute: attributes::Attribute, skill: skills::Skill) -> Vec<Bonus> {
    bonuses.iter()
        .filter(
            |bonus| !bonus.expiring(attribute, skill)
        )
        .map(|bonus| bonus.clone())
        .collect()
}
