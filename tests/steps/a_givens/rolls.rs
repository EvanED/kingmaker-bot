use cucumber::given;
use kingdom::rolls::bonus::{Bonus, BonusType, AppliesTo, AppliesUntil};
use kingdom::rolls::roll_context::RollContext;
use crate::context::TestContext;
use kingdom::spec::attributes::Attribute;

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
