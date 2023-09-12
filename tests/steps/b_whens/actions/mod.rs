use cucumber::when;
use kingdom::{actions::b_commerce::{collect_taxes, improve_lifestyle, trade_commodities}, state::Commodity};

use crate::context::TestContext;

#[when("I collect taxes")]
fn when_i_collect_taxes(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = collect_taxes::collect_taxes(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
}

#[when("I do not collect taxes")]
fn when_i_do_not_collect_taxes(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = collect_taxes::decline_to_collect(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    )
}

#[when("I Improve Lifestyle")]
fn when_i_improve_lifestyle(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = improve_lifestyle::improve_lifestyle(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
}

#[when(expr = "I Trade Commodities of {int} Lumber")]
fn when_i_trade_commodities(world: &mut TestContext, volume: i32) {
    (world.next_turn_state, world.kingdom_state) = trade_commodities::trade_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Lumber,
        volume as i8,
    );
}
