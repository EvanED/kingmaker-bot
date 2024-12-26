use cucumber::when;
use kingdom::{actions::{b_commerce::{collect_taxes, improve_lifestyle, trade_commodities}, c1_leadership::{celebrate_holiday, create_a_masterpiece, prognostication, purchase_commodities, supernatural_solution, take_charge}, c2_region::{build_roads, claim_hex::{self, ClaimHexSkill}, establish_farmland, establish_work_site, go_fishing}, c3_civic::build_structure}, spec::{skills::Skill, terrain}, state::Commodity};

use crate::context::TestContext;

#[when("I collect taxes")]
fn when_i_collect_taxes(world: &mut TestContext) {
    let triple = collect_taxes::collect_taxes(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I do not collect taxes")]
fn when_i_do_not_collect_taxes(world: &mut TestContext) {
    let triple = collect_taxes::decline_to_collect(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Improve Lifestyle")]
fn when_i_improve_lifestyle(world: &mut TestContext) {
    let triple = improve_lifestyle::improve_lifestyle(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Trade Commodities of {int} Lumber")]
fn when_i_trade_commodities(world: &mut TestContext, volume: i32) {
    let triple = trade_commodities::trade_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Lumber,
        volume as i8,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Take Charge with Arts")]
fn when_i_take_charge_arts(world: &mut TestContext) {
    let triple = take_charge::take_charge(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Skill::Arts,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Purchase Commodities of Food (secondary Lumber)")]
fn when_i_purchase_commodities(world: &mut TestContext) {
    let triple = purchase_commodities::purchase_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Food,
        Commodity::Lumber,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Purchase Commodities of Luxuries (secondary Lumber)")]
fn when_i_purchase_commodities2(world: &mut TestContext) {
    let triple = purchase_commodities::purchase_commodities(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        Commodity::Luxuries,
        Commodity::Lumber,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Celebrate Holiday")]
fn when_i_celebrate_holiday(world: &mut TestContext) {
    let triple = celebrate_holiday::celebrate_holiday(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I search for a Supernatural Solution")]
fn when_i_search_for_a_supernatural_solution(world: &mut TestContext) {
    let triple = supernatural_solution::supernatural_solution(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Prognosticate")]
fn when_i_prognosticate(world: &mut TestContext) {
    let triple = prognostication::prognosticate(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Create a Masterpiece")]
fn when_i_create_a_masterpiece(world: &mut TestContext) {
    let triple = create_a_masterpiece::create_a_masterpiece(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Go Fishing")]
fn when_i_go_fishing(world: &mut TestContext) {
    let triple = go_fishing::go_fishing(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Establish Farmland on plains")]
fn when_i_establish_farmland_on_plains(world: &mut TestContext) {
    let triple = establish_farmland::establish_farmland(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        establish_farmland::HexType::Plains,
        0i8,
        0i8,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Establish Farmland on hills")]
fn when_i_establish_farmland_on_hills(world: &mut TestContext) {
    let triple = establish_farmland::establish_farmland(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        establish_farmland::HexType::Hills,
        0i8,
        0i8,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Claim Hex with Magic")]
fn when_i_claim_hex_with_magic(world: &mut TestContext) {
    let triple = claim_hex::claim_hex(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        ClaimHexSkill::Magic,
        0,
        0,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Build an Alchemy Lab Structure")]
fn when_i_build_an_alchemy_lab(world: &mut TestContext) {
    let triple = build_structure::build_structure(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        build_structure::Structure::AlchemyLab,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when("I Build a Bridge Structure")]
fn when_i_build_a_bridge(world: &mut TestContext) {
    let triple = build_structure::build_structure(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        build_structure::Structure::Bridge,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

// FIXME: Figure out why this doesn't trigger and get rid of the
//        terrain-specific functions below. (This one is not
//        currently executed.)
#[when(expr = "I Build Roads on {string}")]
fn when_i_build_roads(world: &mut TestContext, terrain: String) {

    let terrain = terrain::TerrainType::from_string(&terrain).unwrap();

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Build Roads on plains")]
fn when_i_build_roads_plains(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Plains;

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Build Roads on mountains")]
fn when_i_build_roads_mountains(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Mountains;

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Build Roads on forest")]
fn when_i_build_roads_forest(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Forest;

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Build Roads on hills")]
fn when_i_build_roads_hills(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Hills;

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Build Roads on swamp")]
fn when_i_build_roads_swamp(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Swamp;

    let triple = build_roads::build_roads(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Establish Work Site on plains")]
fn when_i_establish_work_site_plains(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Plains;

    let triple = establish_work_site::establish_work_site(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}

#[when(expr = "I Establish Work Site on hills")]
fn when_i_establish_work_site_hills(world: &mut TestContext) {

    let terrain = terrain::TerrainType::Hills;

    let triple = establish_work_site::establish_work_site(
        &world.kingdom.as_ref().unwrap(),
        &world.turn_state,
        &world.kingdom_state,
        &world.roll_context.as_ref().unwrap(),
        terrain,
    );
    (_, world.next_turn_state, world.kingdom_state) = triple;
    world.roll_result = Some(triple.0);
}
