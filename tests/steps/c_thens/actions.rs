use assert2::assert;
use cucumber::then;

use crate::context::TestContext;

#[then("we collected taxes this turn")]
fn then_we_collected_taxes(world: &mut TestContext) {
    assert!(world.next_turn_state.collected_taxes);
}

#[then("we did not collect taxes this turn")]
fn then_we_did_not_collect_taxes(world: &mut TestContext) {
    assert!(!world.next_turn_state.collected_taxes);
}

#[then("we traded commodities this turn")]
fn then_we_traded_commodities(world: &mut TestContext) {
    assert!(world.next_turn_state.traded_commodities);
}

#[then("the kingdom will gain one additional Fame point next turn")]
fn then_the_kingdom_will_get_one_additional_fame_point(world: &mut TestContext) {
    assert!(world.next_turn_state.additional_fame_points_scheduled == 1);
}

#[then("the Supernatural Solution fortune is available")]
fn then_the_supernatural_solution_fortune_is_available(world: &mut TestContext) {
    assert!(world.next_turn_state.supernatural_solution_available);
}

#[then("the Supernatural Solution fortune is not available")]
fn then_the_supernatural_solution_fortune_is_not_available(world: &mut TestContext) {
    assert!(!world.next_turn_state.supernatural_solution_available);
}


#[then("Supernatural Solution is not blocked")]
fn then_supernatural_solution_is_not_blocked(world: &mut TestContext) {
    assert!(world.next_turn_state.supernatural_solution_blocked_for_x_turns.is_none());
}

#[then("Supernatural Solution is blocked for two turns")]
fn then_supernatural_solution_is_blocked(world: &mut TestContext) {
    assert!(world.next_turn_state.supernatural_solution_blocked_for_x_turns == Some(2));
}
