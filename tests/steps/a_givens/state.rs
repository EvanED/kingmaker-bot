use cucumber::given;
use kingdom::state::KingdomState;

use crate::context::TestContext;

#[given(expr = "we have {int} Unrest")]
fn given_we_have_x_unrest(world: &mut TestContext, unrest: i32) {
    world.kingdom_state = KingdomState {
        unrest: unrest as i8,
    }
}
