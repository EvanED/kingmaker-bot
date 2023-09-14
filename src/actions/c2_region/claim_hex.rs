use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

// TODO: Require Skill as one of prereqs
pub fn claim_hex(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, skill: Skill) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );


    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.push("mark the hex as claimed".to_string());
    next_turn_state.requirements.push("you may take another region activity".to_string());

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= 1;

    (next_turn_state, next_kingdom_state)
}
