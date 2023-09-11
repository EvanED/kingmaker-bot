use cucumber::then;
use assert2::assert;
use crate::context::TestContext;

#[then(expr = "Unrest is still {int}")]
#[then(expr = "Unrest went up to {int}")]
fn then_unrest_is_n(world: &mut TestContext, unrest: i32) {
    assert!(world.kingdom_state.unrest as i32 == unrest);
}
