use cucumber::when;
use kingdom::{spec::{skills::Skill, attributes::Attribute}, bonus::filter_from_roll};
use crate::context::TestContext;

#[when("I roll Arts")]
fn roll_arts(world: &mut TestContext) {
    let roll_result = world.kingdom.as_ref().unwrap()
        .roll(Skill::Arts, world.roll_context.as_ref().unwrap());
    world.roll_result = Some(roll_result);

    world.remaining_bonuses = filter_from_roll(
        &world.roll_context.as_ref().unwrap().bonuses,
        Attribute::Culture,
        Skill::Arts,
    );
}

#[when("I roll Trade")]
fn roll_trade(world: &mut TestContext) {
    let roll_result = world.kingdom.as_ref().unwrap()
        .roll(Skill::Trade, world.roll_context.as_ref().unwrap());
    world.roll_result = Some(roll_result);

    world.remaining_bonuses = filter_from_roll(
        &world.roll_context.as_ref().unwrap().bonuses,
        Attribute::Economy,
        Skill::Trade,
    );
}
