use cucumber::when;
use kingdom::{actions::{b_commerce::{collect_taxes, improve_lifestyle, trade_commodities}, c1_leadership::{take_charge, purchase_commodities, celebrate_holiday, supernatural_solution, prognostication}}, state::Commodity, spec::skills::Skill};

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

#[when(expr = "I Take Charge with Arts")]
fn when_i_take_charge_arts(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = take_charge::take_charge(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Skill::Arts,
    );
}

#[when("I Purchase Commodities of Food (secondary Lumber)")]
fn when_i_purchase_commodities(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = purchase_commodities::purchase_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Food,
        Commodity::Lumber,
    );
}

#[when("I Purchase Commodities of Luxuries (secondary Lumber)")]
fn when_i_purchase_commodities2(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = purchase_commodities::purchase_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Luxuries,
        Commodity::Lumber,
    );
}

#[when("I Celebrate Holiday")]
fn when_i_celebrate_holiday(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = celebrate_holiday::celebrate_holiday(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
}

#[when("I search for a Supernatural Solution")]
fn when_i_search_for_a_supernatural_solution(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = supernatural_solution::supernatural_solution(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
}

#[when("I Prognosticate")]
fn when_i_prognosticate(world: &mut TestContext) {
    (world.next_turn_state, world.kingdom_state) = prognostication::prognosticate(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
}
