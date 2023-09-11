use cucumber::then;
use assert2::assert;
use crate::context::TestContext;

#[then(expr = "Unrest is still {int}")]
#[then(expr = "Unrest went up to {int}")]
fn then_unrest_is_n(world: &mut TestContext, unrest: i32) {
    assert!(world.kingdom_state.unrest as i32 == unrest);
}

#[then("we are required to increase any Ruin")]
fn then_we_are_required_to_increase_any_ruin(world: &mut TestContext) {
    assert!(world.next_turn_state.requirements == vec!["increase any Ruin"]);
}