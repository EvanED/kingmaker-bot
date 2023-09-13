use cucumber::then;
use assert2::assert;
use kingdom::state::Commodity;
use crate::context::TestContext;

#[then(expr = "Unrest is still {int}")]
#[then(expr = "Unrest went up to {int}")]
#[then(expr = "Unrest went down to {int}")]
fn then_unrest_is_n(world: &mut TestContext, unrest: i32) {
    assert!(world.kingdom_state.unrest as i32 == unrest);
}

#[then(expr = "RP is still {int}")]
#[then(expr = "RP went up to {int}")]
#[then(expr = "RP went down to {int}")]
fn then_rp_is_n(world: &mut TestContext, rp: i32) {
    assert!(world.kingdom_state.resource_points as i32 == rp);
}

#[then("we are required to increase any Ruin")]
fn then_we_are_required_to_increase_any_ruin(world: &mut TestContext) {
    assert!(world.next_turn_state.requirements == vec!["increase any Ruin"]);
}

#[then(expr = "next turn will have {int} bonus RP")]
fn given_next_turn_will_have_x_bonus_rp(world: &mut TestContext, volume: i32) {
    assert!(volume == world.next_turn_state.bonus_rp as i32);
}

#[then(expr = "I have {int} Lumber")]
fn given_next_turn_will_have_x_lumber(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Lumber] as i32);
}

#[then(expr = "I have {int} Food")]
fn given_next_turn_will_have_x_food(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Food] as i32);
}

#[then(expr = "I have {int} Luxuries")]
fn given_next_turn_will_have_x_luxuries(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Luxuries] as i32);
}
