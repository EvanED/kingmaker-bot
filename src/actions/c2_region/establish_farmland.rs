use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn establish_farmland(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, hex_type: HexType) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Agriculture, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let new_msg = "mark the map with the new farmland".to_string();
    let new_requirements = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![new_msg, claim_second_hex_message(hex_type)],
        DegreeOfSuccess::Success         => vec![new_msg],
        _                                => vec![],
    };


    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.dc6_crop_failure_potential_for_x_turns = 2;
    }

    let next_kingdom_state = state.clone();

    (next_turn_state, next_kingdom_state)
}