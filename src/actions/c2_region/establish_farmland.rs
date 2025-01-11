use poise::ChoiceParameter;

use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult, DC}}, spec::{skills::Skill, Kingdom}, state::{Commodity, KingdomState}, turns::TurnState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ChoiceParameter)]
pub enum HexType {
    Plains,
    Hills,
}

// TODO: return 'static str
pub fn claim_second_hex_message(hex_type: HexType) -> String {
    match hex_type {
        HexType::Plains => "may immediately attempt Establish Farmland again (plains only)",
        HexType::Hills  => "may immediately attempt Establish Farmland again (plains or hills)",
    }.to_string()
}

pub fn establish_farmland(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, hex_type: HexType, x: i8, y: i8) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::EstablishFarmland, Skill::Agriculture, context);

    // TODO: Cucumber tests need enhanced for DC change and RP cost (previously it was zero cost)
    let control_dc = state.control_dc(kingdom);
    let dc = match hex_type {
        HexType::Plains => control_dc,
        HexType::Hills  => DC(control_dc.0 + 5),
    };
    let rp_cost = match hex_type {
        HexType::Plains => 1,
        HexType::Hills  => 2,
    };

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let new_msg = format!("mark hex {x}.{y} with the new farmland");
    let new_requirements = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![new_msg, claim_second_hex_message(hex_type)],
        DegreeOfSuccess::Success         => vec![new_msg],
        _                                => vec![],
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree.passed() {
        next_turn_state.commodity_income[Commodity::Food] += 1;
    }
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.dc6_crop_failure_potential_for_x_turns = 2;

    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}
