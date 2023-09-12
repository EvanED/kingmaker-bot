use cucumber::given;
use kingdom::state::Commodity;

use crate::context::TestContext;

#[given(expr = "we have {int} Unrest")]
fn given_we_have_x_unrest(world: &mut TestContext, unrest: i32) {
    world.kingdom_state.unrest = unrest as i8;
}

#[given(expr = "we have {int} Lumber")]
fn given_we_have_n_lumber(world: &mut TestContext, lumber: i32) {
    world.kingdom_state.commodity_stores[Commodity::Lumber] = lumber as i8;
}

#[given(expr = "next turn will have {int} bonus RP")]
fn given_next_turn_will_have_x_bonus_rp(world: &mut TestContext, volume: i32) {
    world.next_turn_state.bonus_rp = volume as i8;
}
