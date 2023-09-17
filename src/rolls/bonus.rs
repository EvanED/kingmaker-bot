use serde::{Serialize, Deserialize};
use strum_macros::AsRefStr;
use std::fmt::Write;

use crate::{spec::{attributes, skills}, Markdownable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, Serialize, Deserialize)]
pub enum BonusType {
    Circumstance,
    Item,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppliesTo {
    Attribute(attributes::Attribute),
    Skill(skills::Skill),
    RandomEventResolutions,
}

impl AppliesTo {
    fn to_markdown(self) -> &'static str {
        match self {
            AppliesTo::Attribute(a) => a.to_markdown(),
            AppliesTo::Skill(s)         => s.to_markdown(),
            AppliesTo::RandomEventResolutions  => "random event rolls",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppliesUntil {
    NextApplicableRoll,
    StartOfTheNextTurn,
    EndOfTheNextTurn,
}

impl AppliesUntil {
    fn to_markdown(self) -> &'static str {
        match self {
            AppliesUntil::NextApplicableRoll => "the next such roll",
            AppliesUntil::StartOfTheNextTurn => "the start of the next kingdom turn",
            AppliesUntil::EndOfTheNextTurn   => "the end of the next kingdom turn",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bonus {
    pub type_: BonusType,
    pub applies_to: AppliesTo,
    pub applies_until: AppliesUntil,
    pub modifier: i8,
    pub reason: String,
}

impl Markdownable for Bonus {
    fn to_markdown(&self) -> String {
        let mut s = String::new();
        self.append_markdown(&mut s);
        s
    }
}

impl Bonus {
    pub fn append_markdown(&self, app_string: &mut String) {
        write!(
            app_string,
            //+1 circ bonus to arts rolls ...
            //1 2  3  4     5              6           7
            "{}{} {} {} to {} rolls until {}, because {}",
            /*1*/ if self.modifier >= 0 {"+"} else {""},
            /*2*/ self.modifier,
            /*3*/ self.type_.as_ref(),
            /*4*/ if self.modifier >= 0 {"bonus"} else {"penalty"},
            /*5*/ self.applies_to.to_markdown(),
            /*6*/ self.applies_until.to_markdown(),
            /*7*/ self.reason,
        ).unwrap();
    }

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
