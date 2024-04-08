use poise::ChoiceParameter;
use serde::{Serialize, Deserialize};
use strum_macros::AsRefStr;
use std::{fmt::Write, str::FromStr};
use strum_macros::EnumString;

use crate::{spec::{attributes, skills, Kingdom}, Markdownable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, Serialize, Deserialize, ChoiceParameter)]
pub enum BonusType {
    Circumstance,
    Item,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum KingdomAction {
    BuildRoads,
    BuildStructure,
    CelebrateHoliday,
    ClaimHex,
    CollectTaxes,
    CreateAMasterpiece,
    EstablishFarmland,
    GoFishing,
    ImproveLifestyle,
    Prognostication,
    PurchaseCommodities,
    SupernaturalSolution,
    TakeCharge,
    TradeCommodities,
    UNSPECIFIED,
}

impl KingdomAction {
    fn to_markdown(self) -> &'static str {
        use KingdomAction::*;
        match self {
            BuildRoads           => "Build Roads action",
            BuildStructure       => "Build Structure action",
            CelebrateHoliday     => "celebrate Holiday action",
            ClaimHex             => "Claim Hex action",
            CollectTaxes         => "Collect Taxes action",
            CreateAMasterpiece   => "Create A Masterpiece action",
            EstablishFarmland    => "Establish Farmland action",
            GoFishing            => "Go Fishing action",
            ImproveLifestyle     => "Improve Lifestyle action",
            Prognostication      => "Prognostication action",
            PurchaseCommodities  => "Purchase Commodities action",
            SupernaturalSolution => "Supernatural Solution action",
            TakeCharge           => "Take Charge action",
            TradeCommodities     => "Trade Commodities action",
            UNSPECIFIED          => "<Unspecified kingdom action",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppliesTo {
    Everything,
    Attribute(attributes::Attribute),
    Skill(skills::Skill),
    KingdomAction(KingdomAction),
    RandomEventResolutions,
}

impl AppliesTo {
    fn to_markdown(self) -> &'static str {
        match self {
            AppliesTo::Everything              => "everything",
            AppliesTo::Attribute(a) => a.to_markdown(),
            AppliesTo::Skill(s)         => s.to_markdown(),
            AppliesTo::RandomEventResolutions  => "random event rolls",
            AppliesTo::KingdomAction(a) => a.to_markdown(),
        }
    }

    pub fn from_string(s: &str) -> Option<AppliesTo> {
        let x = attributes::Attribute::from_str(s);
        if x.is_ok() {
            return Some(AppliesTo::Attribute(x.unwrap()))
        }

        let x = skills::Skill::from_str(s);
        if x.is_ok() {
            return Some(AppliesTo::Skill(x.unwrap()))
        }

        let x = KingdomAction::from_str(s);
        if x.is_ok() {
            return Some(AppliesTo::KingdomAction(x.unwrap()))
        }

        if s == "RandomEventResolutions" {
            return Some(AppliesTo::RandomEventResolutions)
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ChoiceParameter)]
pub enum AppliesUntil {
    NextApplicableRoll,
    StartOfTheNextTurn,
    EndOfTheNextTurn,
    Forever,
}

impl AppliesUntil {
    fn to_markdown(self) -> &'static str {
        match self {
            AppliesUntil::NextApplicableRoll => "the next such roll",
            AppliesUntil::StartOfTheNextTurn => "the start of the next kingdom turn",
            AppliesUntil::EndOfTheNextTurn   => "the end of the next kingdom turn",
            AppliesUntil::Forever            => "the end of the campaign",
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

    pub fn applies(&self, attribute: attributes::Attribute, skill: skills::Skill, action: KingdomAction) -> bool {
        match self.applies_to {
            AppliesTo::Everything              => true,
            AppliesTo::Attribute(a) => a == attribute,
            AppliesTo::Skill(s)         => s == skill,
            AppliesTo::KingdomAction(a) => a == action, // FIXME... what?
            AppliesTo::RandomEventResolutions  => false,            // TODO... what?
        }
    }

    pub fn expires_from_roll(&self, attribute: attributes::Attribute, skill: skills::Skill, action: KingdomAction) -> bool {
        match self.applies_until {
            AppliesUntil::NextApplicableRoll => self.applies(attribute, skill, action),
            AppliesUntil::StartOfTheNextTurn => false,
            AppliesUntil::EndOfTheNextTurn   => false,
            AppliesUntil::Forever            => false,
        }
    }

    pub fn transform_at_turn_start(&self) -> Option<Bonus> {
        match self.applies_until {
            AppliesUntil::NextApplicableRoll => None,
            AppliesUntil::StartOfTheNextTurn => None,
            AppliesUntil::EndOfTheNextTurn   => Some(Bonus{
                applies_until: AppliesUntil::StartOfTheNextTurn,
                reason: self.reason.clone(),
                ..*self
            }),
            AppliesUntil::Forever            => Some(self.clone()),
        }
    }
}

pub fn filter_from_roll(action: KingdomAction, bonuses: &Vec<Bonus>, attribute: attributes::Attribute, skill: skills::Skill) -> Vec<Bonus> {
    bonuses.iter()
        .filter(
            |bonus| !bonus.expires_from_roll(attribute, skill, action)
        )
        .map(|bonus| bonus.clone())
        .collect()
}
