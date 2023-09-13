use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn celebrate_holiday(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn.clone();
    next_turn_state.additional_fame_points_scheduled += 1;

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.fame_points += 1;
    next_kingdom_state.resource_points += context.d4 + context.d4;

    (next_turn_state, next_kingdom_state)
}
