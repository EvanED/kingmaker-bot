use cucumber::given;
use kingdom::bonus::{Bonus, BonusType, AppliesTo, AppliesUntil};
use kingdom::roll_context::RollContext;
use crate::context::TestContext;
use kingdom::spec::{self, enum_map};
use kingdom::spec::attributes::Attribute;
use kingdom::spec::skills::{TrainingLevel, Skill};

#[given("the kingdom Aryc at level 1")]
fn create_aryc(world: &mut TestContext) {
    use Attribute::*;
    use Skill::*;
    use TrainingLevel::*;
    let aryc = spec::Kingdom {
        level: 1,
        attributes: enum_map! {
            Culture   => 0,
            Economy   => 4,
            Loyalty   => 2,
            Stability => 2,
        },
        invested: enum_map! {
            Culture   => true,
            Economy   => true,
            Loyalty   => true,
            Stability => true,
        },
        skills: enum_map! {
            Agriculture => Untrained,
            Arts        => Trained,
            Boating     => Trained,
            Defense     => Trained,
            Engineering => Trained,
            Exploration => Untrained,
            Folklore    => Untrained,
            Industry    => Trained,
            Intrigue    => Untrained,
            Magic       => Trained,
            Politics    => Trained,
            Scholarship => Untrained,
            Statecraft  => Untrained,
            Trade       => Trained,
            Warfare     => Trained,
            Wilderness  => Untrained,
        },
    };
    world.kingdom = Some(aryc);
}

#[given(expr="a die roll of {int}")]
fn set_roll(world: &mut TestContext, d20: i32) {
    let ctx = RollContext {
        d20: d20.try_into().unwrap(),
        bonuses: Vec::new(),
    };
    world.roll_context = Some(ctx);
}

#[given(expr="a circumstance bonus of +{int} to Culture, because {string}")]
fn add_bonus(world: &mut TestContext, modifier: i32, reason: String) {
    let bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Culture),
        applies_until: AppliesUntil::NextApplicableRoll,
        modifier: modifier.try_into().unwrap(),
        reason,
    };
    world.roll_context.as_mut().unwrap().bonuses.push(bonus);
}

#[given(expr="a circumstance bonus of +{int} to Culture, lasting until the next turn, because {string}")]
fn add_longer_bonus(world: &mut TestContext, modifier: i32, reason: String) {
    let bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Culture),
        applies_until: AppliesUntil::StartOfTheNextTurn,
        modifier: modifier.try_into().unwrap(),
        reason,
    };
    world.roll_context.as_mut().unwrap().bonuses.push(bonus);
}
