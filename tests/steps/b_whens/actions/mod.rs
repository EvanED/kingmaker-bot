use cucumber::when;
use kingdom::actions::collect_taxes;

use crate::context::TestContext;

#[when("I collect taxes")]
fn when_i_collect_taxes(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = collect_taxes::collect_taxes(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap()
    );
}