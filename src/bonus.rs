use crate::spec::{attributes, skills};

#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Circumstance,
    Item,
    Status,
}

#[derive(Debug, Clone, Copy)]
pub enum AppliesTo {
    Attribute(attributes::Attribute),
    Skill(skills::Skill),
}

#[derive(Debug)]
pub struct Bonus {
    pub type_: BonusType,
    pub applies_to: AppliesTo,
    pub modifier: i8,
    pub reason: String,
}

impl Bonus {
    pub fn applies(&self, attribute: attributes::Attribute, skill: skills::Skill) -> bool {
        match self.applies_to {
            AppliesTo::Attribute(a) => a == attribute,
            AppliesTo::Skill(s) => s == skill,
        }
    }
}