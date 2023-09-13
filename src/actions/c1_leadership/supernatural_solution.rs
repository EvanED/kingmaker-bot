use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn supernatural_solution(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn.clone();
    next_turn_state.supernatural_solution_available = true;

    let next_kingdom_state = state.clone();

    (next_turn_state, next_kingdom_state)
}
