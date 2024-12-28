use cucumber::given;
use kingdom::state::Commodity;

use crate::context::TestContext;

#[given(expr = "we have {int} Unrest")]
fn given_we_have_x_unrest(world: &mut TestContext, unrest: i32) {
    world.kingdom_state.unrest = unrest as i8;
}

#[given(expr = "we have {int} Lumber")]
#[given(expr = "the kingdom has {int} Lumber")]
fn given_we_have_n_lumber(world: &mut TestContext, lumber: i32) {
    world.kingdom_state.commodity_stores[Commodity::Lumber] = lumber as i8;
}

#[given(expr = "the kingdom has {int} Food")]
fn given_we_have_n_food(world: &mut TestContext, food: i32) {
    world.kingdom_state.commodity_stores[Commodity::Food] = food as i8;
}

#[given(expr = "the kingdom has {int} Ore")]
fn given_we_have_n_stone(world: &mut TestContext, ore: i32) {
    world.kingdom_state.commodity_stores[Commodity::Ore] = ore as i8;
}

#[given(expr = "the kingdom has {int} Stone")]
fn given_we_have_n_ore(world: &mut TestContext, stone: i32) {
    world.kingdom_state.commodity_stores[Commodity::Stone] = stone as i8;
}

#[given(expr = "the kingdom has {int} Luxury")]
fn given_we_have_n_luxury(world: &mut TestContext, luxury: i32) {
    world.kingdom_state.commodity_stores[Commodity::Luxuries] = luxury as i8;
}

#[given(expr = "next turn will have {int} bonus RP")]
fn given_next_turn_will_have_x_bonus_rp(world: &mut TestContext, volume: i32) {
    world.turn_state.bonus_rp = volume as i8;
}

#[given(expr = "the kingdom has {int} RP")]
fn given_we_have_x_rp(world: &mut TestContext, rp: i32) {
    world.kingdom_state.resource_points = rp as i8;
}

#[given(expr = "the kingdom has {int} Fame point")]
#[given(expr = "the kingdom has {int} Fame points")]
fn given_we_have_x_fame(world: &mut TestContext, fame: i32) {
    world.kingdom_state.fame_points = fame as i8;
}

#[given(expr="the kingdom has {int} XP")]
fn given_we_have_x_xp(world: &mut TestContext, xp: i32) {
    world.kingdom_state.xp = xp as i16;
}