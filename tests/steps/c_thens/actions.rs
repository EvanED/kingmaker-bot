use assert2::assert;
use cucumber::then;

use crate::context::TestContext;

#[then("we collected taxes this turn")]
fn then_we_collected_taxes(world: &mut TestContext) {
    assert!(world.next_turn_state.collected_taxes);
}