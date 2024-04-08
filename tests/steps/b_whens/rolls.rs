use cucumber::when;
use kingdom::{rolls::bonus::{self, filter_from_roll}, spec::{attributes::Attribute, skills::Skill}};
use crate::context::TestContext;

#[when("I roll Arts")]
fn roll_arts(world: &mut TestContext) {
    let die_roll = world.kingdom.as_ref().unwrap()
        .roll(&world.kingdom_state, bonus::KingdomAction::UNSPECIFIED, Skill::Arts, world.roll_context.as_ref().unwrap());
    world.die_roll = Some(die_roll);

    world.remaining_bonuses = filter_from_roll(
        bonus::KingdomAction::UNSPECIFIED,
        &world.roll_context.as_ref().unwrap().bonuses,
        Attribute::Culture,
        Skill::Arts,
    );
}

#[when("I roll Trade")]
fn roll_trade(world: &mut TestContext) {
    let die_roll = world.kingdom.as_ref().unwrap()
        .roll(&world.kingdom_state, bonus::KingdomAction::UNSPECIFIED, Skill::Trade, world.roll_context.as_ref().unwrap());
    world.die_roll = Some(die_roll);

    world.remaining_bonuses = filter_from_roll(
        bonus::KingdomAction::UNSPECIFIED,
        &world.roll_context.as_ref().unwrap().bonuses,
        Attribute::Economy,
        Skill::Trade,
    );
}
