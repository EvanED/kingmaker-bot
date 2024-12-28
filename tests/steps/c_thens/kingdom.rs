use cucumber::then;
use assert2::assert;
use kingdom::{actions::c3_civic::build_structure::Structure, spec::skills::Skill, state::Commodity, turns::RandomEventSelectionMethod};
use crate::context::TestContext;

#[then(expr = "Unrest is still {int}")]
#[then(expr = "Unrest went up to {int}")]
#[then(expr = "Unrest went down to {int}")]
fn then_unrest_is_n(world: &mut TestContext, unrest: i32) {
    assert!(world.kingdom_state.unrest as i32 == unrest);
}

#[then(expr = "RP is still {int}")]
#[then(expr = "RP went up to {int}")]
#[then(expr = "RP went down to {int}")]
fn then_rp_is_n(world: &mut TestContext, rp: i32) {
    assert!(world.kingdom_state.resource_points as i32 == rp);
}

#[then(expr = "Fame points is still {int}")]
#[then(expr = "Fame points went up to {int}")]
#[then(expr = "Fame points went down to {int}")]
fn then_fame_points_is_n(world: &mut TestContext, fame: i32) {
    assert!(world.kingdom_state.fame_points as i32 == fame);
}

#[then("the kingdom will not gain one additional Fame point next turn")]
fn then_the_kingdom_will_not_gain_one_additional_fame_point_next_turn(world: &mut TestContext) {
    assert!(world.next_turn_state.additional_fame_points_scheduled == 0);
}

#[then("we are required to increase any Ruin")]
fn then_we_are_required_to_increase_any_ruin(world: &mut TestContext) {
    assert!(world.next_turn_state.requirements == vec!["increase any Ruin"]);
}

#[then(expr = "next turn will have {int} bonus RP")]
fn given_next_turn_will_have_x_bonus_rp(world: &mut TestContext, volume: i32) {
    assert!(volume == world.next_turn_state.bonus_rp as i32);
}

#[then(expr = "I have {int} Lumber")]
#[then(expr = "the kingdom's Lumber is still {int}")]
#[then(expr = "the kingdom's Lumber went up to {int}")]
#[then(expr = "the kingdom's Lumber went down to {int}")]
fn given_next_turn_will_have_x_lumber(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Lumber] as i32);
}

#[then(expr = "I have {int} Food")]
#[then(expr = "the kingdom's Food went up to {int}")]
#[then(expr = "the kingdom's Food is still {int}")]
fn given_next_turn_will_have_x_food(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Food] as i32);
}

#[then(expr = "the kingdom's Ore went down to {int}")]
#[then(expr = "the kingdom's Ore went up to {int}")]
#[then(expr = "the kingdom's Ore is still {int}")]
fn given_next_turn_will_have_x_ore(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Ore] as i32);
}

#[then(expr = "the kingdom's Stone went down to {int}")]
#[then(expr = "the kingdom's Stone went up to {int}")]
#[then(expr = "the kingdom's Stone is still {int}")]
fn given_next_turn_will_have_x_stone(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Stone] as i32);
}

#[then(expr = "I have {int} Luxuries")]
fn given_next_turn_will_have_x_luxuries(world: &mut TestContext, volume: i32) {
    assert!(volume == world.kingdom_state.commodity_stores[Commodity::Luxuries] as i32);
}

#[then("the players have advantage on selection of a random kingdom event this turn")]
fn then_the_players_have_advantage_on_random_event_selection(world: &mut TestContext) {
    assert!(world.next_turn_state.random_event_selection_method == Some(RandomEventSelectionMethod::AdvantagePlayers));
}

#[then("random kingdom event selection is normal")]
fn then_kingdom_selection_is_normal(world: &mut TestContext) {
    assert!(world.next_turn_state.random_event_selection_method == None);
}

#[then("the GM has advantage on selection of a random kingdom event this turn")]
fn then_the_gm_has_advantage_on_random_event_selection(world: &mut TestContext) {
    assert!(world.next_turn_state.random_event_selection_method == Some(RandomEventSelectionMethod::AdvantageGM));
}

#[then(expr = "there are {int} requirements")]
#[then(expr = "there is {int} requirement")]
fn then_there_are_x_requirements(world: &mut TestContext, expected_count: i32) {
    assert!(world.next_turn_state.requirements.len() == expected_count as usize);
}

#[then(expr = "there are no requirements")]
fn then_there_are_no_requirements(world: &mut TestContext) {
    assert!(Vec::<String>::new() == world.next_turn_state.requirements);
}

#[then(expr = "{string} is a requirement")]
fn then_x_is_a_requirements(world: &mut TestContext, requirement: String) {
    assert!(world.next_turn_state.requirements.contains(&requirement));
}

#[then(expr = "there is a Crop Failure potential for next {int} turns")]
fn then_there_is_a_crop_failure_potential_for_next_x_turns(world: &mut TestContext, nturns: i32) {
    assert!(nturns == world.next_turn_state.dc6_crop_failure_potential_for_x_turns as i32);
}

#[then("next turn can re-attempt building a Shrine at no resource cost")]
fn then_next_turn_can_reattempt_building_a_shrine_at_no_resource_cost(world: &mut TestContext) {
    assert!(Some(Structure::Shrine) == world.next_turn_state.can_build_this_structure_for_no_resource_cost);
}

#[then("next turn can re-attempt building an Alchemy Lab at no resource cost")]
fn then_next_turn_can_reattempt_building_an_alchemy_lab_at_no_resource_cost(world: &mut TestContext) {
    assert!(Some(Structure::AlchemyLab) == world.next_turn_state.can_build_this_structure_for_no_resource_cost);
}

#[then("next turn can not re-attempt building anything at no resource cost")]
fn then_next_turn_can_not_reattempt_building_a_shrine_at_no_resource_cost(world: &mut TestContext) {
    assert!(None == world.next_turn_state.can_build_this_structure_for_no_resource_cost);
}

#[then("Take Charge (Arts) has been used this turn")]
fn then_take_charge_arts_has_not_been_used_this_turn(world: &mut TestContext) {
    assert!(world.next_turn_state.take_charge_skills_used[Skill::Arts]);
}

#[then(expr="XP went up to {int}")]
#[then(expr="XP is still {int}")]
fn given_xp_changed_to(world: &mut TestContext, xp: i32) {
    assert!(world.kingdom_state.xp as i32 == xp);
}
