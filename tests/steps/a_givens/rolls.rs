use cucumber::given;
use kingdom::rolls::bonus::{Bonus, BonusType, AppliesTo, AppliesUntil};
use kingdom::rolls::roll_context::RollContext;
use crate::context::TestContext;
use kingdom::spec::attributes::Attribute;

fn set_roll_context(world: &mut TestContext) {
    let ctx = RollContext {
        d4: -10,
        d6: -10,
        d20: -10,
        bonuses: Vec::new(),
    };
    world.roll_context = Some(ctx);
}

#[given(expr="a die roll of {int}")]
fn set_roll(world: &mut TestContext, d20: i32) {
    if world.roll_context.is_none() {
        set_roll_context(world);
    }

    world.roll_context.as_mut().unwrap().d20 = d20 as i8;
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

#[given(expr="the next d4 rolls are {int}")]
fn given_next_d4s(world: &mut TestContext, d4: i32) {
    if world.roll_context.is_none() {
        set_roll_context(world);
    }
    world.roll_context.as_mut().unwrap().d4 = d4 as i8;
}

#[given(expr="the next d6 rolls are {int}")]
fn given_next_d6s(world: &mut TestContext, d6: i32) {
    if world.roll_context.is_none() {
        set_roll_context(world);
    }
    world.roll_context.as_mut().unwrap().d6 = d6 as i8;
}